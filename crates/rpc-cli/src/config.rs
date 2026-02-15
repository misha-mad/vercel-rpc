use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "rpc.config.toml";

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RpcConfig {
    pub input: InputConfig,
    pub output: OutputConfig,
    pub watch: WatchConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct InputConfig {
    pub dir: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct OutputConfig {
    pub types: PathBuf,
    pub client: PathBuf,
    pub imports: ImportsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ImportsConfig {
    pub types_path: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct WatchConfig {
    pub debounce_ms: u64,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            input: InputConfig::default(),
            output: OutputConfig::default(),
            watch: WatchConfig::default(),
        }
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("api"),
        }
    }
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            types: PathBuf::from("src/lib/rpc-types.ts"),
            client: PathBuf::from("src/lib/rpc-client.ts"),
            imports: ImportsConfig::default(),
        }
    }
}

impl Default for ImportsConfig {
    fn default() -> Self {
        Self {
            types_path: "./rpc-types".to_string(),
        }
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self { debounce_ms: 200 }
    }
}

/// Walk up from `start` looking for `rpc.config.toml`.
/// Returns `None` if not found.
pub fn discover(start: &Path) -> Option<PathBuf> {
    let mut dir = start;
    loop {
        let candidate = dir.join(CONFIG_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate);
        }
        dir = dir.parent()?;
    }
}

/// Read and parse a config file at the given path.
pub fn load(path: &Path) -> Result<RpcConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file {}", path.display()))?;
    let config: RpcConfig =
        toml::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;
    Ok(config)
}

/// CLI overrides that can be applied on top of a loaded config.
pub struct CliOverrides {
    pub config: Option<PathBuf>,
    pub no_config: bool,
    pub dir: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub client_output: Option<PathBuf>,
    pub types_import: Option<String>,
}

/// Resolve config: discover/load the file, then apply CLI overrides.
pub fn resolve(cli: &CliOverrides) -> Result<RpcConfig> {
    let mut config = if cli.no_config {
        RpcConfig::default()
    } else if let Some(path) = &cli.config {
        load(path)?
    } else {
        let cwd = std::env::current_dir().context("Failed to get current directory")?;
        match discover(&cwd) {
            Some(path) => load(&path)?,
            None => RpcConfig::default(),
        }
    };

    // Apply CLI overrides
    if let Some(dir) = &cli.dir {
        config.input.dir = dir.clone();
    }
    if let Some(output) = &cli.output {
        config.output.types = output.clone();
    }
    if let Some(client_output) = &cli.client_output {
        config.output.client = client_output.clone();
    }
    if let Some(types_import) = &cli.types_import {
        config.output.imports.types_path = types_import.clone();
    }

    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_default_matches_current_behavior() {
        let config = RpcConfig::default();
        assert_eq!(config.input.dir, PathBuf::from("api"));
        assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
        assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
        assert_eq!(config.output.imports.types_path, "./rpc-types");
        assert_eq!(config.watch.debounce_ms, 200);
    }

    #[test]
    fn test_parse_minimal() {
        let toml_str = r#"
[input]
dir = "src/api"
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.input.dir, PathBuf::from("src/api"));
        // Rest should be defaults
        assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
        assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
        assert_eq!(config.output.imports.types_path, "./rpc-types");
        assert_eq!(config.watch.debounce_ms, 200);
    }

    #[test]
    fn test_parse_full() {
        let toml_str = r#"
[input]
dir = "lambdas"

[output]
types = "types.ts"
client = "client.ts"

[output.imports]
types_path = "./types"

[watch]
debounce_ms = 500
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.input.dir, PathBuf::from("lambdas"));
        assert_eq!(config.output.types, PathBuf::from("types.ts"));
        assert_eq!(config.output.client, PathBuf::from("client.ts"));
        assert_eq!(config.output.imports.types_path, "./types");
        assert_eq!(config.watch.debounce_ms, 500);
    }

    #[test]
    fn test_parse_empty() {
        let config: RpcConfig = toml::from_str("").unwrap();
        assert_eq!(config.input.dir, PathBuf::from("api"));
        assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
        assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
        assert_eq!(config.output.imports.types_path, "./rpc-types");
        assert_eq!(config.watch.debounce_ms, 200);
    }

    #[test]
    fn test_parse_invalid_toml() {
        let result: Result<RpcConfig, _> = toml::from_str("not = [valid toml");
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_fields_ignored() {
        let toml_str = r#"
[input]
dir = "api"
some_future_field = true

[future_section]
key = "value"
"#;
        // serde(default) doesn't deny unknown fields by default, so this should work
        let result: Result<RpcConfig, _> = toml::from_str(toml_str);
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.input.dir, PathBuf::from("api"));
    }

    #[test]
    fn test_discovery_found() {
        let tmp = TempDir::new().unwrap();
        let config_path = tmp.path().join(CONFIG_FILE_NAME);
        std::fs::write(&config_path, "[input]\ndir = \"api\"\n").unwrap();

        let found = discover(tmp.path());
        assert_eq!(found, Some(config_path));
    }

    #[test]
    fn test_discovery_parent() {
        let tmp = TempDir::new().unwrap();
        let config_path = tmp.path().join(CONFIG_FILE_NAME);
        std::fs::write(&config_path, "[input]\ndir = \"api\"\n").unwrap();

        let child = tmp.path().join("sub").join("dir");
        std::fs::create_dir_all(&child).unwrap();

        let found = discover(&child);
        assert_eq!(found, Some(config_path));
    }

    #[test]
    fn test_discovery_missing() {
        let tmp = TempDir::new().unwrap();
        let found = discover(tmp.path());
        assert!(found.is_none());
    }

    #[test]
    fn test_cli_overrides() {
        let tmp = TempDir::new().unwrap();
        let config_path = tmp.path().join(CONFIG_FILE_NAME);
        std::fs::write(
            &config_path,
            r#"
[input]
dir = "lambdas"

[output]
types = "types.ts"
client = "client.ts"
"#,
        )
        .unwrap();

        let config = load(&config_path).unwrap();
        assert_eq!(config.input.dir, PathBuf::from("lambdas"));

        // Now test that CLI overrides work
        let mut config = load(&config_path).unwrap();
        let overrides = CliOverrides {
            config: None,
            no_config: false,
            dir: Some(PathBuf::from("other")),
            output: Some(PathBuf::from("out.ts")),
            client_output: None,
            types_import: Some("./my-types".to_string()),
        };

        if let Some(dir) = &overrides.dir {
            config.input.dir = dir.clone();
        }
        if let Some(output) = &overrides.output {
            config.output.types = output.clone();
        }
        if let Some(client_output) = &overrides.client_output {
            config.output.client = client_output.clone();
        }
        if let Some(types_import) = &overrides.types_import {
            config.output.imports.types_path = types_import.clone();
        }

        assert_eq!(config.input.dir, PathBuf::from("other"));
        assert_eq!(config.output.types, PathBuf::from("out.ts"));
        assert_eq!(config.output.client, PathBuf::from("client.ts")); // not overridden
        assert_eq!(config.output.imports.types_path, "./my-types");
    }
}
