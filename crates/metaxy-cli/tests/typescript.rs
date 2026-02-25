mod common;

use std::path::PathBuf;

use metaxy_cli::codegen::typescript::{generate_types_file, rust_type_to_ts, to_camel_case};
use metaxy_cli::config::FieldNaming;
use metaxy_cli::model::*;

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
fn maps_hashset() {
    let ty = RustType::with_generics("HashSet", vec![RustType::simple("String")]);
    assert_eq!(rust_type_to_ts(&ty), "string[]");
}

#[test]
fn maps_btreeset() {
    let ty = RustType::with_generics("BTreeSet", vec![RustType::simple("i32")]);
    assert_eq!(rust_type_to_ts(&ty), "number[]");
}

#[test]
fn maps_box() {
    let ty = RustType::with_generics("Box", vec![RustType::simple("String")]);
    assert_eq!(rust_type_to_ts(&ty), "string");
}

#[test]
fn maps_arc() {
    let ty = RustType::with_generics("Arc", vec![RustType::simple("MyStruct")]);
    assert_eq!(rust_type_to_ts(&ty), "MyStruct");
}

#[test]
fn maps_rc() {
    let ty = RustType::with_generics("Rc", vec![RustType::simple("i32")]);
    assert_eq!(rust_type_to_ts(&ty), "number");
}

#[test]
fn maps_cow() {
    let ty = RustType::with_generics("Cow", vec![RustType::simple("str")]);
    assert_eq!(rust_type_to_ts(&ty), "string");
}

#[test]
fn maps_hashset_of_option() {
    let inner = RustType::with_generics("Option", vec![RustType::simple("i32")]);
    let ty = RustType::with_generics("HashSet", vec![inner]);
    assert_eq!(rust_type_to_ts(&ty), "(number | null)[]");
}

#[test]
fn maps_box_of_vec() {
    let inner = RustType::with_generics("Vec", vec![RustType::simple("String")]);
    let ty = RustType::with_generics("Box", vec![inner]);
    assert_eq!(rust_type_to_ts(&ty), "string[]");
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
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);

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
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);

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
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);

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
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
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
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Status = \"Active\" | \"Inactive\" | \"Banned\";"));
}

#[test]
fn generates_tuple_enum_as_tagged_union() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Response".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Response = { Ok: string } | { Error: number };"));
}

#[test]
fn generates_struct_enum_as_tagged_union() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Event = { Click: { x: number; y: number } };"));
}

#[test]
fn generates_mixed_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Shape".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
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
            generics: vec![],
            variants: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Empty = never;"));
}

#[test]
fn generates_multi_field_tuple_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Pair".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Both".to_string(),
                kind: VariantKind::Tuple(vec![RustType::simple("String"), RustType::simple("i32")]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Pair = { Both: [string, number] };"));
}

// --- JSDoc tests ---

#[test]
fn test_jsdoc_on_struct() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![field("x", RustType::simple("i32"))],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("A foo struct.".to_string()),
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, false);
    assert!(output.contains("/** A foo struct. */\nexport interface Foo {"));
}

#[test]
fn test_jsdoc_on_struct_multiline() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Bar".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("Line one.\nLine two.".to_string()),
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, false);
    assert!(output.contains("/**\n * Line one.\n * Line two.\n */\nexport interface Bar {"));
}

