use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use metaxy_cli::config::FieldNaming;
use metaxy_cli::{commands, config, watch};

#[derive(Parser)]
#[command(
    name = "metaxy",
    about = "Metaxy CLI — parse Rust functions and generate TypeScript bindings"
)]
struct Cli {
    /// Path to the config file (default: auto-discover metaxy.config.toml)
    #[arg(long, global = true)]
    config: Option<PathBuf>,

    /// Disable config file loading
    #[arg(long, global = true)]
    no_config: bool,

    #[command(subcommand)]
    command: Command,
}

/// Shared input arguments for scan, generate, and watch commands.
#[derive(clap::Args)]
struct InputArgs {
    /// Path to the directory containing Rust lambda source files
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// Glob patterns for files to include (repeatable)
    #[arg(long)]
    include: Vec<String>,

    /// Glob patterns for files to exclude (repeatable)
    #[arg(long)]
    exclude: Vec<String>,
}

/// Shared codegen arguments for generate and watch commands.
#[derive(clap::Args)]
struct GenerateArgs {
    #[command(flatten)]
    input: InputArgs,

    /// Output path for the generated TypeScript types file
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Output path for the generated TypeScript client file
    #[arg(short, long)]
    client_output: Option<PathBuf>,

    /// Output path for the generated Svelte 5 reactive wrapper file
    #[arg(long)]
    svelte_output: Option<PathBuf>,

    /// Output path for the generated React hooks wrapper file
    #[arg(long)]
    react_output: Option<PathBuf>,

    /// Output path for the generated Vue 3 composable wrapper file
    #[arg(long)]
    vue_output: Option<PathBuf>,

    /// Output path for the generated SolidJS reactive primitives file
    #[arg(long)]
    solid_output: Option<PathBuf>,

    /// Import path for the types file used in the client (relative, without extension)
    #[arg(long)]
    types_import: Option<String>,

    /// Suffix appended to the types import specifier (e.g. ".js" for ESM)
    #[arg(long)]
    extension: Option<String>,

    /// Forward Rust doc comments as JSDoc in generated TypeScript
    #[arg(long)]
    preserve_docs: bool,

    /// Generate branded types for single-field tuple structs (newtypes)
    #[arg(long)]
    branded_newtypes: bool,

    /// Field naming convention for generated TypeScript interfaces
    #[arg(long, value_enum)]
    fields: Option<FieldNaming>,

    /// Map a Rust type to a TypeScript type (repeatable, e.g. "chrono::DateTime=string")
    #[arg(long = "type-override", value_parser = parse_type_override)]
    type_overrides: Vec<(String, String)>,

    /// Map a Rust integer type to TypeScript `bigint` (repeatable, e.g. "i64", "u64")
    #[arg(long = "bigint-type")]
    bigint_types: Vec<String>,
}

fn parse_type_override(s: &str) -> Result<(String, String), String> {
    let (key, value) = s
        .split_once('=')
        .ok_or_else(|| format!("expected KEY=VALUE, got `{s}`"))?;
    Ok((key.to_string(), value.to_string()))
}

#[derive(Subcommand)]
enum Command {
    /// Scan the api/ directory and print discovered RPC procedures as JSON
    Scan {
        #[command(flatten)]
        input: InputArgs,
    },

    /// Generate TypeScript type definitions and client from Rust lambda source files
    Generate {
        #[command(flatten)]
        args: GenerateArgs,
    },

    /// Watch the api/ directory and regenerate TypeScript files on changes
    Watch {
        #[command(flatten)]
        args: GenerateArgs,

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
        Command::Scan { input } => {
            let cfg = config::resolve(config::CliOverrides {
                config: cli.config,
                no_config: cli.no_config,
                dir: input.dir,
                include: input.include,
                exclude: input.exclude,
                ..config::CliOverrides::default()
            })?;
            commands::cmd_scan(&cfg)
        }
        Command::Generate { args } => {
            let cfg = config::resolve(config::CliOverrides {
                config: cli.config,
                no_config: cli.no_config,
                dir: args.input.dir,
                include: args.input.include,
                exclude: args.input.exclude,
                output: args.output,
                client_output: args.client_output,
                svelte_output: args.svelte_output,
                react_output: args.react_output,
                vue_output: args.vue_output,
                solid_output: args.solid_output,
                types_import: args.types_import,
                extension: args.extension,
                preserve_docs: args.preserve_docs,
                branded_newtypes: if args.branded_newtypes {
                    Some(true)
                } else {
                    None
                },
                fields: args.fields,
                type_overrides: args.type_overrides,
                bigint_types: args.bigint_types,
                ..config::CliOverrides::default()
            })?;
            commands::cmd_generate(&cfg)
        }
        Command::Watch {
            args,
            debounce_ms,
            clear_screen,
        } => {
            let cfg = config::resolve(config::CliOverrides {
                config: cli.config,
                no_config: cli.no_config,
                dir: args.input.dir,
                include: args.input.include,
                exclude: args.input.exclude,
                output: args.output,
                client_output: args.client_output,
                svelte_output: args.svelte_output,
                react_output: args.react_output,
                vue_output: args.vue_output,
                solid_output: args.solid_output,
                types_import: args.types_import,
                extension: args.extension,
                preserve_docs: args.preserve_docs,
                branded_newtypes: if args.branded_newtypes {
                    Some(true)
                } else {
                    None
                },
                fields: args.fields,
                type_overrides: args.type_overrides,
                bigint_types: args.bigint_types,
                debounce_ms,
                clear_screen,
            })?;
            watch::run(&cfg)
        }
    }
}
