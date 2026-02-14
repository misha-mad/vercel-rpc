mod codegen;
mod model;
mod parser;
mod watch;

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rpc", about = "Vercel RPC CLI — parse Rust lambdas and generate TypeScript bindings")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Scan the api/ directory and print discovered RPC procedures as JSON
    Scan {
        /// Path to the directory containing Rust lambda source files
        #[arg(short, long, default_value = "api")]
        dir: PathBuf,
    },

    /// Generate TypeScript type definitions and client from Rust lambda source files
    Generate {
        /// Path to the directory containing Rust lambda source files
        #[arg(short, long, default_value = "api")]
        dir: PathBuf,

        /// Output path for the generated TypeScript types file
        #[arg(short, long, default_value = "src/lib/rpc-types.ts")]
        output: PathBuf,

        /// Output path for the generated TypeScript client file
        #[arg(short, long, default_value = "src/lib/rpc-client.ts")]
        client_output: PathBuf,

        /// Import path for the types file used in the client (relative, without extension)
        #[arg(long, default_value = "./rpc-types")]
        types_import: String,
    },

    /// Watch the api/ directory and regenerate TypeScript files on changes
    Watch {
        /// Path to the directory containing Rust lambda source files
        #[arg(short, long, default_value = "api")]
        dir: PathBuf,

        /// Output path for the generated TypeScript types file
        #[arg(short, long, default_value = "src/lib/rpc-types.ts")]
        output: PathBuf,

        /// Output path for the generated TypeScript client file
        #[arg(short, long, default_value = "src/lib/rpc-client.ts")]
        client_output: PathBuf,

        /// Import path for the types file used in the client (relative, without extension)
        #[arg(long, default_value = "./rpc-types")]
        types_import: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Scan { dir } => cmd_scan(&dir),
        Command::Generate { dir, output, client_output, types_import } => {
            cmd_generate(&dir, &output, &client_output, &types_import)
        }
        Command::Watch { dir, output, client_output, types_import } => {
            watch::run(&watch::WatchConfig {
                api_dir: dir,
                types_output: output,
                client_output,
                types_import,
            })
        }
    }
}

fn cmd_scan(dir: &PathBuf) -> Result<()> {
    let manifest = parser::scan_directory(dir)?;

    println!(
        "Discovered {} procedure(s) and {} struct(s):\n",
        manifest.procedures.len(),
        manifest.structs.len(),
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

    // Also output raw JSON for tooling consumption
    println!("\n--- JSON manifest ---");
    println!("{}", serde_json::to_string_pretty(&manifest)?);

    Ok(())
}

fn cmd_generate(dir: &PathBuf, output: &PathBuf, client_output: &PathBuf, types_import: &str) -> Result<()> {
    let manifest = parser::scan_directory(dir)?;

    // Generate types file
    let types_content = codegen::typescript::generate_types_file(&manifest);
    write_file(output, &types_content)?;
    println!(
        "Generated {} ({} procedure(s), {} struct(s)) -> {}",
        output.display(),
        manifest.procedures.len(),
        manifest.structs.len(),
        bytecount(&types_content),
    );

    // Generate client file
    let client_content = codegen::client::generate_client_file(&manifest, types_import);
    write_file(client_output, &client_content)?;
    println!(
        "Generated {} -> {}",
        client_output.display(),
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
