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
            let rel = match e.path().strip_prefix(&input.dir) {
                Ok(r) => r,
                Err(_) => e.path(),
            };
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

    let input = func
        .sig
        .inputs
        .iter()
        .find_map(|arg| match arg {
            FnArg::Typed(pat) => Some(extract_rust_type(&pat.ty)),
            _ => None,
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
        let Ok(nested) = attr.parse_args_with(
            syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
        ) else {
            return false;
        };
        nested.iter().any(|path| {
            path.is_ident("Serialize")
                || path.segments.last().is_some_and(|s| s.ident == "Serialize")
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    fn parse_source(source: &str) -> Manifest {
        let mut tmp = NamedTempFile::new().unwrap();
        write!(tmp, "{}", source).unwrap();
        parse_file(tmp.path()).unwrap()
    }

    #[test]
    fn extracts_query_no_input() {
        let manifest = parse_source(
            r#"
            #[rpc_query]
            async fn version() -> String {
                "1.0".to_string()
            }
            "#,
        );
        assert_eq!(manifest.procedures.len(), 1);
        let proc = &manifest.procedures[0];
        assert_eq!(proc.name, "version");
        assert_eq!(proc.kind, ProcedureKind::Query);
        assert!(proc.input.is_none());
        assert_eq!(proc.output.as_ref().unwrap().name, "String");
    }

    #[test]
    fn extracts_query_with_input() {
        let manifest = parse_source(
            r#"
            #[rpc_query]
            async fn hello(name: String) -> String {
                format!("Hello, {}!", name)
            }
            "#,
        );
        assert_eq!(manifest.procedures.len(), 1);
        let proc = &manifest.procedures[0];
        assert_eq!(proc.input.as_ref().unwrap().name, "String");
    }

    #[test]
    fn extracts_mutation() {
        let manifest = parse_source(
            r#"
            #[rpc_mutation]
            async fn create_item(input: CreateInput) -> Item {
                todo!()
            }
            "#,
        );
        assert_eq!(manifest.procedures.len(), 1);
        let proc = &manifest.procedures[0];
        assert_eq!(proc.kind, ProcedureKind::Mutation);
        assert_eq!(proc.input.as_ref().unwrap().name, "CreateInput");
        assert_eq!(proc.output.as_ref().unwrap().name, "Item");
    }

    #[test]
    fn unwraps_result_return_type() {
        let manifest = parse_source(
            r#"
            #[rpc_query]
            async fn fetch_data() -> Result<Vec<Item>, Error> {
                todo!()
            }
            "#,
        );
        let proc = &manifest.procedures[0];
        let output = proc.output.as_ref().unwrap();
        assert_eq!(output.name, "Vec");
        assert_eq!(output.generics[0].name, "Item");
    }

    #[test]
    fn extracts_serde_structs() {
        let manifest = parse_source(
            r#"
            #[derive(Serialize)]
            struct UserInput {
                name: String,
                age: u32,
            }
            "#,
        );
        assert_eq!(manifest.structs.len(), 1);
        assert_eq!(manifest.structs[0].name, "UserInput");
        assert_eq!(manifest.structs[0].fields.len(), 2);
        assert_eq!(manifest.structs[0].fields[0].0, "name");
    }

    #[test]
    fn ignores_non_rpc_functions() {
        let manifest = parse_source(
            r#"
            async fn helper() -> String {
                "not an rpc".to_string()
            }

            #[rpc_query]
            async fn actual_rpc() -> String {
                "rpc".to_string()
            }
            "#,
        );
        assert_eq!(manifest.procedures.len(), 1);
        assert_eq!(manifest.procedures[0].name, "actual_rpc");
    }

    #[test]
    fn extracts_unit_enum() {
        let manifest = parse_source(
            r#"
            #[derive(Serialize)]
            enum Status {
                Active,
                Inactive,
                Banned,
            }
            "#,
        );
        assert_eq!(manifest.enums.len(), 1);
        let e = &manifest.enums[0];
        assert_eq!(e.name, "Status");
        assert_eq!(e.variants.len(), 3);
        assert_eq!(e.variants[0].name, "Active");
        assert!(matches!(e.variants[0].kind, VariantKind::Unit));
    }

    #[test]
    fn extracts_tuple_enum() {
        let manifest = parse_source(
            r#"
            #[derive(Serialize)]
            enum ApiResponse {
                Ok(String),
                Error(u32, String),
            }
            "#,
        );
        assert_eq!(manifest.enums.len(), 1);
        let e = &manifest.enums[0];
        assert_eq!(e.variants.len(), 2);
        match &e.variants[0].kind {
            VariantKind::Tuple(types) => {
                assert_eq!(types.len(), 1);
                assert_eq!(types[0].name, "String");
            }
            _ => panic!("expected Tuple variant"),
        }
        match &e.variants[1].kind {
            VariantKind::Tuple(types) => assert_eq!(types.len(), 2),
            _ => panic!("expected Tuple variant"),
        }
    }

    #[test]
    fn extracts_struct_enum() {
        let manifest = parse_source(
            r#"
            #[derive(Serialize)]
            enum Event {
                Click { x: i32, y: i32 },
                Message { text: String },
            }
            "#,
        );
        assert_eq!(manifest.enums.len(), 1);
        let e = &manifest.enums[0];
        match &e.variants[0].kind {
            VariantKind::Struct(fields) => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, "x");
                assert_eq!(fields[1].0, "y");
            }
            _ => panic!("expected Struct variant"),
        }
    }

    #[test]
    fn extracts_mixed_enum() {
        let manifest = parse_source(
            r#"
            #[derive(Serialize)]
            enum Shape {
                Circle(f64),
                Rectangle { width: f64, height: f64 },
                Unknown,
            }
            "#,
        );
        let e = &manifest.enums[0];
        assert_eq!(e.variants.len(), 3);
        assert!(matches!(e.variants[0].kind, VariantKind::Tuple(_)));
        assert!(matches!(e.variants[1].kind, VariantKind::Struct(_)));
        assert!(matches!(e.variants[2].kind, VariantKind::Unit));
    }

    #[test]
    fn ignores_non_serde_enum() {
        let manifest = parse_source(
            r#"
            enum NotSerde {
                A,
                B,
            }

            #[derive(Serialize)]
            enum IsSerde {
                X,
                Y,
            }
            "#,
        );
        assert_eq!(manifest.enums.len(), 1);
        assert_eq!(manifest.enums[0].name, "IsSerde");
    }

    /// Helper: write a minimal RPC file so scan_directory finds at least one procedure.
    fn write_rpc_file(dir: &Path, rel_path: &str) {
        let path = dir.join(rel_path);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(
            &path,
            r#"
            #[rpc_query]
            async fn handler() -> String { "ok".into() }
            "#,
        )
        .unwrap();
    }

    #[test]
    fn test_include_filters_files() {
        let tmp = TempDir::new().unwrap();
        write_rpc_file(tmp.path(), "handlers/hello.rs");
        write_rpc_file(tmp.path(), "utils/helper.rs");

        let input = InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["handlers/**/*.rs".into()],
            exclude: vec![],
        };

        let manifest = scan_directory(&input).unwrap();
        assert_eq!(manifest.procedures.len(), 1);
        assert_eq!(manifest.procedures[0].name, "handler");
        // The file from utils/ should not appear
        assert!(manifest.procedures.iter().all(|p| {
            p.source_file
                .to_string_lossy()
                .contains("handlers")
        }));
    }

    #[test]
    fn test_exclude_filters_files() {
        let tmp = TempDir::new().unwrap();
        write_rpc_file(tmp.path(), "hello.rs");
        write_rpc_file(tmp.path(), "test_hello.rs");

        let input = InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["**/*.rs".into()],
            exclude: vec!["test_*.rs".into()],
        };

        let manifest = scan_directory(&input).unwrap();
        assert_eq!(manifest.procedures.len(), 1);
        assert!(manifest.procedures[0]
            .source_file
            .to_string_lossy()
            .contains("hello.rs"));
        assert!(!manifest.procedures[0]
            .source_file
            .to_string_lossy()
            .contains("test_hello.rs"));
    }

    #[test]
    fn test_extracts_doc_comments() {
        let manifest = parse_source(
            r#"
            /// Returns the current server time.
            /// Includes a friendly message.
            #[rpc_query]
            async fn time() -> TimeResponse {
                todo!()
            }
            "#,
        );
        assert_eq!(manifest.procedures.len(), 1);
        let proc = &manifest.procedures[0];
        assert_eq!(
            proc.docs.as_deref(),
            Some("Returns the current server time.\nIncludes a friendly message."),
        );
    }

    #[test]
    fn test_extracts_struct_doc_comments() {
        let manifest = parse_source(
            r#"
            /// A timestamp response.
            #[derive(Serialize)]
            struct TimeResponse {
                timestamp: u64,
            }
            "#,
        );
        assert_eq!(manifest.structs.len(), 1);
        assert_eq!(
            manifest.structs[0].docs.as_deref(),
            Some("A timestamp response."),
        );
    }

    #[test]
    fn test_extracts_enum_doc_comments() {
        let manifest = parse_source(
            r#"
            /// The status of an entity.
            #[derive(Serialize)]
            enum Status {
                Active,
                Inactive,
            }
            "#,
        );
        assert_eq!(manifest.enums.len(), 1);
        assert_eq!(
            manifest.enums[0].docs.as_deref(),
            Some("The status of an entity."),
        );
    }

    #[test]
    fn test_no_doc_comments_returns_none() {
        let manifest = parse_source(
            r#"
            #[rpc_query]
            async fn ping() -> String {
                "pong".to_string()
            }
            "#,
        );
        assert!(manifest.procedures[0].docs.is_none());
    }

    #[test]
    fn test_exclude_wins_over_include() {
        let tmp = TempDir::new().unwrap();
        write_rpc_file(tmp.path(), "hello.rs");
        write_rpc_file(tmp.path(), "world.rs");

        let input = InputConfig {
            dir: tmp.path().to_path_buf(),
            include: vec!["**/*.rs".into()],
            exclude: vec!["hello.rs".into()],
        };

        let manifest = scan_directory(&input).unwrap();
        assert_eq!(manifest.procedures.len(), 1);
        assert!(manifest.procedures[0]
            .source_file
            .to_string_lossy()
            .contains("world.rs"));
    }
}
