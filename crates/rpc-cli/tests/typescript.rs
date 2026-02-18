mod common;

use std::path::PathBuf;

use vercel_rpc_cli::codegen::typescript::{generate_types_file, rust_type_to_ts, to_camel_case};
use vercel_rpc_cli::config::FieldNaming;
use vercel_rpc_cli::model::*;

use common::field;

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
                    rename: None,
                },
                EnumVariant {
                    name: "Inactive".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "Banned".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
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
                    rename: None,
                },
                EnumVariant {
                    name: "Error".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("i32")]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
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
                    field("x", RustType::simple("i32")),
                    field("y", RustType::simple("i32")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
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
                    rename: None,
                },
                EnumVariant {
                    name: "Rect".to_string(),
                    kind: VariantKind::Struct(vec![
                        field("w", RustType::simple("f64")),
                        field("h", RustType::simple("f64")),
                    ]),
                    rename: None,
                },
                EnumVariant {
                    name: "Unknown".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
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
            rename_all: None,
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
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
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
            fields: vec![field("x", RustType::simple("i32"))],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("A foo struct.".to_string()),
            rename_all: None,
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
            rename_all: None,
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
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("Entity status.".to_string()),
            rename_all: None,
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
    assert!(
        output.contains("    /** Update item. */\n    update: { input: string; output: boolean };")
    );
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
            rename_all: None,
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
                field("uptime_secs", RustType::simple("u64")),
                field("user_id", RustType::simple("String")),
                field("message", RustType::simple("String")),
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
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
                    field("page_x", RustType::simple("i32")),
                    field("page_y", RustType::simple("i32")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase);
    assert!(output.contains("{ Click: { pageX: number; pageY: number } }"));
}

// --- Serde codegen tests ---

#[test]
fn test_serde_rename_all_camel_case_on_struct() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserProfile".to_string(),
            fields: vec![
                field("first_name", RustType::simple("String")),
                field("last_name", RustType::simple("String")),
                field("created_at", RustType::simple("u64")),
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::CamelCase),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("  firstName: string;"));
    assert!(output.contains("  lastName: string;"));
    assert!(output.contains("  createdAt: number;"));
}

#[test]
fn test_serde_field_rename_overrides_rename_all() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Config".to_string(),
            fields: vec![
                FieldDef {
                    name: "api_key".to_string(),
                    ty: RustType::simple("String"),
                    rename: Some("API_KEY".to_string()),
                    skip: false,
                    has_default: false,
                },
                field("host_name", RustType::simple("String")),
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::CamelCase),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("  API_KEY: string;"));
    assert!(output.contains("  hostName: string;"));
}

#[test]
fn test_serde_skip_field_omitted() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Session".to_string(),
            fields: vec![
                field("token", RustType::simple("String")),
                FieldDef {
                    name: "internal_id".to_string(),
                    ty: RustType::simple("u64"),
                    rename: None,
                    skip: true,
                    has_default: false,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("  token: string;"));
    assert!(!output.contains("internal_id"));
}

#[test]
fn test_serde_default_option_field() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Params".to_string(),
            fields: vec![
                field("required", RustType::simple("String")),
                FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("  required: string;"));
    assert!(output.contains("  label?: string | null;"));
}

#[test]
fn test_serde_rename_all_on_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "EventKind".to_string(),
            variants: vec![
                EnumVariant {
                    name: "UserLogin".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "UserLogout".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::SnakeCase),
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    assert!(output.contains("export type EventKind = \"user_login\" | \"user_logout\";"));
}

#[test]
fn test_serde_variant_rename_override() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            variants: vec![
                EnumVariant {
                    name: "Active".to_string(),
                    kind: VariantKind::Unit,
                    rename: Some("enabled".to_string()),
                },
                EnumVariant {
                    name: "Inactive".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::SnakeCase),
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve);
    // rename overrides rename_all for the first variant
    assert!(output.contains("\"enabled\""));
    // rename_all applies to the second variant
    assert!(output.contains("\"inactive\""));
}

#[test]
fn test_serde_rename_all_takes_priority_over_config_naming() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Data".to_string(),
            fields: vec![field("my_field", RustType::simple("String"))],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::ScreamingSnakeCase),
        }],
        enums: vec![],
    };
    // Even with CamelCase config, serde rename_all wins
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase);
    assert!(output.contains("  MY_FIELD: string;"));
}
