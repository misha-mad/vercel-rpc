use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::RpcConfig;
use crate::model::Manifest;
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
        let generics = format_generic_params(&s.generics);
        if !s.tuple_fields.is_empty() {
            let types: Vec<String> = s.tuple_fields.iter().map(|t| t.to_string()).collect();
            println!("\n  struct {}{generics}({})", s.name, types.join(", "));
        } else {
            println!("\n  struct {}{generics} {{", s.name);
            for field in &s.fields {
                println!("    {}: {},", field.name, field.ty);
            }
            println!("  }}");
        }
    }

    for e in &manifest.enums {
        let generics = format_generic_params(&e.generics);
        let variants: Vec<&str> = e.variants.iter().map(|v| v.name.as_str()).collect();
        println!(
            "\n  enum {}{generics} {{ {} }}",
            e.name,
            variants.join(", ")
        );
    }

    // Also output raw JSON for tooling consumption
    println!("\n--- JSON manifest ---");
    println!("{}", serde_json::to_string_pretty(&manifest)?);

    Ok(())
}

/// Generates TypeScript type definitions and a typed RPC client from the
/// configured directory, writing the results to the configured output paths.
pub fn cmd_generate(config: &RpcConfig) -> Result<()> {
    let manifest = generate_all(config)?;

    println!(
        "Generated {} procedure(s), {} struct(s), {} enum(s)",
        manifest.procedures.len(),
        manifest.structs.len(),
        manifest.enums.len(),
    );
    println!("  → {}", config.output.types.display());
    println!("  → {}", config.output.client.display());
    if let Some(path) = &config.output.svelte {
        println!("  → {}", path.display());
    }
    if let Some(path) = &config.output.react {
        println!("  → {}", path.display());
    }
    if let Some(path) = &config.output.vue {
        println!("  → {}", path.display());
    }
    if let Some(path) = &config.output.solid {
        println!("  → {}", path.display());
    }

    Ok(())
}

/// Scans the source directory and generates all configured TypeScript output files.
///
/// Returns the manifest so callers can use it for logging/reporting.
pub fn generate_all(config: &RpcConfig) -> Result<Manifest> {
    let manifest = parser::scan_directory(&config.input)?;

    let types_content = codegen::typescript::generate_types_file(
        &manifest,
        config.codegen.preserve_docs,
        config.codegen.naming.fields,
        config.codegen.branded_newtypes,
    );
    write_file(&config.output.types, &types_content)?;

    let client_content = codegen::client::generate_client_file(
        &manifest,
        &config.output.imports.types_specifier(),
        config.codegen.preserve_docs,
    );
    write_file(&config.output.client, &client_content)?;

    write_framework_files(config, &manifest)?;

    Ok(manifest)
}

/// Computes the client import specifier from config (e.g. `"./rpc-client"`).
fn client_import_path(config: &RpcConfig) -> String {
    let client_stem = config
        .output
        .client
        .file_stem()
        .unwrap_or_default()
        .to_string_lossy();
    format!("./{client_stem}{}", config.output.imports.extension)
}

/// A framework codegen entry: optional output path paired with its generator function.
type FrameworkEntry<'a> = (
    Option<&'a PathBuf>,
    fn(&Manifest, &str, &str, bool) -> String,
);

/// Generates and writes all optional framework wrapper files (Svelte, React, Vue, Solid).
fn write_framework_files(config: &RpcConfig, manifest: &Manifest) -> Result<()> {
    let client_import = client_import_path(config);
    let types_specifier = config.output.imports.types_specifier();

    let frameworks: [FrameworkEntry<'_>; 4] = [
        (
            config.output.svelte.as_ref(),
            codegen::svelte::generate_svelte_file,
        ),
        (
            config.output.react.as_ref(),
            codegen::react::generate_react_file,
        ),
        (config.output.vue.as_ref(), codegen::vue::generate_vue_file),
        (
            config.output.solid.as_ref(),
            codegen::solid::generate_solid_file,
        ),
    ];

    for (path_opt, generator) in &frameworks {
        if let Some(path) = path_opt {
            let content = generator(
                manifest,
                &client_import,
                &types_specifier,
                config.codegen.preserve_docs,
            );
            if !content.is_empty() {
                write_file(path, &content)?;
            }
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

/// Formats generic type parameters for display (e.g. `<T>`, `<A, B>`).
fn format_generic_params(generics: &[String]) -> String {
    if generics.is_empty() {
        String::new()
    } else {
        format!("<{}>", generics.join(", "))
    }
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
