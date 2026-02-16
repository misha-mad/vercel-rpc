use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

const CONFIG_FILE_NAME: &str = "rpc.config.toml";

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct RpcConfig {
    pub input: InputConfig,
    pub output: OutputConfig,
    pub codegen: CodegenConfig,
    pub watch: WatchConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct InputConfig {
    pub dir: PathBuf,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
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
    pub extension: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, clap::ValueEnum)]
pub enum FieldNaming {
    #[serde(rename = "preserve")]
    #[value(name = "preserve")]
    Preserve,
    #[serde(rename = "camelCase")]
    #[value(name = "camelCase")]
    CamelCase,
}

impl Default for FieldNaming {
    fn default() -> Self {
        Self::Preserve
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct NamingConfig {
    pub fields: FieldNaming,
}

impl Default for NamingConfig {
    fn default() -> Self {
        Self {
            fields: FieldNaming::default(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CodegenConfig {
    pub preserve_docs: bool,
    pub naming: NamingConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct WatchConfig {
    pub debounce_ms: u64,
    pub clear_screen: bool,
}

impl Default for RpcConfig {
    fn default() -> Self {
        Self {
            input: InputConfig::default(),
            output: OutputConfig::default(),
            codegen: CodegenConfig::default(),
            watch: WatchConfig::default(),
        }
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            dir: PathBuf::from("api"),
            include: vec!["**/*.rs".into()],
            exclude: vec![],
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
            extension: String::new(),
        }
    }
}

impl ImportsConfig {
    /// Returns the full import specifier: `types_path` + `extension`.
    pub fn types_specifier(&self) -> String {
        format!("{}{}", self.types_path, self.extension)
    }
}

impl Default for CodegenConfig {
    fn default() -> Self {
        Self {
            preserve_docs: false,
            naming: NamingConfig::default(),
        }
    }
}

impl Default for WatchConfig {
    fn default() -> Self {
        Self {
            debounce_ms: 200,
            clear_screen: false,
        }
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
    // input
    pub dir: Option<PathBuf>,
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    // output
    pub output: Option<PathBuf>,
    pub client_output: Option<PathBuf>,
    pub types_import: Option<String>,
    pub extension: Option<String>,
    // codegen
    pub preserve_docs: bool,
    pub fields: Option<FieldNaming>,
    // watch
    pub debounce_ms: Option<u64>,
    pub clear_screen: bool,
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
    if !cli.include.is_empty() {
        config.input.include = cli.include.clone();
    }
    if !cli.exclude.is_empty() {
        config.input.exclude = cli.exclude.clone();
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
    if let Some(extension) = &cli.extension {
        config.output.imports.extension = extension.clone();
    }
    if cli.preserve_docs {
        config.codegen.preserve_docs = true;
    }
    if let Some(fields) = cli.fields {
        config.codegen.naming.fields = fields;
    }
    if let Some(debounce_ms) = cli.debounce_ms {
        config.watch.debounce_ms = debounce_ms;
    }
    if cli.clear_screen {
        config.watch.clear_screen = true;
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
        assert_eq!(config.input.include, vec!["**/*.rs".to_string()]);
        assert!(config.input.exclude.is_empty());
        assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
        assert_eq!(config.output.client, PathBuf::from("src/lib/rpc-client.ts"));
        assert_eq!(config.output.imports.types_path, "./rpc-types");
        assert_eq!(config.output.imports.extension, "");
        assert_eq!(config.output.imports.types_specifier(), "./rpc-types");
        assert!(!config.codegen.preserve_docs);
        assert_eq!(config.codegen.naming.fields, FieldNaming::Preserve);
        assert_eq!(config.watch.debounce_ms, 200);
        assert!(!config.watch.clear_screen);
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
include = ["src/**/*.rs"]
exclude = ["src/tests/**"]

[output]
types = "types.ts"
client = "client.ts"

[output.imports]
types_path = "./types"
extension = ".js"

[codegen]
preserve_docs = true

[codegen.naming]
fields = "camelCase"

[watch]
debounce_ms = 500
clear_screen = true
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.input.dir, PathBuf::from("lambdas"));
        assert_eq!(config.input.include, vec!["src/**/*.rs".to_string()]);
        assert_eq!(config.input.exclude, vec!["src/tests/**".to_string()]);
        assert_eq!(config.output.types, PathBuf::from("types.ts"));
        assert_eq!(config.output.client, PathBuf::from("client.ts"));
        assert_eq!(config.output.imports.types_path, "./types");
        assert_eq!(config.output.imports.extension, ".js");
        assert_eq!(config.output.imports.types_specifier(), "./types.js");
        assert!(config.codegen.preserve_docs);
        assert_eq!(config.codegen.naming.fields, FieldNaming::CamelCase);
        assert_eq!(config.watch.debounce_ms, 500);
        assert!(config.watch.clear_screen);
    }

    #[test]
    fn test_parse_include_exclude() {
        let toml_str = r#"
[input]
include = ["handlers/**/*.rs", "api/**/*.rs"]
exclude = ["**/test_*.rs"]
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.input.dir, PathBuf::from("api")); // default
        assert_eq!(
            config.input.include,
            vec!["handlers/**/*.rs".to_string(), "api/**/*.rs".to_string()]
        );
        assert_eq!(config.input.exclude, vec!["**/test_*.rs".to_string()]);
    }

    #[test]
    fn test_parse_codegen_preserve_docs() {
        let toml_str = r#"
[codegen]
preserve_docs = true
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert!(config.codegen.preserve_docs);
        // Other fields should be defaults
        assert_eq!(config.input.dir, PathBuf::from("api"));
        assert_eq!(config.output.types, PathBuf::from("src/lib/rpc-types.ts"));
    }

    #[test]
    fn test_parse_codegen_naming_fields() {
        let toml_str = r#"
[codegen.naming]
fields = "camelCase"
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.codegen.naming.fields, FieldNaming::CamelCase);
        // Other fields should be defaults
        assert!(!config.codegen.preserve_docs);
        assert_eq!(config.input.dir, PathBuf::from("api"));

        // Test preserve value
        let toml_str2 = r#"
[codegen.naming]
fields = "preserve"
"#;
        let config2: RpcConfig = toml::from_str(toml_str2).unwrap();
        assert_eq!(config2.codegen.naming.fields, FieldNaming::Preserve);

        // Test default (no naming section)
        let toml_str3 = r#"
[codegen]
preserve_docs = true
"#;
        let config3: RpcConfig = toml::from_str(toml_str3).unwrap();
        assert_eq!(config3.codegen.naming.fields, FieldNaming::Preserve);
    }

    #[test]
    fn test_parse_extension() {
        let toml_str = r#"
[output.imports]
extension = ".js"
"#;
        let config: RpcConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.output.imports.types_path, "./rpc-types"); // default
        assert_eq!(config.output.imports.extension, ".js");
        assert_eq!(config.output.imports.types_specifier(), "./rpc-types.js");
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
        let overrides = CliOverrides {
            config: Some(config_path),
            no_config: false,
            dir: Some(PathBuf::from("other")),
            include: vec!["handlers/**/*.rs".into()],
            exclude: vec!["**/test_*.rs".into()],
            output: Some(PathBuf::from("out.ts")),
            client_output: None,
            types_import: Some("./my-types".to_string()),
            extension: Some(".js".to_string()),
            preserve_docs: true,
            fields: Some(FieldNaming::CamelCase),
            debounce_ms: Some(500),
            clear_screen: true,
        };

        let config = resolve(&overrides).unwrap();

        assert_eq!(config.input.dir, PathBuf::from("other"));
        assert_eq!(config.input.include, vec!["handlers/**/*.rs".to_string()]);
        assert_eq!(config.input.exclude, vec!["**/test_*.rs".to_string()]);
        assert_eq!(config.output.types, PathBuf::from("out.ts"));
        assert_eq!(config.output.client, PathBuf::from("client.ts")); // not overridden
        assert_eq!(config.output.imports.types_path, "./my-types");
        assert_eq!(config.output.imports.extension, ".js");
        assert!(config.codegen.preserve_docs);
        assert_eq!(config.codegen.naming.fields, FieldNaming::CamelCase);
        assert_eq!(config.watch.debounce_ms, 500);
        assert!(config.watch.clear_screen);
    }
}
