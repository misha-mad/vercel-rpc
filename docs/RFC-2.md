# RFC-2: Configuration File (`rpc.config.toml`)

- **Status:** Implemented
- **Topic:** Project-level configuration file for rpc-cli
- **Date:** February 2026

## 1. Summary

Replace the CLI-flags-only configuration model with an optional `rpc.config.toml` file. All current CLI flags become config fields. CLI flags remain as overrides. When no config file is found, the current defaults apply — full backward compatibility.

## 2. Motivation

Today every option must be passed as a CLI flag:

```bash
rpc generate --dir api --output src/lib/rpc-types.ts --client-output src/lib/rpc-client.ts --types-import ./rpc-types
```

This has several problems:

1. **Repetition** — `generate` and `watch` share the exact same four flags. Users must duplicate them in `package.json` scripts or shell aliases.
2. **Not composable** — adding new options (naming conventions, import extensions, codegen style) means adding more flags, making the command line unwieldy.
3. **No project-level defaults** — contributors must know the right flags or read `package.json`. A config file is self-documenting and version-controlled.
4. **Blocked features** — several planned features (field naming conventions, ESM import extensions, doc-comment preservation) need configuration that doesn't fit well as CLI flags.

## 3. Config File Format

TOML is chosen for consistency with the Rust ecosystem (`Cargo.toml`, `rustfmt.toml`). The file is named `rpc.config.toml` and placed at the project root (next to `Cargo.toml` or `package.json`).

### Full Example

```toml
# rpc.config.toml — all fields are optional, defaults shown

[input]
dir = "api"                          # Rust source directory to scan
include = ["**/*.rs"]                # glob patterns for file inclusion
exclude = []                         # glob patterns for file exclusion

[output]
types = "src/lib/rpc-types.ts"       # generated types file path
client = "src/lib/rpc-client.ts"     # generated client file path

[output.imports]
types_path = "./rpc-types"           # import specifier used in client file
extension = ""                       # appended to import path (e.g. ".js" for ESM)

[codegen]
preserve_docs = false                # forward Rust /// comments as JSDoc

[codegen.naming]
fields = "preserve"                  # "preserve" | "camelCase"

[watch]
debounce_ms = 200                    # file watcher debounce interval
clear_screen = false                 # clear terminal before regeneration
```

### Minimal Example

```toml
# Typical SvelteKit project — only override what differs from defaults

[output]
types = "src/lib/generated/rpc-types.ts"
client = "src/lib/generated/rpc-client.ts"

[output.imports]
extension = ".js"

[codegen.naming]
fields = "camelCase"
```

### Empty or Missing File

When `rpc.config.toml` is absent or empty, behavior is identical to the current CLI defaults. No config file is ever required.

## 4. Resolution Order

Values are resolved with the following priority (highest first):

```
CLI flag  >  rpc.config.toml  >  built-in default
```

This means:
- A `rpc.config.toml` sets project-level defaults.
- A CLI flag overrides any config file value for that invocation.
- Built-in defaults fill in anything not specified by either.

### Example

```toml
# rpc.config.toml
[input]
dir = "lambdas"
```

```bash
# Uses dir = "lambdas" from config
rpc generate

# Overrides to dir = "api" for this run only
rpc generate --dir api
```

## 5. Config Discovery

The CLI searches for `rpc.config.toml` using the following strategy:

1. Start from the current working directory.
2. Walk up parent directories until a `rpc.config.toml` is found.
3. Stop at the filesystem root.
4. If no file is found, use built-in defaults.

A `--config <path>` flag allows explicit override, skipping discovery:

```bash
rpc generate --config ./custom-config.toml
```

A `--no-config` flag disables config file loading entirely:

```bash
rpc generate --no-config --dir api
```

## 6. Field Reference

### `[input]`

| Field     | Type       | Default       | Description                                     |
|-----------|------------|---------------|-------------------------------------------------|
| `dir`     | `string`   | `"api"`       | Directory containing Rust lambda source files   |
| `include` | `string[]` | `["**/*.rs"]` | Glob patterns — only matching files are scanned |
| `exclude` | `string[]` | `[]`          | Glob patterns — matching files are skipped      |

When both `include` and `exclude` match a file, `exclude` wins.

### `[output]`