#[test]
fn test_jsdoc_on_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Active".to_string(),
                kind: VariantKind::Unit,
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("Entity status.".to_string()),
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, false);
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
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, false);
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
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, false);
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
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![StructDef {
            name: "Foo".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("A foo.".to_string()),
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
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
            generics: vec![],
            fields: vec![
                field("uptime_secs", RustType::simple("u64")),
                field("user_id", RustType::simple("String")),
                field("message", RustType::simple("String")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase, false);
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
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase, false);
    assert!(output.contains("{ Click: { pageX: number; pageY: number } }"));
}

// --- Serde codegen tests ---

#[test]
fn test_serde_rename_all_camel_case_on_struct() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserProfile".to_string(),
            generics: vec![],
            fields: vec![
                field("first_name", RustType::simple("String")),
                field("last_name", RustType::simple("String")),
                field("created_at", RustType::simple("u64")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::CamelCase),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
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
            generics: vec![],
            fields: vec![
                FieldDef {
                    name: "api_key".to_string(),
                    ty: RustType::simple("String"),
                    rename: Some("API_KEY".to_string()),
                    skip: false,
                    has_default: false,
                    flatten: false,
                },
                field("host_name", RustType::simple("String")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::CamelCase),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("  API_KEY: string;"));
    assert!(output.contains("  hostName: string;"));
}

#[test]
fn test_serde_skip_field_omitted() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Session".to_string(),
            generics: vec![],
            fields: vec![
                field("token", RustType::simple("String")),
                FieldDef {
                    name: "internal_id".to_string(),
                    ty: RustType::simple("u64"),
                    rename: None,
                    skip: true,
                    has_default: false,
                    flatten: false,
                },
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("  token: string;"));
    assert!(!output.contains("internal_id"));
}

#[test]
fn test_serde_default_option_field() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Params".to_string(),
            generics: vec![],
            fields: vec![
                field("required", RustType::simple("String")),
                FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                    flatten: false,
                },
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
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
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type EventKind = \"user_login\" | \"user_logout\";"));
}

#[test]
fn test_serde_variant_rename_override() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
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
            generics: vec![],
            fields: vec![field("my_field", RustType::simple("String"))],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::ScreamingSnakeCase),
        }],
        enums: vec![],
    };
    // Even with CamelCase config, serde rename_all wins
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase, false);
    assert!(output.contains("  MY_FIELD: string;"));
}

#[test]
fn test_serde_default_on_non_option_field_is_not_optional() {
    // `#[serde(default)]` on a non-Option field should NOT produce `?:` syntax.
    // Only `default + Option<T>` makes a field optional in TypeScript.
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Config".to_string(),
            generics: vec![],
            fields: vec![FieldDef {
                name: "retries".to_string(),
                ty: RustType::simple("u32"),
                rename: None,
                skip: false,
                has_default: true,
                flatten: false,
            }],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("  retries: number;"));
    assert!(!output.contains("retries?"));
}

// --- insta snapshot tests ---

#[test]
fn snapshot_complete_types() {
    let manifest = common::make_test_manifest();
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_empty_manifest() {
    let manifest = Manifest::default();
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_types_with_jsdoc() {
    let mut manifest = common::make_test_manifest();
    for s in &mut manifest.structs {
        s.docs = Some(format!("Documentation for {}.", s.name));
    }
    for p in &mut manifest.procedures {
        p.docs = Some(format!("Documentation for {}.", p.name));
    }
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_types_camel_case() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "ServerInfo".to_string(),
            generics: vec![],
            fields: vec![
                field("uptime_secs", RustType::simple("u64")),
                field("user_id", RustType::simple("String")),
                field("is_active", RustType::simple("bool")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::CamelCase, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_enum_unit() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_enum_mixed() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Shape".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_serde_rename_all() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserProfile".to_string(),
            generics: vec![],
            fields: vec![
                field("first_name", RustType::simple("String")),
                field("last_name", RustType::simple("String")),
                field("created_at", RustType::simple("u64")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::CamelCase),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_serde_skip_and_default() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Config".to_string(),
            generics: vec![],
            fields: vec![
                field("name", RustType::simple("String")),
                FieldDef {
                    name: "internal".to_string(),
                    ty: RustType::simple("u64"),
                    rename: None,
                    skip: true,
                    has_default: false,
                    flatten: false,
                },
                FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                    flatten: false,
                },
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

// --- Internal tagging codegen tests ---

#[test]
fn internal_tag_struct_variants() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Shape".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Circle".to_string(),
                    kind: VariantKind::Struct(vec![field("radius", RustType::simple("f64"))]),
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
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains(
        "export type Shape = { type: \"Circle\"; radius: number } | { type: \"Rect\"; w: number; h: number };"
    ));
}

#[test]
fn internal_tag_unit_variants() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Status".to_string(),
            generics: vec![],
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
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Status = { type: \"Active\" } | { type: \"Inactive\" };"));
}

#[test]
fn internal_tag_mixed_unit_struct() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Action".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Noop".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "Move".to_string(),
                    kind: VariantKind::Struct(vec![
                        field("x", RustType::simple("i32")),
                        field("y", RustType::simple("i32")),
                    ]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Internal {
                tag: "kind".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains(
        "export type Action = { kind: \"Noop\" } | { kind: \"Move\"; x: number; y: number };"
    ));
}

#[test]
fn internal_tag_newtype_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Wrapper".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Data".to_string(),
                kind: VariantKind::Tuple(vec![RustType::simple("Payload")]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Wrapper = { type: \"Data\" } & Payload;"));
}

// --- Adjacent tagging codegen tests ---

#[test]
fn adjacent_tag_struct_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Event = { t: \"Click\"; c: { x: number; y: number } };"));
}

