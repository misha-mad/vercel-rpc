use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

use crate::config::RpcConfig;
use crate::{codegen, parser};

/// Scans the configured directory and prints discovered RPC procedures, structs,
/// and enums to stdout, followed by a JSON manifest.
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
            .map(|t| t.to_string())
            .unwrap_or_else(|| "()".to_string());
        let output_str = proc
            .output
            .as_ref()
            .map(|t| t.to_string())
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
        for field in &s.fields {
            println!("    {}: {},", field.name, field.ty);
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

/// Generates TypeScript type definitions and a typed RPC client from the
/// configured directory, writing the results to the configured output paths.
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

    // Generate svelte wrapper file (opt-in)
    if let Some(svelte_path) = &config.output.svelte {
        let client_stem = config
            .output
            .client
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let client_import = format!("./{client_stem}{}", config.output.imports.extension);
        let svelte_content = codegen::svelte::generate_svelte_file(
            &manifest,
            &client_import,
            &config.output.imports.types_specifier(),
            config.codegen.preserve_docs,
        );
        if !svelte_content.is_empty() {
            write_file(svelte_path, &svelte_content)?;
            println!(
                "Generated {} -> {}",
                svelte_path.display(),
                bytecount(&svelte_content),
            );
        }
    }

    // Generate react wrapper file (opt-in)
    if let Some(react_path) = &config.output.react {
        let client_stem = config
            .output
            .client
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let client_import = format!("./{client_stem}{}", config.output.imports.extension);
        let react_content = codegen::react::generate_react_file(
            &manifest,
            &client_import,
            &config.output.imports.types_specifier(),
            config.codegen.preserve_docs,
        );
        if !react_content.is_empty() {
            write_file(react_path, &react_content)?;
            println!(
                "Generated {} -> {}",
                react_path.display(),
                bytecount(&react_content),
            );
        }
    }

    // Generate solid wrapper file (opt-in)
    if let Some(solid_path) = &config.output.solid {
        let client_stem = config
            .output
            .client
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let client_import = format!("./{client_stem}{}", config.output.imports.extension);
        let solid_content = codegen::solid::generate_solid_file(
            &manifest,
            &client_import,
            &config.output.imports.types_specifier(),
            config.codegen.preserve_docs,
        );
        if !solid_content.is_empty() {
            write_file(solid_path, &solid_content)?;
            println!(
                "Generated {} -> {}",
                solid_path.display(),
                bytecount(&solid_content),
            );
        }
    }

    // Generate vue wrapper file (opt-in)
    if let Some(vue_path) = &config.output.vue {
        let client_stem = config
            .output
            .client
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();
        let client_import = format!("./{client_stem}{}", config.output.imports.extension);
        let vue_content = codegen::vue::generate_vue_file(
            &manifest,
            &client_import,
            &config.output.imports.types_specifier(),
            config.codegen.preserve_docs,
        );
        if !vue_content.is_empty() {
            write_file(vue_path, &vue_content)?;
            println!(
                "Generated {} -> {}",
                vue_path.display(),
                bytecount(&vue_content),
            );
        }
    }

    Ok(())
}

/// Writes content to a file, creating parent directories as needed.
pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    fs::write(path, content).with_context(|| format!("Failed to write {}", path.display()))?;
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
