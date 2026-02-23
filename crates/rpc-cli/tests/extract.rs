mod common;

use tempfile::TempDir;

use vercel_rpc_cli::config::InputConfig;
use vercel_rpc_cli::model::*;
use vercel_rpc_cli::parser::extract::scan_directory;

#[test]
fn extracts_query_no_input() {
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    assert_eq!(manifest.structs[0].fields[0].name, "name");
}

#[test]
fn ignores_non_rpc_functions() {
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
            assert_eq!(fields[0].name, "x");
            assert_eq!(fields[1].name, "y");
        }
        _ => panic!("expected Struct variant"),
    }
}

#[test]
fn extracts_mixed_enum() {
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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

#[test]
fn test_include_filters_files() {
    let tmp = TempDir::new().unwrap();
    common::write_rpc_file(tmp.path(), "handlers/hello.rs");
    common::write_rpc_file(tmp.path(), "utils/helper.rs");

    let input = InputConfig {
        dir: tmp.path().to_path_buf(),
        include: vec!["handlers/**/*.rs".into()],
        exclude: vec![],
    };

    let manifest = scan_directory(&input).unwrap();
    assert_eq!(manifest.procedures.len(), 1);
    assert_eq!(manifest.procedures[0].name, "handler");
    // The file from utils/ should not appear
    assert!(
        manifest
            .procedures
            .iter()
            .all(|p| { p.source_file.to_string_lossy().contains("handlers") })
    );
}

#[test]
fn test_exclude_filters_files() {
    let tmp = TempDir::new().unwrap();
    common::write_rpc_file(tmp.path(), "hello.rs");
    common::write_rpc_file(tmp.path(), "test_hello.rs");

    let input = InputConfig {
        dir: tmp.path().to_path_buf(),
        include: vec!["**/*.rs".into()],
        exclude: vec!["test_*.rs".into()],
    };

    let manifest = scan_directory(&input).unwrap();
    assert_eq!(manifest.procedures.len(), 1);
    assert!(
        manifest.procedures[0]
            .source_file
            .to_string_lossy()
            .contains("hello.rs")
    );
    assert!(
        !manifest.procedures[0]
            .source_file
            .to_string_lossy()
            .contains("test_hello.rs")
    );
}

#[test]
fn test_extracts_doc_comments() {
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
    let manifest = common::parse_source(
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
fn test_doc_hidden_ignored() {
    let manifest = common::parse_source(
        r#"
            #[doc(hidden)]
            #[rpc_query]
            async fn internal() -> String {
                "ok".to_string()
            }
            "#,
    );
    assert_eq!(manifest.procedures.len(), 1);
    // #[doc(hidden)] is not a NameValue meta, so no docs extracted
    assert!(manifest.procedures[0].docs.is_none());
}

#[test]
fn test_no_doc_comments_returns_none() {
    let manifest = common::parse_source(
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
fn test_rpc_function_no_return_type() {
    let manifest = common::parse_source(
        r#"
            #[rpc_query]
            async fn fire_and_forget() {}
            "#,
    );
    assert_eq!(manifest.procedures.len(), 1);
    let proc = &manifest.procedures[0];
    assert_eq!(proc.name, "fire_and_forget");
    assert!(proc.output.is_none());
}

#[test]
fn test_serde_path_derive() {
    let manifest = common::parse_source(
        r#"
            #[derive(serde::Serialize)]
            struct PathDerived {
                value: String,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert_eq!(manifest.structs[0].name, "PathDerived");
}

#[test]
fn test_exclude_wins_over_include() {
    let tmp = TempDir::new().unwrap();
    common::write_rpc_file(tmp.path(), "hello.rs");
    common::write_rpc_file(tmp.path(), "world.rs");

    let input = InputConfig {
        dir: tmp.path().to_path_buf(),
        include: vec!["**/*.rs".into()],
        exclude: vec!["hello.rs".into()],
    };

    let manifest = scan_directory(&input).unwrap();
    assert_eq!(manifest.procedures.len(), 1);
    assert!(
        manifest.procedures[0]
            .source_file
            .to_string_lossy()
            .contains("world.rs")
    );
}

// --- Serde attribute extraction tests ---

#[test]
fn extracts_struct_rename_all() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            #[serde(rename_all = "camelCase")]
            struct UserProfile {
                first_name: String,
                last_name: String,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert_eq!(manifest.structs[0].rename_all, Some(RenameRule::CamelCase),);
}

#[test]
fn extracts_field_rename() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Config {
                #[serde(rename = "apiKey")]
                api_key: String,
                host: String,
            }
            "#,
    );
    let fields = &manifest.structs[0].fields;
    assert_eq!(fields[0].rename.as_deref(), Some("apiKey"));
    assert_eq!(fields[1].rename, None);
}

#[test]
fn extracts_field_skip() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Session {
                token: String,
                #[serde(skip)]
                internal_id: u64,
            }
            "#,
    );
    let fields = &manifest.structs[0].fields;
    assert!(!fields[0].skip);
    assert!(fields[1].skip);
}

#[test]
fn extracts_field_skip_serializing() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Data {
                visible: String,
                #[serde(skip_serializing)]
                secret: String,
            }
            "#,
    );
    assert!(manifest.structs[0].fields[1].skip);
}

#[test]
fn extracts_field_default() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Options {
                #[serde(default)]
                verbose: bool,
                name: String,
            }
            "#,
    );
    let fields = &manifest.structs[0].fields;
    assert!(fields[0].has_default);
    assert!(!fields[1].has_default);
}