#[test]
fn adjacent_tag_tuple_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Msg".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Scroll".to_string(),
                kind: VariantKind::Tuple(vec![RustType::simple("f64")]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Msg = { t: \"Scroll\"; c: number };"));
}

#[test]
fn adjacent_tag_unit_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Signal".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Empty".to_string(),
                kind: VariantKind::Unit,
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Signal = { t: \"Empty\" };"));
}

#[test]
fn adjacent_tag_mixed() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Cmd".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Noop".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "Set".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
                EnumVariant {
                    name: "Move".to_string(),
                    kind: VariantKind::Struct(vec![
                        field("x", RustType::simple("i32")),
                        field("y", RustType::simple("i32")),
                    ]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains(
        "export type Cmd = { t: \"Noop\" } | { t: \"Set\"; c: string } | { t: \"Move\"; c: { x: number; y: number } };"
    ));
}

// --- Untagged codegen tests ---

#[test]
fn untagged_tuple_variants() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Value".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Str".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
                EnumVariant {
                    name: "Num".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("f64")]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Value = string | number;"));
}

#[test]
fn untagged_struct_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Wrapper".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Data".to_string(),
                kind: VariantKind::Struct(vec![field("value", RustType::simple("String"))]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Wrapper = { value: string };"));
}

#[test]
fn untagged_unit_variant() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Maybe".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Nothing".to_string(),
                kind: VariantKind::Unit,
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Maybe = null;"));
}

#[test]
fn untagged_mixed() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Input".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "None".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "Text".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
                EnumVariant {
                    name: "Form".to_string(),
                    kind: VariantKind::Struct(vec![field("field", RustType::simple("String"))]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Input = null | string | { field: string };"));
}

// --- Tagging + rename_all tests ---

#[test]
fn internal_tag_with_rename_all() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
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
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(
        output
            .contains("export type Event = { type: \"user_login\" } | { type: \"user_logout\" };")
    );
}

#[test]
fn adjacent_tag_with_rename_all() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Msg".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "UserLogin".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
                EnumVariant {
                    name: "SystemError".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::KebabCase),
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(
        output.contains(
            "export type Msg = { t: \"user-login\"; c: string } | { t: \"system-error\" };"
        )
    );
}

// --- Optional fields in enum struct variants ---

#[test]
fn external_struct_variant_optional_field() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "E".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "V".to_string(),
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                    flatten: false,
                }]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ V: { label?: string | null } }"));
}

#[test]
fn internal_struct_variant_optional_field() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "E".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "V".to_string(),
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                    flatten: false,
                }]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ type: \"V\"; label?: string | null }"));
}

#[test]
fn adjacent_struct_variant_optional_field() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "E".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "V".to_string(),
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                    flatten: false,
                }]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ t: \"V\"; c: { label?: string | null } }"));
}

#[test]
fn untagged_struct_variant_optional_field() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "E".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "V".to_string(),
                kind: VariantKind::Struct(vec![FieldDef {
                    name: "label".to_string(),
                    ty: RustType::with_generics("Option", vec![RustType::simple("String")]),
                    rename: None,
                    skip: false,
                    has_default: true,
                    flatten: false,
                }]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ label?: string | null }"));
}

