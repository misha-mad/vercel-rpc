#![allow(dead_code)]

use std::io::Write;
use std::path::{Path, PathBuf};

use tempfile::NamedTempFile;

use vercel_rpc_cli::model::*;
use vercel_rpc_cli::parser::extract::parse_file;

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
                    ("timestamp".to_string(), RustType::simple("u64")),
                    ("message".to_string(), RustType::simple("String")),
                ],
                source_file: PathBuf::from("api/time.rs"),
                docs: None,
            },
            StructDef {
                name: "CreateInput".to_string(),
                fields: vec![
                    ("title".to_string(), RustType::simple("String")),
                    ("count".to_string(), RustType::simple("i32")),
                ],
                source_file: PathBuf::from("api/create_item.rs"),
                docs: None,
            },
            StructDef {
                name: "Item".to_string(),
                fields: vec![
                    ("id".to_string(), RustType::simple("u64")),
                    ("title".to_string(), RustType::simple("String")),
                    (
                        "tags".to_string(),
                        RustType::with_generics("Vec", vec![RustType::simple("String")]),
                    ),
                ],
                source_file: PathBuf::from("api/create_item.rs"),
                docs: None,
            },
        ],
        enums: vec![],
    }
}
