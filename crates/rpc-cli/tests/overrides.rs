mod common;

use std::collections::HashMap;
use std::path::PathBuf;

use vercel_rpc_cli::codegen::overrides::{apply_type_overrides, build_base_index};
use vercel_rpc_cli::model::*;

fn make_overrides(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

// --- build_base_index ---

#[test]
fn base_index_strips_path_prefix() {
    let overrides = make_overrides(&[("chrono::DateTime", "string")]);
    let base = build_base_index(&overrides);
    assert_eq!(base.get("DateTime").unwrap(), "string");
    assert!(!base.contains_key("chrono::DateTime"));
}

#[test]
fn base_index_keeps_simple_name() {
    let overrides = make_overrides(&[("Uuid", "string")]);
    let base = build_base_index(&overrides);
    assert_eq!(base.get("Uuid").unwrap(), "string");
}

#[test]
fn base_index_deeply_nested_path() {
    let overrides = make_overrides(&[("serde_json::value::Value", "unknown")]);
    let base = build_base_index(&overrides);
    assert_eq!(base.get("Value").unwrap(), "unknown");
}

// --- override_type matching ---

#[test]
fn override_simple_type() {
    let overrides = make_overrides(&[("DateTime", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field("created_at", RustType::simple("DateTime"))],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    assert_eq!(manifest.structs[0].fields[0].ty.name, "string");
}

#[test]
fn override_clears_generics() {
    let overrides = make_overrides(&[("chrono::DateTime", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field(
                "created_at",
                RustType::with_generics("chrono::DateTime", vec![RustType::simple("Utc")]),
            )],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    let ty = &manifest.structs[0].fields[0].ty;
    assert_eq!(ty.name, "string");
    assert!(ty.generics.is_empty());
}

#[test]
fn override_matches_imported_name_via_base_index() {
    // Config key has full path, but source only has the imported short name
    let overrides = make_overrides(&[("chrono::DateTime", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field(
                "created_at",
                RustType::with_generics("DateTime", vec![RustType::simple("Utc")]),
            )],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    let ty = &manifest.structs[0].fields[0].ty;
    assert_eq!(ty.name, "string");
    assert!(ty.generics.is_empty());
}

#[test]
fn override_nested_in_vec() {
    let overrides = make_overrides(&[("uuid::Uuid", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field(
                "ids",
                RustType::with_generics("Vec", vec![RustType::simple("Uuid")]),
            )],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    let ty = &manifest.structs[0].fields[0].ty;
    assert_eq!(ty.name, "Vec");
    assert_eq!(ty.generics[0].name, "string");
}

#[test]
fn override_nested_in_option() {
    let overrides = make_overrides(&[("Decimal", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field(
                "price",
                RustType::with_generics("Option", vec![RustType::simple("Decimal")]),
            )],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    let inner = &manifest.structs[0].fields[0].ty.generics[0];
    assert_eq!(inner.name, "string");
}

#[test]
fn override_in_procedure_io() {
    let overrides = make_overrides(&[("Uuid", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![Procedure {
            name: "get_user".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("Uuid")),
            output: Some(RustType::simple("User")),
            source_file: PathBuf::from("test.rs"),
            docs: None,
        }],
        structs: vec![],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    assert_eq!(manifest.procedures[0].input.as_ref().unwrap().name, "string");
    assert_eq!(manifest.procedures[0].output.as_ref().unwrap().name, "User");
}

#[test]
fn override_in_enum_tuple_variant() {
    let overrides = make_overrides(&[("Value", "unknown")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Payload".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Json".to_string(),
                kind: VariantKind::Tuple(vec![RustType::simple("Value")]),
                rename: None,
            }],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    match &manifest.enums[0].variants[0].kind {
        VariantKind::Tuple(types) => assert_eq!(types[0].name, "unknown"),
        _ => panic!("expected Tuple"),
    }
}

#[test]
fn override_in_enum_struct_variant() {
    let overrides = make_overrides(&[("DateTime", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Created".to_string(),
                kind: VariantKind::Struct(vec![common::field(
                    "at",
                    RustType::simple("DateTime"),
                )]),
                rename: None,
            }],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    match &manifest.enums[0].variants[0].kind {
        VariantKind::Struct(fields) => assert_eq!(fields[0].ty.name, "string"),
        _ => panic!("expected Struct"),
    }
}

#[test]
fn override_in_tuple_struct() {
    let overrides = make_overrides(&[("Uuid", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserId".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("Uuid")],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    assert_eq!(manifest.structs[0].tuple_fields[0].name, "string");
}

#[test]
fn no_match_preserves_type() {
    let overrides = make_overrides(&[("Uuid", "string")]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field("name", RustType::simple("String"))],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    assert_eq!(manifest.structs[0].fields[0].ty.name, "String");
}

#[test]
fn empty_overrides_noop() {
    let overrides = HashMap::new();
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field("id", RustType::simple("u64"))],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    assert_eq!(manifest.structs[0].fields[0].ty.name, "u64");
}

#[test]
fn exact_full_path_takes_priority_over_base_name() {
    // Two overrides: one exact for chrono::DateTime, one for DateTime
    let overrides = make_overrides(&[
        ("chrono::DateTime", "string"),
        ("DateTime", "Date"),
    ]);
    let base = build_base_index(&overrides);
    let mut manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![common::field(
                "created_at",
                RustType::simple("chrono::DateTime"),
            )],
            tuple_fields: vec![],
            source_file: PathBuf::from("test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    apply_type_overrides(&mut manifest, &overrides, &base);
    // Exact match on "chrono::DateTime" wins
    assert_eq!(manifest.structs[0].fields[0].ty.name, "string");
}
