mod common;

use std::path::PathBuf;

use vercel_rpc_cli::codegen::svelte::generate_svelte_file;
use vercel_rpc_cli::model::*;

// --- imports ---

#[test]
fn svelte_imports_client_and_types() {
    let manifest = common::make_test_manifest();
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(
        output.contains(
            "import { type RpcClient, RpcError, type CallOptions } from \"./rpc-client\""
        )
    );
    assert!(output.contains("import type { Procedures"));
    assert!(output.contains("from \"./rpc-types\""));
}

#[test]
fn svelte_reexports() {
    let manifest = common::make_test_manifest();
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export { RpcError }"));
    assert!(output.contains("export type { RpcClient, CallOptions, Procedures"));
}

// --- createQuery ---

#[test]
fn svelte_contains_create_query() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function createQuery"));
}

#[test]
fn svelte_contains_create_mutation() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function createMutation"));
}

// --- Interfaces ---

#[test]
fn svelte_contains_query_options() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryOptions<K extends QueryKey>"));
}

#[test]
fn svelte_contains_query_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryResult<K extends QueryKey>"));
}

#[test]
fn svelte_contains_mutation_options() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationOptions<K extends MutationKey>"));
}

#[test]
fn svelte_contains_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationResult<K extends MutationKey>"));
}

// --- Runes ---

#[test]
fn svelte_uses_state_rune() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("$state"));
}

#[test]
fn svelte_uses_effect_rune() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("$effect"));
}

// --- Void/non-void ---

#[test]
fn svelte_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "time",
        None,
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains(
        "createQuery<K extends \"time\">(client: RpcClient, key: K, options?: QueryOptions<K>): QueryResult<K>"
    ));
}

#[test]
fn svelte_non_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
    assert!(output.contains(
        "createQuery<K extends \"hello\">(client: RpcClient, key: K, input: () => QueryInput<K>, options?: QueryOptions<K>): QueryResult<K>"
    ));
}

#[test]
fn svelte_void_query_set() {
    let manifest = common::make_manifest(vec![
        common::make_query("time", None, Some(RustType::simple("String"))),
        common::make_query(
            "hello",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
    ]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
}

#[test]
fn svelte_no_void_query_type_when_all_non_void() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(!output.contains("type VoidQueryKey"));
}

// --- Conditional emission ---

#[test]
fn svelte_queries_only_no_mutation() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("createQuery"));
    assert!(!output.contains("createMutation"));
    assert!(!output.contains("MutationKey"));
}

#[test]
fn svelte_mutations_only_no_query() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("createMutation"));
    assert!(!output.contains("createQuery"));
    assert!(!output.contains("QueryKey"));
}

#[test]
fn svelte_empty_manifest_returns_empty() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.is_empty());
}

// --- Result members ---

#[test]
fn svelte_refetch_in_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("refetch: fetchData"));
}

#[test]
fn svelte_reset_in_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("reset: ()"));
}

#[test]
fn svelte_mutate_async_in_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("mutateAsync"));
}

// --- Custom import paths ---

#[test]
fn svelte_custom_import_paths() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "$lib/rpc-client.js", "$lib/rpc-types.js", false);
    assert!(output.contains("from \"$lib/rpc-client.js\""));
    assert!(output.contains("from \"$lib/rpc-types.js\""));
}

// --- Type helpers ---

#[test]
fn svelte_type_helpers_emitted() {
    let manifest = common::make_test_manifest();
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type QueryKey = keyof Procedures[\"queries\"]"));
    assert!(output.contains("type MutationKey = keyof Procedures[\"mutations\"]"));
    assert!(output.contains("type QueryInput<K extends QueryKey>"));
    assert!(output.contains("type QueryOutput<K extends QueryKey>"));
    assert!(output.contains("type MutationInput<K extends MutationKey>"));
    assert!(output.contains("type MutationOutput<K extends MutationKey>"));
}

// --- User type imports ---

#[test]
fn svelte_imports_user_types() {
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
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("import type { Procedures, TimeResponse } from \"./rpc-types\""));
    assert!(output.contains("TimeResponse"));
}

// --- Polling cleanup ---

#[test]
fn svelte_refetch_interval_cleanup() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("setInterval"));
    assert!(output.contains("clearInterval"));
}

// --- Void mutation ---

#[test]
fn svelte_void_mutation_key() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "reset",
        None,
        Some(RustType::simple("bool")),
    )]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidMutationKey = \"reset\""));
}

// --- insta snapshot tests ---

#[test]
fn snapshot_svelte_full() {
    let manifest = common::make_test_manifest();
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_svelte_queries_only() {
    let manifest = common::make_manifest(vec![
        common::make_query(
            "get_user",
            Some(RustType::simple("String")),
            Some(RustType::simple("User")),
        ),
        common::make_query("version", None, Some(RustType::simple("String"))),
    ]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_svelte_mutations_only() {
    let manifest = common::make_manifest(vec![
        common::make_mutation(
            "create_item",
            Some(RustType::simple("CreateInput")),
            Some(RustType::simple("Item")),
        ),
        common::make_mutation("reset", None, Some(RustType::simple("bool"))),
    ]);
    let output = generate_svelte_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}
