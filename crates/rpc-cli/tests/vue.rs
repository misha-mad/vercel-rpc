mod common;

use std::path::PathBuf;

use vercel_rpc_cli::codegen::vue::generate_vue_file;
use vercel_rpc_cli::model::*;

// --- imports ---

#[test]
fn vue_imports_client_and_types() {
    let manifest = common::make_test_manifest();
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(
        output.contains(
            "import { type RpcClient, RpcError, type CallOptions } from \"./rpc-client\""
        )
    );
    assert!(output.contains("import type { Procedures"));
    assert!(output.contains("from \"./rpc-types\""));
}

#[test]
fn vue_imports_vue() {
    let manifest = common::make_test_manifest();
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains(
        "import { ref, computed, watch, onScopeDispose, type Ref, type ComputedRef } from \"vue\""
    ));
}

#[test]
fn vue_reexports() {
    let manifest = common::make_test_manifest();
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export { RpcError }"));
    assert!(output.contains("export type { RpcClient, CallOptions, Procedures"));
}

// --- useQuery ---

#[test]
fn vue_contains_use_query() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function useQuery"));
}

#[test]
fn vue_contains_use_mutation() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function useMutation"));
}

// --- Interfaces ---

#[test]
fn vue_contains_query_options() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryOptions<K extends QueryKey>"));
}

#[test]
fn vue_contains_query_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryResult<K extends QueryKey>"));
}

#[test]
fn vue_contains_mutation_options() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationOptions<K extends MutationKey>"));
}

#[test]
fn vue_contains_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationResult<K extends MutationKey>"));
}

// --- Vue reactivity ---

#[test]
fn vue_uses_computed() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("computed("));
}

#[test]
fn vue_uses_ref() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("ref<"));
}

#[test]
fn vue_uses_watch() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("watch("));
}

#[test]
fn vue_uses_on_scope_dispose() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("onScopeDispose("));
}

#[test]
fn vue_query_result_uses_ref_type() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("readonly data: Ref<QueryOutput<K> | undefined>"));
    assert!(output.contains("readonly error: Ref<RpcError | undefined>"));
    assert!(output.contains("readonly isLoading: Ref<boolean>"));
    assert!(output.contains("readonly isSuccess: ComputedRef<boolean>"));
    assert!(output.contains("readonly isError: ComputedRef<boolean>"));
}

// --- Void/non-void ---

#[test]
fn vue_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "time",
        None,
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains(
        "useQuery<K extends \"time\">(client: RpcClient, key: K, options?: QueryOptions<K> | (() => QueryOptions<K>)): QueryResult<K>"
    ));
}

#[test]
fn vue_non_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
    assert!(output.contains(
        "useQuery<K extends \"hello\">(client: RpcClient, key: K, input: () => QueryInput<K>, options?: QueryOptions<K> | (() => QueryOptions<K>)): QueryResult<K>"
    ));
}

#[test]
fn vue_void_query_set() {
    let manifest = common::make_manifest(vec![
        common::make_query("time", None, Some(RustType::simple("String"))),
        common::make_query(
            "hello",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
    ]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
}

#[test]
fn vue_no_void_query_type_when_all_non_void() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(!output.contains("type VoidQueryKey"));
}

// --- Conditional emission ---

#[test]
fn vue_queries_only_no_mutation() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("useQuery"));
    assert!(!output.contains("useMutation"));
    assert!(!output.contains("MutationKey"));
}

#[test]
fn vue_mutations_only_no_query() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("useMutation"));
    assert!(!output.contains("useQuery"));
    assert!(!output.contains("QueryKey"));
}

#[test]
fn vue_empty_manifest_returns_empty() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.is_empty());
}

// --- Result members ---

#[test]
fn vue_refetch_in_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("refetch:"));
}

#[test]
fn vue_reset_in_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("reset:"));
}

#[test]
fn vue_mutate_async_in_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("mutateAsync"));
}

// --- Custom import paths ---

#[test]
fn vue_custom_import_paths() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(
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
fn vue_type_helpers_emitted() {
    let manifest = common::make_test_manifest();
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type QueryKey = keyof Procedures[\"queries\"]"));
    assert!(output.contains("type MutationKey = keyof Procedures[\"mutations\"]"));
    assert!(output.contains("type QueryInput<K extends QueryKey>"));
    assert!(output.contains("type QueryOutput<K extends QueryKey>"));
    assert!(output.contains("type MutationInput<K extends MutationKey>"));
    assert!(output.contains("type MutationOutput<K extends MutationKey>"));
}

// --- User type imports ---

#[test]
fn vue_imports_user_types() {
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
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("import type { Procedures, TimeResponse } from \"./rpc-types\""));
    assert!(output.contains("TimeResponse"));
}

// --- Polling cleanup ---

#[test]
fn vue_refetch_interval_cleanup() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("setInterval"));
    assert!(output.contains("clearInterval"));
    assert!(output.contains("controller.abort()"));
}

// --- Void mutation ---

#[test]
fn vue_void_mutation_key() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "reset",
        None,
        Some(RustType::simple("bool")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidMutationKey = \"reset\""));
}

// --- Watch uses JSON.stringify instead of deep: true ---

#[test]
fn vue_watch_uses_serialized_input() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("JSON.stringify(input)"));
    assert!(!output.contains("deep: true"));
}

// --- Vue-specific: enabled supports getter ---

#[test]
fn vue_enabled_supports_getter() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("enabled?: boolean | (() => boolean)"));
}

// --- Vue-specific: input is getter ---

#[test]
fn vue_non_void_query_uses_getter_input() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("input: () => QueryInput<K>"));
}

// --- AbortController & reactive options ---

#[test]
fn vue_abort_controller_in_effect() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("new AbortController()"));
    assert!(output.contains("controller.abort()"));
}

#[test]
fn vue_fetch_receives_signal() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("ctrl.signal"));
}

#[test]
fn vue_abort_guard_in_catch() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("signal?.aborted"));
}

#[test]
fn vue_signal_merge() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("AbortSignal.any("));
}

#[test]
fn vue_options_accepts_getter() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("QueryOptions<K> | (() => QueryOptions<K>)"));
}

#[test]
fn vue_resolve_options_helper() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("resolveOptions"));
}

// --- insta snapshot tests ---

#[test]
fn snapshot_vue_full() {
    let manifest = common::make_test_manifest();
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_vue_queries_only() {
    let manifest = common::make_manifest(vec![
        common::make_query(
            "get_user",
            Some(RustType::simple("String")),
            Some(RustType::simple("User")),
        ),
        common::make_query("version", None, Some(RustType::simple("String"))),
    ]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_vue_mutations_only() {
    let manifest = common::make_manifest(vec![
        common::make_mutation(
            "create_item",
            Some(RustType::simple("CreateInput")),
            Some(RustType::simple("Item")),
        ),
        common::make_mutation("reset", None, Some(RustType::simple("bool"))),
    ]);
    let output = generate_vue_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}
