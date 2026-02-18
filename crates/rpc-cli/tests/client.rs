mod common;

use std::path::PathBuf;

use vercel_rpc_cli::codegen::client::generate_client_file;
use vercel_rpc_cli::model::*;

#[test]
fn contains_generated_header() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.starts_with("// This file is auto-generated"));
}

#[test]
fn imports_procedures_type() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("import type { Procedures } from \"./rpc-types\""));
}

#[test]
fn reexports_procedures_type() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("export type { Procedures }"));
}

#[test]
fn contains_rpc_error_class() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("export class RpcError extends Error"));
    assert!(output.contains("this.status = status"));
}

#[test]
fn contains_fetch_helper() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("async function rpcFetch("));
    assert!(output.contains("encodeURIComponent(JSON.stringify(input))"));
}

#[test]
fn generates_query_method() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("query(key: \"hello\", input: string): Promise<string>"));
    assert!(output.contains("rpcFetch(baseUrl, \"GET\", key, args[0])"));
    assert!(output.contains("export interface RpcClient"));
}

#[test]
fn generates_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "version",
        None,
        Some(RustType::simple("String")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("query(key: \"version\"): Promise<string>"));
}

#[test]
fn generates_mutation_method() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("mutate(key: \"create_item\", input: CreateInput): Promise<Item>"));
    assert!(output.contains("rpcFetch(baseUrl, \"POST\", key, args[0])"));
    assert!(output.contains("export interface RpcClient"));
}

#[test]
fn generates_void_mutation_overload() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "reset",
        None,
        Some(RustType::simple("bool")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("mutate(key: \"reset\"): Promise<boolean>"));
}

#[test]
fn generates_create_rpc_client_factory() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("export function createRpcClient(baseUrl: string)"));
}

#[test]
fn generates_type_helpers() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("type QueryKey = keyof Procedures[\"queries\"]"));
    assert!(output.contains("type QueryInput<K extends QueryKey>"));
    assert!(output.contains("type QueryOutput<K extends QueryKey>"));
}

#[test]
fn mixed_queries_and_mutations() {
    let manifest = common::make_manifest(vec![
        common::make_query(
            "get_user",
            Some(RustType::simple("String")),
            Some(RustType::simple("User")),
        ),
        common::make_mutation(
            "delete_user",
            Some(RustType::simple("String")),
            Some(RustType::simple("bool")),
        ),
    ]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("query(key: \"get_user\", input: string): Promise<User>"));
    assert!(output.contains("mutate(key: \"delete_user\", input: string): Promise<boolean>"));
}

#[test]
fn empty_manifest_no_methods() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("createRpcClient"));
    assert!(!output.contains("query("));
    assert!(!output.contains("mutate("));
}

