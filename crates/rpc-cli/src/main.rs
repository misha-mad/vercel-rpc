//! CLI tool for the [vercel-rpc](https://github.com/misha-mad/vercel-rpc) project.
//!
//! Scans Rust lambda source files for `#[rpc_query]` / `#[rpc_mutation]`
//! functions and `#[derive(Serialize)]` types, then generates TypeScript type
//! definitions and a fully typed RPC client.
//!
//! # Binary
//!
//! The installed binary is called `rpc` and provides three subcommands:
//!
//! - **`rpc scan`** — parse a directory and print discovered procedures as
//!   human-readable text plus a JSON manifest.
//! - **`rpc generate`** — produce `rpc-types.ts` (interfaces + `Procedures`
//!   type) and `rpc-client.ts` (typed `RpcClient` + `createRpcClient` factory).
//! - **`rpc watch`** — same as `generate`, but re-runs automatically whenever
//!   a `.rs` file changes (configurable debounce).
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐  scan   ┌──────────┐  codegen  ┌────────────────┐
//! │  api/*.rs   │ ──────► │ Manifest │ ────────► │ rpc-types.ts   │
//! │  attributes │  (syn)  │          │ (fmt)     │ rpc-client.ts  │
//! └─────────────┘         └──────────┘           └────────────────┘
//! ```
//!
//! - [`parser`] — walks the source directory, parses each `.rs` file with
//!   `syn`, and builds a [`model::Manifest`].
//! - [`codegen::typescript`] — converts the manifest into a `rpc-types.ts`
//!   file with TypeScript interfaces, enum types, and a `Procedures` map.
//! - [`codegen::client`] — converts the manifest into a `rpc-client.ts` file
//!   with a typed `RpcClient` interface and `createRpcClient` factory.
//! - [`watch`] — wraps `generate` in a file-watcher loop with debouncing.

mod codegen;
mod config;
mod model;
mod parser;
mod watch;

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

use config::{FieldNaming, RpcConfig};

#[derive(Parser)]
#[command(name = "rpc", about = "Vercel RPC CLI — parse Rust lambdas and generate TypeScript bindings")]
struct Cli {
    /// Path to the config file (default: auto-discover rpc.config.toml)
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    /// Disable config file loading
    #[arg(long, global = true)]
    no_config: bool,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan the api/ directory and print discovered RPC procedures as JSON
    Scan {
        /// Path to the directory containing Rust lambda source files
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Glob patterns for files to include (repeatable)
        #[arg(long)]
        include: Vec<String>,

        /// Glob patterns for files to exclude (repeatable)
        #[arg(long)]
        exclude: Vec<String>,
    },

    /// Generate TypeScript type definitions and client from Rust lambda source files
    Generate {
        /// Path to the directory containing Rust lambda source files
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Glob patterns for files to include (repeatable)
        #[arg(long)]
        include: Vec<String>,

        /// Glob patterns for files to exclude (repeatable)
        #[arg(long)]
        exclude: Vec<String>,

        /// Output path for the generated TypeScript types file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output path for the generated TypeScript client file
        #[arg(short, long)]
        client_output: Option<PathBuf>,

        /// Import path for the types file used in the client (relative, without extension)
        #[arg(long)]
        types_import: Option<String>,

        /// Suffix appended to the types import specifier (e.g. ".js" for ESM)
        #[arg(long)]
        extension: Option<String>,

        /// Forward Rust doc comments as JSDoc in generated TypeScript
        #[arg(long)]
        preserve_docs: bool,

        /// Field naming convention for generated TypeScript interfaces
        #[arg(long, value_enum)]
        fields: Option<FieldNaming>,
    },

    /// Watch the api/ directory and regenerate TypeScript files on changes
    Watch {
        /// Path to the directory containing Rust lambda source files
        #[arg(short, long)]
        dir: Option<PathBuf>,

        /// Glob patterns for files to include (repeatable)
        #[arg(long)]
        include: Vec<String>,

        /// Glob patterns for files to exclude (repeatable)
        #[arg(long)]
        exclude: Vec<String>,

        /// Output path for the generated TypeScript types file
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output path for the generated TypeScript client file
        #[arg(short, long)]
        client_output: Option<PathBuf>,

        /// Import path for the types file used in the client (relative, without extension)
        #[arg(long)]
        types_import: Option<String>,

        /// Suffix appended to the types import specifier (e.g. ".js" for ESM)
        #[arg(long)]
        extension: Option<String>,

        /// Forward Rust doc comments as JSDoc in generated TypeScript
        #[arg(long)]
        preserve_docs: bool,

        /// Field naming convention for generated TypeScript interfaces
        #[arg(long, value_enum)]
        fields: Option<FieldNaming>,

        /// File watcher debounce interval in milliseconds
        #[arg(long)]
        debounce_ms: Option<u64>,

        /// Clear the terminal before each regeneration
        #[arg(long)]
        clear_screen: bool,
    },
}

