mod common;

use std::path::PathBuf;

use vercel_rpc_cli::codegen::react::generate_react_file;
use vercel_rpc_cli::model::*;

// --- imports ---

#[test]
fn react_imports_client_and_types() {
    let manifest = common::make_test_manifest();
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(
        output.contains(
            "import { type RpcClient, RpcError, type CallOptions } from \"./rpc-client\""
        )
    );
    assert!(output.contains("import type { Procedures"));
    assert!(output.contains("from \"./rpc-types\""));
}

#[test]
fn react_imports_react() {
    let manifest = common::make_test_manifest();
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("import { useState, useEffect, useRef, useCallback } from \"react\""));
}

#[test]
fn react_reexports() {
    let manifest = common::make_test_manifest();
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export { RpcError }"));
    assert!(output.contains("export type { RpcClient, CallOptions, Procedures"));
}

// --- useQuery ---

#[test]
fn react_contains_use_query() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function useQuery"));
}

#[test]
fn react_contains_use_mutation() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function useMutation"));
}

// --- Interfaces ---

#[test]
fn react_contains_query_options() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryOptions<K extends QueryKey>"));
}

#[test]
fn react_contains_query_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryResult<K extends QueryKey>"));
}

#[test]
fn react_contains_mutation_options() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationOptions<K extends MutationKey>"));
}

#[test]
fn react_contains_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationResult<K extends MutationKey>"));
}

// --- React hooks ---

#[test]
fn react_uses_use_state() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("useState"));
}

#[test]
fn react_uses_use_effect() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("useEffect"));
}

// --- Void/non-void ---

#[test]
fn react_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "time",
        None,
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains(
        "useQuery<K extends \"time\">(client: RpcClient, key: K, options?: QueryOptions<K>): QueryResult<K>"
    ));
}

#[test]
fn react_non_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
    assert!(output.contains(
        "useQuery<K extends \"hello\">(client: RpcClient, key: K, input: QueryInput<K>, options?: QueryOptions<K>): QueryResult<K>"
    ));
}

#[test]
fn react_void_query_set() {
    let manifest = common::make_manifest(vec![
        common::make_query("time", None, Some(RustType::simple("String"))),
        common::make_query(
            "hello",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
    ]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
}

#[test]
fn react_no_void_query_type_when_all_non_void() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(!output.contains("type VoidQueryKey"));
}

// --- Conditional emission ---

#[test]
fn react_queries_only_no_mutation() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("useQuery"));
    assert!(!output.contains("useMutation"));
    assert!(!output.contains("MutationKey"));
}

#[test]
fn react_mutations_only_no_query() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("useMutation"));
    assert!(!output.contains("useQuery"));
    assert!(!output.contains("QueryKey"));
}

#[test]
fn react_empty_manifest_returns_empty() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.is_empty());
}

// --- Result members ---

#[test]
fn react_refetch_in_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("refetch: fetchData"));
}

#[test]
fn react_reset_in_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("reset,"));
}

#[test]
fn react_mutate_async_in_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("mutateAsync"));
}

// --- Custom import paths ---

#[test]
fn react_custom_import_paths() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(
        &manifest,
        "@/lib/rpc-client.js",
        "@/lib/rpc-types.js",
        false,
    );
    assert!(output.contains("from \"@/lib/rpc-client.js\""));
    assert!(output.contains("from \"@/lib/rpc-types.js\""));
}

// --- Type helpers ---

#[test]
fn react_type_helpers_emitted() {
    let manifest = common::make_test_manifest();
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type QueryKey = keyof Procedures[\"queries\"]"));
    assert!(output.contains("type MutationKey = keyof Procedures[\"mutations\"]"));
    assert!(output.contains("type QueryInput<K extends QueryKey>"));
    assert!(output.contains("type QueryOutput<K extends QueryKey>"));
    assert!(output.contains("type MutationInput<K extends MutationKey>"));
    assert!(output.contains("type MutationOutput<K extends MutationKey>"));
}

// --- User type imports ---

#[test]
fn react_imports_user_types() {
    let manifest = Manifest {
        procedures: vec![common::make_query(
            "get_time",
            None,
            Some(RustType::simple("TimeResponse")),
        )],
        structs: vec![StructDef {
            name: "TimeResponse".to_string(),
            fields: vec![common::field("timestamp", RustType::simple("u64"))],
            source_file: PathBuf::from("api/time.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("import type { Procedures, TimeResponse } from \"./rpc-types\""));
    assert!(output.contains("TimeResponse"));
}

// --- Polling cleanup ---

#[test]
fn react_refetch_interval_cleanup() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("setInterval"));
    assert!(output.contains("clearInterval"));
}

// --- Void mutation ---

#[test]
fn react_void_mutation_key() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "reset",
        None,
        Some(RustType::simple("bool")),
    )]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidMutationKey = \"reset\""));
}

// --- insta snapshot tests ---

#[test]
fn snapshot_react_full() {
    let manifest = common::make_test_manifest();
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_react_queries_only() {
    let manifest = common::make_manifest(vec![
        common::make_query(
            "get_user",
            Some(RustType::simple("String")),
            Some(RustType::simple("User")),
        ),
        common::make_query("version", None, Some(RustType::simple("String"))),
    ]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_react_mutations_only() {
    let manifest = common::make_manifest(vec![
        common::make_mutation(
            "create_item",
            Some(RustType::simple("CreateInput")),
            Some(RustType::simple("Item")),
        ),
        common::make_mutation("reset", None, Some(RustType::simple("bool"))),
    ]);
    let output = generate_react_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}
