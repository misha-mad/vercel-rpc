mod common;

use std::path::PathBuf;

use vercel_rpc_cli::codegen::typescript::{generate_types_file, rust_type_to_ts, to_camel_case};
use vercel_rpc_cli::config::FieldNaming;
use vercel_rpc_cli::model::*;

// --- rust_type_to_ts ---

#[test]
fn maps_string_types() {
    assert_eq!(rust_type_to_ts(&RustType::simple("String")), "string");
    assert_eq!(rust_type_to_ts(&RustType::simple("str")), "string");
    assert_eq!(rust_type_to_ts(&RustType::simple("char")), "string");
}

#[test]
fn maps_numeric_types() {
    for name in [
        "i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "usize", "isize",
    ] {
        assert_eq!(
            rust_type_to_ts(&RustType::simple(name)),
            "number",
            "failed for {name}"
        );
    }
}

#[test]
fn maps_bool() {
    assert_eq!(rust_type_to_ts(&RustType::simple("bool")), "boolean");
}

#[test]
fn maps_unit() {
    assert_eq!(rust_type_to_ts(&RustType::simple("()")), "void");
}

#[test]
fn maps_vec() {
    let ty = RustType::with_generics("Vec", vec![RustType::simple("String")]);
    assert_eq!(rust_type_to_ts(&ty), "string[]");
}

#[test]
fn maps_vec_of_option() {
    let inner = RustType::with_generics("Option", vec![RustType::simple("i32")]);
    let ty = RustType::with_generics("Vec", vec![inner]);
    assert_eq!(rust_type_to_ts(&ty), "(number | null)[]");
}

#[test]
fn maps_option() {
    let ty = RustType::with_generics("Option", vec![RustType::simple("String")]);
    assert_eq!(rust_type_to_ts(&ty), "string | null");
}

#[test]
fn maps_hashmap() {
    let ty = RustType::with_generics(
        "HashMap",
        vec![RustType::simple("String"), RustType::simple("i32")],
    );
    assert_eq!(rust_type_to_ts(&ty), "Record<string, number>");
}

#[test]
fn maps_btreemap() {
    let ty = RustType::with_generics(
        "BTreeMap",
        vec![RustType::simple("String"), RustType::simple("bool")],
    );
    assert_eq!(rust_type_to_ts(&ty), "Record<string, boolean>");
}

#[test]
fn maps_tuple() {
    let ty = RustType::with_generics(
        "tuple",
        vec![RustType::simple("String"), RustType::simple("i32")],
    );
    assert_eq!(rust_type_to_ts(&ty), "[string, number]");
}

#[test]
fn maps_custom_struct() {
    assert_eq!(
        rust_type_to_ts(&RustType::simple("TimeResponse")),
        "TimeResponse"
    );
}

#[test]
fn maps_nested_generics() {
    // Option<Vec<String>> → string[] | null
    let ty = RustType::with_generics(
        "Option",
        vec![RustType::with_generics(
            "Vec",
            vec![RustType::simple("String")],
        )],
    );
    assert_eq!(rust_type_to_ts(&ty), "string[] | null");
}

// --- generate_types_file ---

#[test]
fn generates_complete_types_file() {
    let manifest = common::make_test_manifest();
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);

    // Header present
    assert!(output.starts_with("// This file is auto-generated"));

    // Interfaces generated
    assert!(output.contains("export interface TimeResponse {"));
    assert!(output.contains("  timestamp: number;"));
    assert!(output.contains("  message: string;"));

    assert!(output.contains("export interface CreateInput {"));
    assert!(output.contains("  title: string;"));
    assert!(output.contains("  count: number;"));

    assert!(output.contains("export interface Item {"));
    assert!(output.contains("  id: number;"));
    assert!(output.contains("  tags: string[];"));

    // Procedures type
    assert!(output.contains("export type Procedures = {"));
    assert!(output.contains("  queries: {"));
    assert!(output.contains("    hello: { input: string; output: string };"));
    assert!(output.contains("    time: { input: void; output: TimeResponse };"));
    assert!(output.contains("  mutations: {"));
    assert!(output.contains("    create_item: { input: CreateInput; output: Item };"));
}

#[test]
fn generates_empty_manifest() {
    let manifest = Manifest::default();
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);

    assert!(output.contains("queries: {"));
    assert!(output.contains("mutations: {"));
    // No interfaces
    assert!(!output.contains("export interface"));
}

#[test]
fn generates_queries_only() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "ping".to_string(),
            kind: ProcedureKind::Query,
            input: None,
            output: Some(RustType::simple("String")),
            source_file: PathBuf::from("api/ping.rs"),
            docs: None,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);

    assert!(output.contains("    ping: { input: void; output: string };"));
    assert!(!output.contains("export interface"));
}

#[test]
fn generates_complex_nested_types() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "search".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("String")),
            output: Some(RustType::with_generics(
                "Vec",
                vec![RustType::with_generics(
                    "Option",
                    vec![RustType::simple("Item")],
                )],
            )),
            source_file: PathBuf::from("api/search.rs"),
            docs: None,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("    search: { input: string; output: (Item | null)[] };"));
}

// --- enum codegen ---

