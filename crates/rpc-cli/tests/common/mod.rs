#![allow(dead_code)]

use std::io::Write;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use vercel_rpc_cli::model::*;
use vercel_rpc_cli::parser::extract::parse_file;

/// Shorthand to create a `FieldDef` with no serde overrides.
pub fn field(name: &str, ty: RustType) -> FieldDef {
    FieldDef {
        name: name.to_string(),
        ty,
        rename: None,
        skip: false,
        has_default: false,
    }
}

pub fn parse_source(source: &str) -> Manifest {
    let mut tmp = NamedTempFile::new().unwrap();
    write!(tmp, "{}", source).unwrap();
    parse_file(tmp.path()).unwrap()
}

pub fn write_rpc_file(dir: &Path, rel_path: &str) {
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

pub fn make_query(name: &str, input: Option<RustType>, output: Option<RustType>) -> Procedure {
    Procedure {
        name: name.to_string(),
        kind: ProcedureKind::Query,
        input,
        output,
        source_file: PathBuf::from("api/test.rs"),
        docs: None,
    }
}

pub fn make_mutation(name: &str, input: Option<RustType>, output: Option<RustType>) -> Procedure {
    Procedure {
        name: name.to_string(),
        kind: ProcedureKind::Mutation,
        input,
        output,
        source_file: PathBuf::from("api/test.rs"),
        docs: None,
    }
}

pub fn make_manifest(procedures: Vec<Procedure>) -> Manifest {
    Manifest {
        procedures,
        structs: vec![],
        enums: vec![],
    }
}

pub fn make_test_manifest() -> Manifest {
    Manifest {
        procedures: vec![
            Procedure {
                name: "hello".to_string(),
                kind: ProcedureKind::Query,
                input: Some(RustType::simple("String")),
                output: Some(RustType::simple("String")),
                source_file: PathBuf::from("api/hello.rs"),
                docs: None,
            },
            Procedure {
                name: "time".to_string(),
                kind: ProcedureKind::Query,
                input: None,
                output: Some(RustType::simple("TimeResponse")),
                source_file: PathBuf::from("api/time.rs"),
                docs: None,
            },
            Procedure {
                name: "create_item".to_string(),
                kind: ProcedureKind::Mutation,
                input: Some(RustType::simple("CreateInput")),
                output: Some(RustType::simple("Item")),
                source_file: PathBuf::from("api/create_item.rs"),
                docs: None,
            },
        ],
        structs: vec![
            StructDef {
                name: "TimeResponse".to_string(),
                fields: vec![
                    field("timestamp", RustType::simple("u64")),
                    field("message", RustType::simple("String")),
                ],
                source_file: PathBuf::from("api/time.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "CreateInput".to_string(),
                fields: vec![
                    field("title", RustType::simple("String")),
                    field("count", RustType::simple("i32")),
                ],
                source_file: PathBuf::from("api/create_item.rs"),
                docs: None,
                rename_all: None,
            },
            StructDef {
                name: "Item".to_string(),
                fields: vec![
                    field("id", RustType::simple("u64")),
                    field("title", RustType::simple("String")),
                    field(
                        "tags",
                        RustType::with_generics("Vec", vec![RustType::simple("String")]),
                    ),
                ],
                source_file: PathBuf::from("api/create_item.rs"),
                docs: None,
                rename_all: None,
            },
        ],
        enums: vec![],
    }
}