| Field    | Type     | Default                    | Description                       |
|----------|----------|----------------------------|-----------------------------------|
| `types`  | `string` | `"src/lib/rpc-types.ts"`   | Output path for types file        |
| `client` | `string` | `"src/lib/rpc-client.ts"`  | Output path for client file       |

### `[output.imports]`

| Field        | Type     | Default         | Description                                             |
|--------------|----------|-----------------|---------------------------------------------------------|
| `types_path` | `string` | `"./rpc-types"` | Import specifier for types file used inside client file |
| `extension`  | `string` | `""`            | Suffix appended to import path (e.g. `".js"` for ESM)   |

Generated import in `rpc-client.ts`:

```typescript
// types_path = "./rpc-types", extension = ""
import type { Procedures, ... } from "./rpc-types";

// types_path = "./rpc-types", extension = ".js"
import type { Procedures, ... } from "./rpc-types.js";
```

### `[codegen]`

| Field           | Type   | Default | Description                                    |
|-----------------|--------|---------|------------------------------------------------|
| `preserve_docs` | `bool` | `false` | Generate JSDoc from Rust `///` doc-comments    |

### `[codegen.naming]`

| Field    | Type     | Default      | Description                                  |
|----------|----------|--------------|----------------------------------------------|
| `fields` | `string` | `"preserve"` | Field naming: `"preserve"` or `"camelCase"`  |

`"preserve"` keeps field names as-is from Rust (typically `snake_case`). `"camelCase"` converts all struct fields to camelCase in the generated TypeScript. This setting is overridden by explicit `#[serde(rename)]` or `#[serde(rename_all)]` attributes on individual types.

### `[watch]`

| Field          | Type   | Default | Description                                 |
|----------------|--------|---------|---------------------------------------------|
| `debounce_ms`  | `u64`  | `200`   | Debounce interval for file watcher          |
| `clear_screen` | `bool` | `false` | Clear terminal before each regeneration     |

## 7. Implementation Plan

### 7.1 New Dependency

Add the `toml` crate to `rpc-cli`:

```toml
# crates/rpc-cli/Cargo.toml
toml = "0.8"
glob = "0.3"        # for include/exclude patterns
```

### 7.2 Config Struct

A `config` module in `rpc-cli` (implemented as a single file):

```
crates/rpc-cli/src/
└── config.rs       # RpcConfig struct, TOML parsing, discovery, CLI merge
```

Core types:

```rust
// config/mod.rs

use std::path::PathBuf;
use serde::Deserialize;

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
    pub imports: ImportsConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ImportsConfig {
    pub types_path: String,
    pub extension: String,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CodegenConfig {
    pub preserve_docs: bool,
    pub naming: NamingConfig,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct NamingConfig {
    pub fields: FieldNaming,
}

#[derive(Debug, Default, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum FieldNaming {
    #[default]
    Preserve,
    CamelCase,
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct WatchConfig {
    pub debounce_ms: u64,
    pub clear_screen: bool,
}
```

Each nested struct implements `Default` with the values listed in the field reference above.

### 7.3 Config Discovery

```rust
// config/file.rs

use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

const CONFIG_FILENAME: &str = "rpc.config.toml";

/// Walk up from `start` looking for rpc.config.toml.
/// Returns None if no config file is found.
pub fn discover(start: &Path) -> Option<PathBuf> {
    let mut dir = start.canonicalize().ok()?;
    loop {
        let candidate = dir.join(CONFIG_FILENAME);
        if candidate.is_file() {
            return Some(candidate);
        }
        if !dir.pop() {
            return None;
        }
    }
}

/// Parse a TOML config file into RpcConfig.
pub fn parse(path: &Path) -> Result<super::RpcConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path.display()))?;
    toml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", path.display()))
}
```

### 7.4 Merge Logic

```rust
// config/merge.rs

/// Merge CLI flags into a loaded config.
/// CLI flags use Option<T> — None means "not provided", Some means "override".
pub fn merge(file_config: RpcConfig, cli: &CliOverrides) -> RpcConfig {
    // For each field: cli.field.unwrap_or(file_config.field)
}
```

CLI argument structs change from direct values to `Option<T>`:

```rust
#[derive(Subcommand)]
enum Command {
    Generate {
        #[arg(short, long)]
        dir: Option<PathBuf>,       // was: PathBuf with default_value

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(short, long)]
        client_output: Option<PathBuf>,

        #[arg(long)]
        types_import: Option<String>,

        #[arg(long)]
        config: Option<PathBuf>,    // new: explicit config path

        #[arg(long)]
        no_config: bool,            // new: disable config loading
    },
    // ...
}
```