#[test]
fn extracts_enum_rename_all() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            #[serde(rename_all = "snake_case")]
            enum EventKind {
                UserLogin,
                UserLogout,
            }
            "#,
    );
    assert_eq!(manifest.enums[0].rename_all, Some(RenameRule::SnakeCase));
}

#[test]
fn extracts_variant_rename() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            enum Status {
                #[serde(rename = "ok")]
                Active,
                Inactive,
            }
            "#,
    );
    assert_eq!(manifest.enums[0].variants[0].rename.as_deref(), Some("ok"));
    assert_eq!(manifest.enums[0].variants[1].rename, None);
}

#[test]
fn no_serde_attrs_returns_defaults() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Plain {
                value: String,
            }
            "#,
    );
    let s = &manifest.structs[0];
    assert_eq!(s.rename_all, None);
    assert_eq!(s.fields[0].rename, None);
    assert!(!s.fields[0].skip);
    assert!(!s.fields[0].has_default);
}

// --- Enum tagging extraction tests ---

#[test]
fn parse_internally_tagged_enum() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            #[serde(tag = "type")]
            enum Shape {
                Circle { radius: f64 },
                Rect { w: f64, h: f64 },
            }
            "#,
    );
    assert_eq!(manifest.enums.len(), 1);
    assert_eq!(
        manifest.enums[0].tagging,
        EnumTagging::Internal {
            tag: "type".to_string()
        },
    );
}

#[test]
fn parse_adjacently_tagged_enum() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            #[serde(tag = "t", content = "c")]
            enum Message {
                Text(String),
                Ping,
            }
            "#,
    );
    assert_eq!(manifest.enums.len(), 1);
    assert_eq!(
        manifest.enums[0].tagging,
        EnumTagging::Adjacent {
            tag: "t".to_string(),
            content: "c".to_string(),
        },
    );
}

#[test]
fn parse_untagged_enum() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            #[serde(untagged)]
            enum Value {
                Str(String),
                Num(f64),
            }
            "#,
    );
    assert_eq!(manifest.enums.len(), 1);
    assert_eq!(manifest.enums[0].tagging, EnumTagging::Untagged);
}

#[test]
fn parse_default_external_enum() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            enum Status {
                Active,
                Inactive,
            }
            "#,
    );
    assert_eq!(manifest.enums.len(), 1);
    assert_eq!(manifest.enums[0].tagging, EnumTagging::External);
}

// --- Generic extraction tests ---

#[test]
fn extracts_generic_struct() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Paginated<T> {
                items: Vec<T>,
                total: u64,
                page: u32,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert_eq!(manifest.structs[0].name, "Paginated");
    assert_eq!(manifest.structs[0].generics, vec!["T".to_string()]);
}

#[test]
fn extracts_multi_generic_struct() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Pair<A, B> {
                first: A,
                second: B,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert_eq!(
        manifest.structs[0].generics,
        vec!["A".to_string(), "B".to_string()]
    );
}

#[test]
fn extracts_non_generic_struct_has_empty_generics() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Plain {
                value: String,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert!(manifest.structs[0].generics.is_empty());
}

