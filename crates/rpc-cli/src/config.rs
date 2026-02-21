use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Deserialize;

pub const CONFIG_FILE_NAME: &str = "rpc.config.toml";

#[derive(Debug, Default, Deserialize)]
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
    pub svelte: Option<PathBuf>,
    pub react: Option<PathBuf>,
    pub vue: Option<PathBuf>,
    pub solid: Option<PathBuf>,
    pub imports: ImportsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ImportsConfig {
    pub types_path: String,
    pub extension: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Deserialize, clap::ValueEnum)]
pub enum FieldNaming {
    #[default]
    #[serde(rename = "preserve")]
    #[value(name = "preserve")]
    Preserve,
    #[serde(rename = "camelCase")]
    #[value(name = "camelCase")]
    CamelCase,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct NamingConfig {
    pub fields: FieldNaming,
}

#[derive(Debug, Default, Deserialize)]
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
            svelte: None,
            react: None,
            vue: None,
            solid: None,
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
#[derive(Default)]
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
    pub svelte_output: Option<PathBuf>,
    pub react_output: Option<PathBuf>,
    pub vue_output: Option<PathBuf>,
    pub solid_output: Option<PathBuf>,
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
pub fn resolve(cli: CliOverrides) -> Result<RpcConfig> {
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

    // Apply CLI overrides (move values instead of cloning)
    if let Some(dir) = cli.dir {
        config.input.dir = dir;
    }
    if !cli.include.is_empty() {
        config.input.include = cli.include;
    }
    if !cli.exclude.is_empty() {
        config.input.exclude = cli.exclude;
    }
    if let Some(output) = cli.output {
        config.output.types = output;
    }
    if let Some(client_output) = cli.client_output {
        config.output.client = client_output;
    }
    if let Some(svelte_output) = cli.svelte_output {
        config.output.svelte = Some(svelte_output);
    }
    if let Some(react_output) = cli.react_output {
        config.output.react = Some(react_output);
    }
    if let Some(vue_output) = cli.vue_output {
        config.output.vue = Some(vue_output);
    }
    if let Some(solid_output) = cli.solid_output {
        config.output.solid = Some(solid_output);
    }
    if let Some(types_import) = cli.types_import {
        config.output.imports.types_path = types_import;
    }
    if let Some(extension) = cli.extension {
        config.output.imports.extension = extension;
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