// --- Tagging snapshot tests ---

#[test]
fn snapshot_internal_tagged() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Shape".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Circle".to_string(),
                    kind: VariantKind::Struct(vec![field("radius", RustType::simple("f64"))]),
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
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_adjacent_tagged() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Message".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Text".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
                EnumVariant {
                    name: "Binary".to_string(),
                    kind: VariantKind::Tuple(vec![
                        RustType::simple("String"),
                        RustType::simple("u32"),
                    ]),
                    rename: None,
                },
                EnumVariant {
                    name: "Ping".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "Data".to_string(),
                    kind: VariantKind::Struct(vec![
                        field("key", RustType::simple("String")),
                        field("value", RustType::simple("String")),
                    ]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_untagged() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Value".to_string(),
            generics: vec![],
            variants: vec![
                EnumVariant {
                    name: "Null".to_string(),
                    kind: VariantKind::Unit,
                    rename: None,
                },
                EnumVariant {
                    name: "Text".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
                EnumVariant {
                    name: "Pair".to_string(),
                    kind: VariantKind::Tuple(vec![
                        RustType::simple("String"),
                        RustType::simple("i32"),
                    ]),
                    rename: None,
                },
                EnumVariant {
                    name: "Record".to_string(),
                    kind: VariantKind::Struct(vec![
                        field("id", RustType::simple("u64")),
                        field("name", RustType::simple("String")),
                    ]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

// --- Generic struct/enum codegen tests ---

#[test]
fn generates_generic_interface() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Paginated".to_string(),
            generics: vec!["T".to_string()],
            fields: vec![
                field(
                    "items",
                    RustType::with_generics("Vec", vec![RustType::simple("T")]),
                ),
                field("total", RustType::simple("u64")),
                field("page", RustType::simple("u32")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export interface Paginated<T> {"));
    assert!(output.contains("  items: T[];"));
    assert!(output.contains("  total: number;"));
    assert!(output.contains("  page: number;"));
}

#[test]
fn generates_multi_param_interface() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Pair".to_string(),
            generics: vec!["A".to_string(), "B".to_string()],
            fields: vec![
                field("first", RustType::simple("A")),
                field("second", RustType::simple("B")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export interface Pair<A, B> {"));
}

#[test]
fn maps_user_defined_with_generics() {
    let ty = RustType::with_generics("Paginated", vec![RustType::simple("User")]);
    assert_eq!(rust_type_to_ts(&ty), "Paginated<User>");
}

#[test]
fn maps_user_defined_nested_generics() {
    let ty = RustType::with_generics(
        "Paginated",
        vec![RustType::with_generics(
            "Vec",
            vec![RustType::simple("User")],
        )],
    );
    assert_eq!(rust_type_to_ts(&ty), "Paginated<User[]>");
}

#[test]
fn procedure_output_preserves_generics() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "list_users".to_string(),
            kind: ProcedureKind::Query,
            input: None,
            output: Some(RustType::with_generics(
                "Paginated",
                vec![RustType::simple("User")],
            )),
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("output: Paginated<User>"));
}

#[test]
fn generates_generic_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Response".to_string(),
            generics: vec!["T".to_string()],
            variants: vec![
                EnumVariant {
                    name: "Ok".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("T")]),
                    rename: None,
                },
                EnumVariant {
                    name: "Error".to_string(),
                    kind: VariantKind::Tuple(vec![RustType::simple("String")]),
                    rename: None,
                },
            ],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Response<T> = { Ok: T } | { Error: string };"));
}

#[test]
fn snapshot_generic_struct() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "list_users".to_string(),
            kind: ProcedureKind::Query,
            input: None,
            output: Some(RustType::with_generics(
                "Paginated",
                vec![RustType::simple("User")],
            )),
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![
            StructDef {
                name: "Paginated".to_string(),
                generics: vec!["T".to_string()],
                fields: vec![
                    field(
                        "items",
                        RustType::with_generics("Vec", vec![RustType::simple("T")]),
                    ),
                    field("total", RustType::simple("u64")),
                    field("page", RustType::simple("u32")),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "User".to_string(),
                generics: vec![],
                fields: vec![
                    field("id", RustType::simple("u64")),
                    field("name", RustType::simple("String")),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
        ],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

// --- Newtype / Tuple struct codegen tests ---

#[test]
fn generates_newtype_alias() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserId".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("String")],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type UserId = string;"));
    assert!(!output.contains("export interface"));
}

#[test]
fn generates_branded_newtype() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserId".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("String")],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, true);
    assert!(output.contains("export type UserId = string & { readonly __brand: \"UserId\" };"));
}

#[test]
fn generates_multi_field_tuple_type() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Pair".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("String"), RustType::simple("i32")],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Pair = [string, number];"));
}

#[test]
fn generates_generic_branded_newtype() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Wrapper".to_string(),
            generics: vec!["T".to_string()],
            fields: vec![],
            tuple_fields: vec![RustType::simple("T")],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, true);
    assert!(output.contains("export type Wrapper<T> = T & { readonly __brand: \"Wrapper\" };"));
}