#[test]
fn complex_types_in_overloads() {
    let manifest = common::make_manifest(vec![common::make_query(
        "search",
        Some(RustType::simple("SearchQuery")),
        Some(RustType::with_generics(
            "Vec",
            vec![RustType::simple("SearchResult")],
        )),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("query(key: \"search\", input: SearchQuery): Promise<SearchResult[]>"));
}

#[test]
fn uses_get_for_queries_post_for_mutations() {
    let manifest = common::make_manifest(vec![
        common::make_query(
            "q",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
        common::make_mutation(
            "m",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
    ]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("rpcFetch(baseUrl, \"GET\", key"));
    assert!(output.contains("rpcFetch(baseUrl, \"POST\", key"));
}

#[test]
fn custom_types_import_path() {
    let output = generate_client_file(&common::make_manifest(vec![]), "$lib/rpc-types", false);
    assert!(output.contains("from \"$lib/rpc-types\""));
}

#[test]
fn import_path_with_extension() {
    let output = generate_client_file(&common::make_manifest(vec![]), "./rpc-types.js", false);
    assert!(output.contains("from \"./rpc-types.js\""));
}

#[test]
fn interface_based_overloads() {
    let manifest = common::make_manifest(vec![common::make_query(
        "test",
        None,
        Some(RustType::simple("String")),
    )]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("export interface RpcClient"));
    assert!(output.contains("query(key: \"test\"): Promise<string>"));
    assert!(output.contains("createRpcClient(baseUrl: string): RpcClient"));
    assert!(output.contains("as RpcClient"));
}

#[test]
fn response_unwrapping() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("json?.result?.data ?? json"));
}

#[test]
fn imports_referenced_structs() {
    let manifest = Manifest {
        procedures: vec![common::make_query(
            "get_time",
            None,
            Some(RustType::simple("TimeResponse")),
        )],
        structs: vec![StructDef {
            name: "TimeResponse".to_string(),
            fields: vec![("timestamp".to_string(), RustType::simple("u64"))],
            source_file: PathBuf::from("api/time.rs"),
            docs: None,
        }],
        enums: vec![],
    };
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("import type { Procedures, TimeResponse } from \"./rpc-types\""));
    assert!(output.contains("export type { Procedures, TimeResponse }"));
}

#[test]
fn error_handling_in_fetch() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(output.contains("if (!res.ok)"));
    assert!(output.contains("throw new RpcError("));
}

#[test]
fn test_jsdoc_on_overload() {
    let manifest = common::make_manifest(vec![
        Procedure {
            name: "hello".to_string(),
            kind: ProcedureKind::Query,
            input: Some(RustType::simple("String")),
            output: Some(RustType::simple("String")),
            source_file: PathBuf::from("api/hello.rs"),
            docs: Some("Say hello.".to_string()),
        },
        Procedure {
            name: "reset".to_string(),
            kind: ProcedureKind::Mutation,
            input: None,
            output: Some(RustType::simple("bool")),
            source_file: PathBuf::from("api/reset.rs"),
            docs: Some("Reset state.".to_string()),
        },
    ]);
    let output = generate_client_file(&manifest, "./rpc-types", true);
    assert!(
        output.contains(
            "  /** Say hello. */\n  query(key: \"hello\", input: string): Promise<string>;"
        )
    );
    assert!(output.contains("  /** Reset state. */\n  mutate(key: \"reset\"): Promise<boolean>;"));
}

#[test]
fn test_jsdoc_on_void_query_overload() {
    let manifest = common::make_manifest(vec![Procedure {
        name: "version".to_string(),
        kind: ProcedureKind::Query,
        input: None,
        output: Some(RustType::simple("String")),
        source_file: PathBuf::from("api/version.rs"),
        docs: Some("Get version.".to_string()),
    }]);
    let output = generate_client_file(&manifest, "./rpc-types", true);
    assert!(output.contains("  /** Get version. */\n  query(key: \"version\"): Promise<string>;"));
}

#[test]
fn test_jsdoc_on_non_void_mutation_overload() {
    let manifest = common::make_manifest(vec![Procedure {
        name: "update".to_string(),
        kind: ProcedureKind::Mutation,
        input: Some(RustType::simple("String")),
        output: Some(RustType::simple("bool")),
        source_file: PathBuf::from("api/update.rs"),
        docs: Some("Update item.".to_string()),
    }]);
    let output = generate_client_file(&manifest, "./rpc-types", true);
    assert!(output.contains(
        "  /** Update item. */\n  mutate(key: \"update\", input: string): Promise<boolean>;"
    ));
}

#[test]
fn test_no_jsdoc_on_overload_when_disabled() {
    let manifest = common::make_manifest(vec![Procedure {
        name: "hello".to_string(),
        kind: ProcedureKind::Query,
        input: Some(RustType::simple("String")),
        output: Some(RustType::simple("String")),
        source_file: PathBuf::from("api/hello.rs"),
        docs: Some("Say hello.".to_string()),
    }]);
    let output = generate_client_file(&manifest, "./rpc-types", false);
    assert!(!output.contains("/**"));
}
