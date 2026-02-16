use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::config::RpcConfig;
use crate::{codegen, parser};

pub fn cmd_scan(config: &RpcConfig) -> Result<()> {
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

pub fn cmd_generate(config: &RpcConfig) -> Result<()> {
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
pub fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    fs::write(path, content)
        .with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}

/// Formats byte count in a human-readable way.
pub fn bytecount(s: &str) -> String {
    let bytes = s.len();
    if bytes < 1024 {
        format!("{bytes} bytes")
    } else {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    }
}