#[test]
fn procedure_uses_newtype() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "get_user".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("UserId")),
            output: Some(RustType::simple("User")),
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![StructDef {
            name: "UserId".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("String")],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, true);
    assert!(output.contains("export type UserId = string & { readonly __brand: \"UserId\" };"));
    assert!(output.contains("input: UserId; output: User"));
}

#[test]
fn generates_newtype_with_jsdoc() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "UserId".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("String")],
            source_file: PathBuf::from("api/test.rs"),
            docs: Some("A unique user identifier.".to_string()),
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, true, FieldNaming::Preserve, true);
    assert!(output.contains("/** A unique user identifier. */\nexport type UserId = string & { readonly __brand: \"UserId\" };"));
}

#[test]
fn multi_field_tuple_ignores_branded_flag() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Pair".to_string(),
            generics: vec![],
            fields: vec![],
            tuple_fields: vec![RustType::simple("String"), RustType::simple("i32")],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    // Branded flag should not affect multi-field tuple structs
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, true);
    assert!(output.contains("export type Pair = [string, number];"));
    assert!(!output.contains("__brand"));
}

#[test]
fn snapshot_branded_newtypes() {
    let manifest = Manifest {
        procedures: vec![Procedure {
            name: "get_user".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("UserId")),
            output: Some(RustType::simple("User")),
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![
            StructDef {
                name: "UserId".to_string(),
                generics: vec![],
                fields: vec![],
                tuple_fields: vec![RustType::simple("String")],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "User".to_string(),
                generics: vec![],
                fields: vec![
                    field("id", RustType::simple("u64")),
                    field("name", RustType::simple("String")),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "Coords".to_string(),
                generics: vec![],
                fields: vec![],
                tuple_fields: vec![RustType::simple("f64"), RustType::simple("f64")],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
        ],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, true);
    insta::assert_snapshot!(output);
}

// --- Flatten codegen tests ---

fn flatten_field(name: &str, ty: RustType) -> FieldDef {
    FieldDef {
        name: name.to_string(),
        ty,
        rename: None,
        skip: false,
        has_default: false,
        flatten: true,
    }
}

#[test]
fn generates_struct_with_flatten() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Full".to_string(),
            generics: vec![],
            fields: vec![
                field("id", RustType::simple("u64")),
                flatten_field("meta", RustType::simple("Metadata")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Full = { id: number } & Metadata;"));
}

#[test]
fn generates_multiple_flatten() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Combined".to_string(),
            generics: vec![],
            fields: vec![
                field("id", RustType::simple("u64")),
                flatten_field("a", RustType::simple("A")),
                flatten_field("b", RustType::simple("B")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Combined = { id: number } & A & B;"));
}

#[test]
fn flatten_only_no_regular_fields() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Merged".to_string(),
            generics: vec![],
            fields: vec![
                flatten_field("a", RustType::simple("A")),
                flatten_field("b", RustType::simple("B")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Merged = A & B;"));
}

#[test]
fn flatten_with_rename_all() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Data".to_string(),
            generics: vec![],
            fields: vec![
                field("my_field", RustType::simple("String")),
                flatten_field("extra", RustType::simple("Extra")),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: Some(RenameRule::CamelCase),
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    // rename_all applies to regular fields only; flattened type name is unchanged
    assert!(output.contains("export type Data = { myField: string } & Extra;"));
}

#[test]
fn flatten_skipped_field_omitted() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![StructDef {
            name: "Data".to_string(),
            generics: vec![],
            fields: vec![
                field("id", RustType::simple("u64")),
                FieldDef {
                    name: "hidden".to_string(),
                    ty: RustType::simple("Secret"),
                    rename: None,
                    skip: true,
                    has_default: false,
                    flatten: true,
                },
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    // flatten + skip → omit; no flatten, so standard interface
    assert!(output.contains("export interface Data {"));
    assert!(output.contains("  id: number;"));
    assert!(!output.contains("Secret"));
}

#[test]
fn flatten_in_external_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    field("x", RustType::simple("i32")),
                    flatten_field("meta", RustType::simple("Meta")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ Click: { x: number } & Meta }"));
}

#[test]
fn flatten_in_internal_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    field("x", RustType::simple("i32")),
                    flatten_field("meta", RustType::simple("Meta")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Internal {
                tag: "type".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ type: \"Click\"; x: number } & Meta"));
}

#[test]
fn flatten_in_adjacent_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    field("x", RustType::simple("i32")),
                    flatten_field("meta", RustType::simple("Meta")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Adjacent {
                tag: "t".to_string(),
                content: "c".to_string(),
            },
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("{ t: \"Click\"; c: { x: number } & Meta }"));
}

#[test]
fn flatten_in_untagged_enum() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    field("x", RustType::simple("i32")),
                    flatten_field("meta", RustType::simple("Meta")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::Untagged,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    assert!(output.contains("export type Event = { x: number } & Meta;"));
}

#[test]
fn snapshot_flatten() {
    let manifest = Manifest {
        procedures: vec![],
        structs: vec![
            StructDef {
                name: "Full".to_string(),
                generics: vec![],
                fields: vec![
                    field("id", RustType::simple("u64")),
                    flatten_field("meta", RustType::simple("Metadata")),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "Merged".to_string(),
                generics: vec![],
                fields: vec![
                    flatten_field("a", RustType::simple("A")),
                    flatten_field("b", RustType::simple("B")),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/test.rs"),
                docs: None,
                rename_all: None,
            },
        ],
        enums: vec![EnumDef {
            name: "Event".to_string(),
            generics: vec![],
            variants: vec![EnumVariant {
                name: "Click".to_string(),
                kind: VariantKind::Struct(vec![
                    field("x", RustType::simple("i32")),
                    flatten_field("meta", RustType::simple("Meta")),
                ]),
                rename: None,
            }],
            source_file: PathBuf::from("api/test.rs"),
            docs: None,
            rename_all: None,
            tagging: EnumTagging::External,
        }],
    };
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

// --- Type overrides (end-to-end via generate_types_file) ---

#[test]
fn snapshot_type_overrides() {
    use metaxy_cli::codegen::overrides::{apply_type_overrides, build_base_index};
    use std::collections::HashMap;

    let overrides: HashMap<String, String> = [
        ("chrono::DateTime".to_string(), "string".to_string()),
        ("uuid::Uuid".to_string(), "string".to_string()),
        ("serde_json::Value".to_string(), "unknown".to_string()),
    ]
    .into_iter()
    .collect();
    let base_index = build_base_index(&overrides);

    let mut manifest = Manifest {
        procedures: vec![
            Procedure {
                name: "get_user".to_string(),
                kind: ProcedureKind::Query,
                input: Some(RustType::simple("Uuid")),
                output: Some(RustType::simple("User")),
                source_file: PathBuf::from("api/user.rs"),
                docs: None,
                timeout_ms: None,
                idempotent: false,
            },
            Procedure {
                name: "create_event".to_string(),
                kind: ProcedureKind::Mutation,
                input: Some(RustType::simple("EventInput")),
                output: Some(RustType::simple("Event")),
                source_file: PathBuf::from("api/event.rs"),
                docs: None,
                timeout_ms: None,
                idempotent: false,
            },
        ],
        structs: vec![
            StructDef {
                name: "User".to_string(),
                generics: vec![],
                fields: vec![
                    field("id", RustType::simple("Uuid")),
                    field("name", RustType::simple("String")),
                    field(
                        "created_at",
                        RustType::with_generics("DateTime", vec![RustType::simple("Utc")]),
                    ),
                    field("metadata", RustType::simple("Value")),
                    field(
                        "tags",
                        RustType::with_generics("Vec", vec![RustType::simple("String")]),
                    ),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/user.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "EventInput".to_string(),
                generics: vec![],
                fields: vec![
                    field("title", RustType::simple("String")),
                    field(
                        "scheduled_at",
                        RustType::with_generics("Option", vec![RustType::simple("DateTime")]),
                    ),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/event.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "Event".to_string(),
                generics: vec![],
                fields: vec![
                    field("id", RustType::simple("Uuid")),
                    field("title", RustType::simple("String")),
                ],
                tuple_fields: vec![],
                source_file: PathBuf::from("api/event.rs"),
                docs: None,
                rename_all: None,
            },
        ],
        enums: vec![],
    };

    apply_type_overrides(&mut manifest, &overrides, &base_index);
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}

// --- rust_type_to_ts with qualified paths ---

#[test]
fn maps_qualified_path_to_base_name() {
    let ty = RustType::simple("chrono::DateTime");
    assert_eq!(rust_type_to_ts(&ty), "DateTime");
}

#[test]
fn maps_qualified_path_with_generics() {
    let ty = RustType::with_generics("chrono::DateTime", vec![RustType::simple("Utc")]);
    assert_eq!(rust_type_to_ts(&ty), "DateTime<Utc>");
}

// --- RustType::base_name ---

#[test]
fn base_name_simple() {
    assert_eq!(RustType::simple("String").base_name(), "String");
}

#[test]
fn base_name_qualified() {
    assert_eq!(RustType::simple("chrono::DateTime").base_name(), "DateTime");
}

#[test]
fn base_name_deeply_qualified() {
    assert_eq!(
        RustType::simple("serde_json::value::Value").base_name(),
        "Value"
    );
}

// --- BigInt types ---

#[test]
fn snapshot_bigint_types() {
    use metaxy_cli::codegen::overrides::{apply_type_overrides, build_base_index};
    use std::collections::HashMap;

    // Simulate bigint_types = ["i64", "u64"] merged into effective overrides
    let mut effective_overrides = HashMap::new();
    for ty in ["i64", "u64"] {
        effective_overrides
            .entry(ty.to_string())
            .or_insert_with(|| "bigint".to_string());
    }
    let base_index = build_base_index(&effective_overrides);

    let mut manifest = Manifest {
        procedures: vec![Procedure {
            name: "get_stats".to_string(),
            kind: ProcedureKind::Query,
            input: None,
            output: Some(RustType::simple("Stats")),
            source_file: PathBuf::from("api/stats.rs"),
            docs: None,
            timeout_ms: None,
            idempotent: false,
        }],
        structs: vec![StructDef {
            name: "Stats".to_string(),
            generics: vec![],
            fields: vec![
                field("count", RustType::simple("u32")),
                field("total_bytes", RustType::simple("u64")),
                field("max_value", RustType::simple("i64")),
                field(
                    "ids",
                    RustType::with_generics("Vec", vec![RustType::simple("u64")]),
                ),
                field(
                    "optional_big",
                    RustType::with_generics("Option", vec![RustType::simple("i64")]),
                ),
            ],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/stats.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };

    apply_type_overrides(&mut manifest, &effective_overrides, &base_index);
    let output = generate_types_file(&manifest, false, FieldNaming::Preserve, false);
    insta::assert_snapshot!(output);
}