#[test]
fn generates_unit_enum_as_string_union() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            variants: vec![
                EnumVariant {
                    name: "Active".to_string(),
                    kind: VariantKind::Unit,
                },
                EnumVariant {
                    name: "Inactive".to_string(),
                    kind: VariantKind::Unit,
                },
                EnumVariant {
                    name: "Banned".to_string(),
                    kind: VariantKind::Unit,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("export type Status = \"Active\" | \"Inactive\" | \"Banned\";"));
}

#[test]
fn generates_tuple_enum_as_tagged_union() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Response".to_string(),
            variants: vec![
                EnumVariant {
                    name: "Ok".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                },
                EnumVariant {
                    name: "Error".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("i32")]),
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("export type Response = { Ok: string } | { Error: number };"));
}

#[test]
fn generates_struct_enum_as_tagged_union() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    ("x".to_string(), RustType::simple("i32")),
                    ("y".to_string(), RustType::simple("i32")),
                ]),
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("export type Event = { Click: { x: number; y: number } };"));
}

#[test]
fn generates_mixed_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Shape".to_string(),
            variants: vec![
                EnumVariant {
                    name: "Circle".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("f64")]),
                },
                EnumVariant {
                    name: "Rect".to_string(),
                    kind: VariantKind::Struct(vec![
                        ("w".to_string(), RustType::simple("f64")),
                        ("h".to_string(), RustType::simple("f64")),
                    ]),
                },
                EnumVariant {
                    name: "Unknown".to_string(),
                    kind: VariantKind::Unit,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains(
        "export type Shape = { Circle: number } | { Rect: { w: number; h: number } } | \"Unknown\";"
    ));
}

#[test]
fn generates_empty_enum_as_never() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Empty".to_string(),
            variants: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("export type Empty = never;"));
}

#[test]
fn generates_multi_field_tuple_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Pair".to_string(),
            variants: vec![EnumVariant {
                name: "Both".to_string(),
                kind: VariantKind::Tuple(vec![RustType::simple("String"), RustType::simple("i32")]),
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("export type Pair = { Both: [string, number] };"));
}

// --- JSDoc tests ---

#[test]
fn test_jsdoc_on_struct() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            fields: vec![("x".to_string(), RustType::simple("i32"))],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("A foo struct.".to_string()),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve);
    assert!(output.contains("/** A foo struct. */\nexport interface Foo {"));
}

#[test]
fn test_jsdoc_on_struct_multiline() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Bar".to_string(),
            fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("Line one.\nLine two.".to_string()),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve);
    assert!(output.contains("/**\n * Line one.\n * Line two.\n */\nexport interface Bar {"));
}

#[test]
fn test_jsdoc_on_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            variants: vec![EnumVariant {
                name: "Active".to_string(),
                kind: VariantKind::Unit,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("Entity status.".to_string()),
        }],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve);
    assert!(output.contains("/** Entity status. */\nexport type Status ="));
}

#[test]
fn test_jsdoc_on_procedure() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "hello".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("String")),
            output: Some(RustType::simple("String")),
            source_file: PathBuf::from("api/hello.rs"),
            docs: Some("Say hello.".to_string()),
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve);
    assert!(
        output.contains("    /** Say hello. */\n    hello: { input: string; output: string };")
    );
}

#[test]
fn test_jsdoc_on_mutation_procedure() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "update".to_string(),
            kind: ProcedureKind::Mutation,
            input: Some(RustType::simple("String")),
            output: Some(RustType::simple("bool")),
            source_file: PathBuf::from("api/update.rs"),
            docs: Some("Update item.".to_string()),
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve);
    assert!(output
        .contains("    /** Update item. */\n    update: { input: string; output: boolean };"));
}

#[test]
fn test_no_jsdoc_when_disabled() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "hello".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("String")),
            output: Some(RustType::simple("String")),
            source_file: PathBuf::from("api/hello.rs"),
            docs: Some("Say hello.".to_string()),
        }],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("A foo.".to_string()),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(!output.contains("/**"));
}

// --- to_camel_case ---

#[test]
fn test_to_camel_case() {
    assert_eq!(to_camel_case("uptime_secs"), "uptimeSecs");
    assert_eq!(to_camel_case("user_id"), "userId");
    assert_eq!(to_camel_case("message"), "message");
    assert_eq!(to_camel_case("created_at_ms"), "createdAtMs");
    assert_eq!(to_camel_case(""), "");
}

// --- camelCase field naming ---

#[test]
fn test_camel_case_fields() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "ServerInfo".to_string(),
            fields: vec![
                ("uptime_secs".to_string(), RustType::simple("u64")),
                ("user_id".to_string(), RustType::simple("String")),
                ("message".to_string(), RustType::simple("String")),
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase);
    assert!(output.contains("  uptimeSecs: number;"));
    assert!(output.contains("  userId: string;"));
    assert!(output.contains("  message: string;"));
}

#[test]
fn test_camel_case_enum_struct_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    ("page_x".to_string(), RustType::simple("i32")),
                    ("page_y".to_string(), RustType::simple("i32")),
                ]),
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase);
    assert!(output.contains("{ Click: { pageX: number; pageY: number } }"));
}
