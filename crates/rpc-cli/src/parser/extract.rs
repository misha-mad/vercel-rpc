use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use syn::{Attribute, File, FnArg, Item, ItemFn, ReturnType};
use walkdir::WalkDir;

use crate::config::InputConfig;
use crate::model::{EnumDef, EnumVariant, Manifest, Procedure, ProcedureKind, StructDef, VariantKind};
use super::types::{extract_rust_type, extract_struct_fields};

/// RPC attribute names recognized by the parser.
const RPC_QUERY_ATTR: &str = "rpc_query";
const RPC_MUTATION_ATTR: &str = "rpc_mutation";

/// Builds a `GlobSet` from a list of glob pattern strings.
fn build_glob_set(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        let glob = GlobBuilder::new(pattern)
            .literal_separator(false)
            .build()
            .with_context(|| format!("Invalid glob pattern: {pattern}"))?;
        builder.add(glob);
    }
    builder
        .build()
        .context("Failed to build glob set")
}

/// Scans `.rs` files in the configured directory and extracts RPC metadata.
///
/// Walks the directory recursively, applying `include`/`exclude` glob patterns
/// from the config, then parsing each matching Rust source file for
/// `#[rpc_query]` / `#[rpc_mutation]` annotated functions and `#[derive(Serialize)]` structs.
pub fn scan_directory(input: &InputConfig) -> Result<Manifest> {
    let mut manifest = Manifest::default();

    let include_set = build_glob_set(&input.include)?;
    let exclude_set = build_glob_set(&input.exclude)?;

    let entries: Vec<_> = WalkDir::new(&input.dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            if !e.path().extension().is_some_and(|ext| ext == "rs") {
                return false;
            }
            let rel = e.path().strip_prefix(&input.dir).unwrap_or(e.path());
            include_set.is_match(rel) && !exclude_set.is_match(rel)
        })
        .collect();

    if entries.is_empty() {
        anyhow::bail!(
            "No .rs files found in {}",
            input.dir.display()
        );
    }

    for entry in entries {
        let path = entry.path();
        let file_manifest = parse_file(path)
            .with_context(|| format!("Failed to parse {}", path.display()))?;

        manifest.procedures.extend(file_manifest.procedures);
        manifest.structs.extend(file_manifest.structs);
        manifest.enums.extend(file_manifest.enums);
    }

    // Sort for deterministic output
    manifest.procedures.sort_by(|a, b| a.name.cmp(&b.name));
    manifest.structs.sort_by(|a, b| a.name.cmp(&b.name));
    manifest.enums.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(manifest)
}

/// Parses a single Rust source file and extracts all RPC procedures and struct definitions.
pub fn parse_file(path: &Path) -> Result<Manifest> {
    let source = fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path.display()))?;

    let syntax: File = syn::parse_file(&source)
        .with_context(|| format!("Syntax error in {}", path.display()))?;

    let mut manifest = Manifest::default();

    for item in &syntax.items {
        match item {
            Item::Fn(func) => {
                if let Some(procedure) = try_extract_procedure(func, path) {
                    manifest.procedures.push(procedure);
                }
            }
            Item::Struct(item_struct) => {
                if has_serde_derive(&item_struct.attrs) {
                    let fields = extract_struct_fields(&item_struct.fields);
                    let docs = extract_docs(&item_struct.attrs);
                    manifest.structs.push(StructDef {
                        name: item_struct.ident.to_string(),
                        fields,
                        source_file: path.to_path_buf(),
                        docs,
                    });
                }
            }
            Item::Enum(item_enum) => {
                if has_serde_derive(&item_enum.attrs) {
                    let variants = extract_enum_variants(item_enum);
                    let docs = extract_docs(&item_enum.attrs);
                    manifest.enums.push(EnumDef {
                        name: item_enum.ident.to_string(),
                        variants,
                        source_file: path.to_path_buf(),
                        docs,
                    });
                }
            }
            _ => {}
        }
    }

    Ok(manifest)
}

/// Extracts doc comments from `#[doc = "..."]` attributes (written as `///` in source).
///
/// Returns `None` if no doc comments are present.
fn extract_docs(attrs: &[Attribute]) -> Option<String> {
    let lines: Vec<String> = attrs
        .iter()
        .filter_map(|attr| {
            if !attr.path().is_ident("doc") {
                return None;
            }
            if let syn::Meta::NameValue(nv) = &attr.meta {
                if let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = &nv.value
                {
                    let text = s.value();
                    // `///` comments produce a leading space, strip it
                    return Some(text.strip_prefix(' ').unwrap_or(&text).to_string());
                }
            }
            None
        })
        .collect();

    if lines.is_empty() {
        None
    } else {
        Some(lines.join("\n"))
    }
}

/// Attempts to extract an RPC procedure from a function item.
/// Returns `None` if the function doesn't have an RPC attribute.
fn try_extract_procedure(func: &ItemFn, path: &Path) -> Option<Procedure> {
    let kind = detect_rpc_kind(&func.attrs)?;
    let name = func.sig.ident.to_string();
    let docs = extract_docs(&func.attrs);

    let input = func.sig.inputs.iter().find_map(|arg| {
        let FnArg::Typed(pat) = arg else { return None };
        Some(extract_rust_type(&pat.ty))
    });

    let output = match &func.sig.output {
        ReturnType::Default => None,
        ReturnType::Type(_, ty) => {
            let rust_type = extract_rust_type(ty);
            // Unwrap Result<T, _> to just T
            if rust_type.name == "Result" && !rust_type.generics.is_empty() {
                Some(rust_type.generics[0].clone())
            } else {
                Some(rust_type)
            }
        }
    };

    Some(Procedure {
        name,
        kind,
        input,
        output,
        source_file: path.to_path_buf(),
        docs,
    })
}

/// Checks function attributes for `#[rpc_query]` or `#[rpc_mutation]`.
fn detect_rpc_kind(attrs: &[Attribute]) -> Option<ProcedureKind> {
    for attr in attrs {
        if attr.path().is_ident(RPC_QUERY_ATTR) {
            return Some(ProcedureKind::Query);
        }
        if attr.path().is_ident(RPC_MUTATION_ATTR) {
            return Some(ProcedureKind::Mutation);
        }
    }
    None
}

/// Extracts variants from a Rust enum into `EnumVariant` representations.
fn extract_enum_variants(item_enum: &syn::ItemEnum) -> Vec<EnumVariant> {
    item_enum
        .variants
        .iter()
        .map(|v| {
            let name = v.ident.to_string();
            let kind = match &v.fields {
                syn::Fields::Unit => VariantKind::Unit,
                syn::Fields::Unnamed(fields) => {
                    let types = fields
                        .unnamed
                        .iter()
                        .map(|f| extract_rust_type(&f.ty))
                        .collect();
                    VariantKind::Tuple(types)
                }
                syn::Fields::Named(fields) => {
                    let named = fields
                        .named
                        .iter()
                        .filter_map(|f| {
                            let field_name = f.ident.as_ref()?.to_string();
                            let ty = extract_rust_type(&f.ty);
                            Some((field_name, ty))
                        })
                        .collect();
                    VariantKind::Struct(named)
                }
            };
            EnumVariant { name, kind }
        })
        .collect()
}

/// Checks if a struct has `#[derive(Serialize)]` or `#[derive(serde::Serialize)]`.
fn has_serde_derive(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if !attr.path().is_ident("derive") {
            return false;
        }
        attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        )
        .is_ok_and(|nested| {
            nested.iter().any(|path| {
                path.is_ident("Serialize")
                    || path.segments.last().is_some_and(|s| s.ident == "Serialize")
            })
        })
    })
}