#[cfg(not(tarpaulin_include))]
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { dir, include, exclude } => {
            let cfg = config::resolve(&config::CliOverrides {
                config: cli.config,
                no_config: cli.no_config,
                dir,
                include,
                exclude,
                output: None,
                client_output: None,
                types_import: None,
                extension: None,
                preserve_docs: false,
                fields: None,
                debounce_ms: None,
                clear_screen: false,
            })?;
            cmd_scan(&cfg)
        }
        Command::Generate {
            dir, include, exclude, output, client_output,
            types_import, extension, preserve_docs, fields,
        } => {
            let cfg = config::resolve(&config::CliOverrides {
                config: cli.config,
                no_config: cli.no_config,
                dir,
                include,
                exclude,
                output,
                client_output,
                types_import,
                extension,
                preserve_docs,
                fields,
                debounce_ms: None,
                clear_screen: false,
            })?;
            cmd_generate(&cfg)
        }
        Command::Watch {
            dir, include, exclude, output, client_output,
            types_import, extension, preserve_docs, fields,
            debounce_ms, clear_screen,
        } => {
            let cfg = config::resolve(&config::CliOverrides {
                config: cli.config,
                no_config: cli.no_config,
                dir,
                include,
                exclude,
                output,
                client_output,
                types_import,
                extension,
                preserve_docs,
                fields,
                debounce_ms,
                clear_screen,
            })?;
            watch::run(&cfg)
        }
    }
}

fn cmd_scan(config: &RpcConfig) -> Result<()> {
    let manifest = parser::scan_directory(&config.input)?;

    println!(
        "Discovered {} procedure(s), {} struct(s), {} enum(s):\n",
        manifest.procedures.len(),
        manifest.structs.len(),
        manifest.enums.len(),
    );

    for proc in &manifest.procedures {
        let input_str = proc
            .input
            .as_ref()
            .map(|t| t.display())
            .unwrap_or_else(|| "()".to_string());
        let output_str = proc
            .output
            .as_ref()
            .map(|t| t.display())
            .unwrap_or_else(|| "()".to_string());

        println!(
            "  {:?} {} ({}) -> {}  [{}]",
            proc.kind,
            proc.name,
            input_str,
            output_str,
            proc.source_file.display(),
        );
    }

    for s in &manifest.structs {
        println!("\n  struct {} {{", s.name);
        for (name, ty) in &s.fields {
            println!("    {}: {},", name, ty.display());
        }
        println!("  }}");
    }

    for e in &manifest.enums {
        let variants: Vec<&str> = e.variants.iter().map(|v| v.name.as_str()).collect();
        println!("\n  enum {} {{ {} }}", e.name, variants.join(", "));
    }

    // Also output raw JSON for tooling consumption
    println!("\n--- JSON manifest ---");
    println!("{}", serde_json::to_string_pretty(&manifest)?);

    Ok(())
}

fn cmd_generate(config: &RpcConfig) -> Result<()> {
    let manifest = parser::scan_directory(&config.input)?;

    // Generate types file
    let types_content = codegen::typescript::generate_types_file(
        &manifest,
        config.codegen.preserve_docs,
        config.codegen.naming.fields,
    );
    write_file(&config.output.types, &types_content)?;
    println!(
        "Generated {} ({} procedure(s), {} struct(s), {} enum(s)) -> {}",
        config.output.types.display(),
        manifest.procedures.len(),
        manifest.structs.len(),
        manifest.enums.len(),
        bytecount(&types_content),
    );

    // Generate client file
    let client_content = codegen::client::generate_client_file(
        &manifest,
        &config.output.imports.types_specifier(),
        config.codegen.preserve_docs,
    );
    write_file(&config.output.client, &client_content)?;
    println!(
        "Generated {} -> {}",
        config.output.client.display(),
        bytecount(&client_content),
    );

    Ok(())
}