### 7.5 Integration into `main.rs`

The new flow in `main()`:

```rust
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Generate { dir, output, client_output, types_import, config, no_config } => {
            // 1. Load config file (unless --no-config)
            let file_config = if no_config {
                RpcConfig::default()
            } else {
                let config_path = config
                    .or_else(|| config::file::discover(&std::env::current_dir()?));
                match config_path {
                    Some(path) => config::file::parse(&path)?,
                    None => RpcConfig::default(),
                }
            };

            // 2. Merge CLI overrides
            let resolved = config::merge(file_config, &CliOverrides {
                dir, output, client_output, types_import,
            });

            // 3. Run generation with resolved config
            cmd_generate(&resolved)
        }
        // ...
    }
}
```

### 7.6 Include/Exclude Patterns

The `include` and `exclude` fields use standard glob patterns (via the `glob` crate). They are applied during the directory scan in `parser::scan_directory`:

```rust
// parser/extract.rs — modified scan loop

fn should_include(path: &Path, config: &InputConfig) -> bool {
    let rel = path.strip_prefix(&config.dir).unwrap_or(path);
    let rel_str = rel.to_string_lossy();

    let included = config.include.iter().any(|pat| glob_match(pat, &rel_str));
    let excluded = config.exclude.iter().any(|pat| glob_match(pat, &rel_str));

    included && !excluded
}
```

### 7.7 Test Plan

| Test                        | Description                                                     |
|-----------------------------|-----------------------------------------------------------------|
| `test_default_config`       | `RpcConfig::default()` matches current hardcoded defaults       |
| `test_parse_minimal`        | Minimal TOML with one field parses correctly, rest are defaults |
| `test_parse_full`           | Full TOML with all fields parses correctly                      |
| `test_parse_invalid`        | Invalid TOML returns a descriptive error                        |
| `test_parse_unknown_field`  | Unknown fields are ignored (forward compat)                     |
| `test_discovery_found`      | Config found in current directory                               |
| `test_discovery_parent`     | Config found in parent directory                                |
| `test_discovery_missing`    | Returns `None` when no config exists                            |
| `test_merge_cli_overrides`  | CLI flags override config file values                           |
| `test_merge_cli_none`       | `None` CLI flags fall through to config values                  |
| `test_include_exclude`      | Glob patterns correctly filter files                            |
| `test_exclude_wins`         | When both match, `exclude` takes priority                       |
| `test_no_config_flag`       | `--no-config` skips file loading entirely                       |
| `test_explicit_config_path` | `--config path` loads from specified location                   |

### 7.8 Migration & Backward Compatibility

- **No breaking changes.** All existing CLI invocations work unchanged.
- Default values match the current hardcoded defaults exactly.
- The config file is optional — projects without it behave identically to before.
- Existing `package.json` scripts continue to work. Users can gradually move flags into `rpc.config.toml` and simplify their scripts.

## 8. Future Extensions

This RFC deliberately keeps the config surface small. The following fields are deferred to future RFCs but the config structure is designed to accommodate them:

```toml
# Future: codegen style
[codegen]
client_style = "factory"             # "class" | "factory" | "hooks"
barrel_export = true

# Future: type overrides for external crates
[codegen.type_overrides]
"chrono::DateTime" = "string"
"uuid::Uuid" = "string"

# Future: bigint mapping
[codegen]
bigint_types = ["i64", "u64"]
```

Each of these will be introduced alongside the feature it configures, as separate RFCs or PRs.

## 9. Alternatives Considered

### JSON (`rpc.config.json`)

Pros: native to JS/TS ecosystem. Cons: no comments, verbose, inconsistent with Rust tooling conventions. Rejected.

### YAML (`rpc.config.yaml`)

Pros: clean syntax. Cons: whitespace-sensitive, requires a heavier parser, not standard in Rust ecosystem. Rejected.

### `package.json` `"rpc"` field

Pros: no extra file. Cons: couples to Node.js, not accessible from pure Rust contexts, no comments. Rejected.

### `Cargo.toml` `[metadata.rpc]`

Pros: no extra file, Rust-native. Cons: `Cargo.toml` lives at crate root, not project root. In monorepos the Rust workspace and the SvelteKit project may be at different levels. A dedicated file avoids this ambiguity. Rejected.
