use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use vercel_rpc_cli::config::FieldNaming;
use vercel_rpc_cli::{commands, config, watch};

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
            commands::cmd_scan(&cfg)
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
            commands::cmd_generate(&cfg)
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