/// Writes content to a file, creating parent directories as needed.
fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    fs::write(path, content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Formats byte count in a human-readable way.
fn bytecount(s: &str) -> String {
    let bytes = s.len();
    if bytes < 1024 {
        format!("{bytes} bytes")
    } else {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    // --- bytecount ---

    #[test]
    fn bytecount_small() {
        assert_eq!(bytecount("hello"), "5 bytes");
    }

    #[test]
    fn bytecount_empty() {
        assert_eq!(bytecount(""), "0 bytes");
    }

    #[test]
    fn bytecount_kilobytes() {
        let s = "x".repeat(2048);
        assert_eq!(bytecount(&s), "2.0 KB");
    }

    #[test]
    fn bytecount_boundary() {
        let s = "x".repeat(1023);
        assert_eq!(bytecount(&s), "1023 bytes");
    }

    // --- write_file ---

    #[test]
    fn write_file_creates_parent_dirs() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("a/b/c/output.ts");
        write_file(&path, "content").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "content");
    }

    #[test]
    fn write_file_overwrites_existing() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("out.ts");
        write_file(&path, "first").unwrap();
        write_file(&path, "second").unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "second");
    }

    // --- cmd_scan ---

    #[test]
    fn cmd_scan_empty_dir_errors() {
        let tmp = TempDir::new().unwrap();
        let cfg = RpcConfig {
            input: config::InputConfig {
                dir: tmp.path().to_path_buf(),
                include: vec!["**/*.rs".into()],
                exclude: vec![],
            },
            ..RpcConfig::default()
        };
        let err = cmd_scan(&cfg).unwrap_err();
        assert!(err.to_string().contains("No .rs files found"));
    }

    #[test]
    fn cmd_scan_with_procedure() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("hello.rs"),
            r#"
use serde::Serialize;

#[derive(Serialize)]
struct Greeting {
    message: String,
}

#[rpc_query]
async fn hello(name: String) -> Greeting {
    Greeting { message: format!("Hello, {}!", name) }
}
"#,
        )
        .unwrap();

        let cfg = RpcConfig {
            input: config::InputConfig {
                dir: tmp.path().to_path_buf(),
                include: vec!["**/*.rs".into()],
                exclude: vec![],
            },
            ..RpcConfig::default()
        };
        cmd_scan(&cfg).unwrap();
    }

    #[test]
    fn cmd_scan_with_enum() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("status.rs"),
            r#"
use serde::Serialize;

#[derive(Serialize)]
enum Status {
    Active,
    Inactive,
}

#[rpc_query]
async fn get_status() -> Status {
    Status::Active
}
"#,
        )
        .unwrap();

        let cfg = RpcConfig {
            input: config::InputConfig {
                dir: tmp.path().to_path_buf(),
                include: vec!["**/*.rs".into()],
                exclude: vec![],
            },
            ..RpcConfig::default()
        };
        cmd_scan(&cfg).unwrap();
    }

    // --- cmd_generate ---

    #[test]
    fn cmd_generate_produces_files() {
        let tmp = TempDir::new().unwrap();
        let api_dir = tmp.path().join("api");
        fs::create_dir(&api_dir).unwrap();
        fs::write(
            api_dir.join("ping.rs"),
            r#"
#[rpc_query]
async fn ping() -> String {
    "pong".to_string()
}
"#,
        )
        .unwrap();

        let types_path = tmp.path().join("out/rpc-types.ts");
        let client_path = tmp.path().join("out/rpc-client.ts");

        let cfg = RpcConfig {
            input: config::InputConfig {
                dir: api_dir,
                include: vec!["**/*.rs".into()],
                exclude: vec![],
            },
            output: config::OutputConfig {
                types: types_path.clone(),
                client: client_path.clone(),
                imports: config::ImportsConfig::default(),
            },
            ..RpcConfig::default()
        };
        cmd_generate(&cfg).unwrap();

        let types = fs::read_to_string(&types_path).unwrap();
        assert!(types.contains("ping"));

        let client = fs::read_to_string(&client_path).unwrap();
        assert!(client.contains("ping"));
    }

    #[test]
    fn cmd_generate_empty_dir_errors() {
        let tmp = TempDir::new().unwrap();
        let types_path = tmp.path().join("rpc-types.ts");
        let client_path = tmp.path().join("rpc-client.ts");

        let cfg = RpcConfig {
            input: config::InputConfig {
                dir: tmp.path().to_path_buf(),
                include: vec!["**/*.rs".into()],
                exclude: vec![],
            },
            output: config::OutputConfig {
                types: types_path,
                client: client_path,
                imports: config::ImportsConfig::default(),
            },
            ..RpcConfig::default()
        };
        let err = cmd_generate(&cfg).unwrap_err();
        assert!(err.to_string().contains("No .rs files found"));
    }
}