#[test]
fn extracts_generic_enum() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            enum Response<T> {
                Ok(T),
                Error(String),
            }
            "#,
    );
    assert_eq!(manifest.enums.len(), 1);
    assert_eq!(manifest.enums[0].generics, vec!["T".to_string()]);
}

#[test]
fn extracts_generic_struct_skips_lifetimes() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Borrowed<'a, T> {
                data: &'a T,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert_eq!(manifest.structs[0].generics, vec!["T".to_string()]);
}

// --- Tuple struct extraction tests ---

#[test]
fn extracts_newtype_tuple_field() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct UserId(String);
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    let s = &manifest.structs[0];
    assert_eq!(s.name, "UserId");
    assert!(s.fields.is_empty());
    assert_eq!(s.tuple_fields.len(), 1);
    assert_eq!(s.tuple_fields[0], RustType::simple("String"));
}

#[test]
fn extracts_multi_field_tuple_struct() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Pair(String, i32);
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    let s = &manifest.structs[0];
    assert!(s.fields.is_empty());
    assert_eq!(s.tuple_fields.len(), 2);
    assert_eq!(s.tuple_fields[0], RustType::simple("String"));
    assert_eq!(s.tuple_fields[1], RustType::simple("i32"));
}

#[test]
fn named_struct_has_empty_tuple_fields() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Plain {
                value: String,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    assert!(manifest.structs[0].tuple_fields.is_empty());
    assert_eq!(manifest.structs[0].fields.len(), 1);
}

#[test]
fn extracts_generic_tuple_struct() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Wrapper<T>(T);
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    let s = &manifest.structs[0];
    assert_eq!(s.generics, vec!["T".to_string()]);
    assert_eq!(s.tuple_fields.len(), 1);
    assert_eq!(s.tuple_fields[0], RustType::simple("T"));
}

// --- Flatten extraction tests ---

#[test]
fn extracts_flatten_on_field() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Full {
                id: u64,
                #[serde(flatten)]
                meta: Metadata,
            }
            "#,
    );
    assert_eq!(manifest.structs.len(), 1);
    let fields = &manifest.structs[0].fields;
    assert_eq!(fields.len(), 2);
    assert!(!fields[0].flatten);
    assert!(fields[1].flatten);
    assert_eq!(fields[1].name, "meta");
    assert_eq!(fields[1].ty.name, "Metadata");
}

#[test]
fn flatten_in_enum_struct_variant() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            enum Event {
                Click {
                    x: i32,
                    #[serde(flatten)]
                    meta: Meta,
                },
            }
            "#,
    );
    assert_eq!(manifest.enums.len(), 1);
    match &manifest.enums[0].variants[0].kind {
        VariantKind::Struct(fields) => {
            assert_eq!(fields.len(), 2);
            assert!(!fields[0].flatten);
            assert!(fields[1].flatten);
        }
        _ => panic!("expected Struct variant"),
    }
}

#[test]
fn multiple_flattened_fields() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Combined {
                id: u64,
                #[serde(flatten)]
                a: PartA,
                #[serde(flatten)]
                b: PartB,
            }
            "#,
    );
    let fields = &manifest.structs[0].fields;
    assert_eq!(fields.len(), 3);
    assert!(!fields[0].flatten);
    assert!(fields[1].flatten);
    assert!(fields[2].flatten);
}

#[test]
fn flatten_with_skip() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Data {
                #[serde(flatten, skip)]
                hidden: Secret,
                value: String,
            }
            "#,
    );
    let fields = &manifest.structs[0].fields;
    assert_eq!(fields.len(), 2);
    assert!(fields[0].flatten);
    assert!(fields[0].skip);
}

// --- Qualified path preservation tests ---

#[test]
fn preserves_qualified_type_path() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Event {
                created_at: chrono::DateTime<chrono::Utc>,
            }
            "#,
    );
    let field_ty = &manifest.structs[0].fields[0].ty;
    assert_eq!(field_ty.name, "chrono::DateTime");
    assert_eq!(field_ty.generics[0].name, "chrono::Utc");
}

#[test]
fn simple_type_path_unchanged() {
    let manifest = common::parse_source(
        r#"
            #[derive(Serialize)]
            struct Foo {
                name: String,
            }
            "#,
    );
    assert_eq!(manifest.structs[0].fields[0].ty.name, "String");
}
