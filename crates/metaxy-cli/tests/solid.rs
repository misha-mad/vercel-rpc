mod common;

use std::path::PathBuf;

use metaxy_cli::codegen::solid::generate_solid_file;
use metaxy_cli::model::*;

// --- imports ---

#[test]
fn solid_imports_client_and_types() {
    let manifest = common::make_test_manifest();
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(
        output.contains(
            "import { type RpcClient, RpcError, type CallOptions } from \"./rpc-client\""
        )
    );
    assert!(output.contains("import type { Procedures"));
    assert!(output.contains("from \"./rpc-types\""));
}

#[test]
fn solid_imports_solid() {
    let manifest = common::make_test_manifest();
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains(
        "import { createSignal, createEffect, createMemo, onCleanup, batch, untrack } from \"solid-js\""
    ));
}

#[test]
fn solid_reexports() {
    let manifest = common::make_test_manifest();
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export { RpcError }"));
    assert!(output.contains("export type { RpcClient, CallOptions, Procedures"));
}

// --- createQuery ---

#[test]
fn solid_contains_create_query() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function createQuery"));
}

#[test]
fn solid_contains_create_mutation() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export function createMutation"));
}

// --- Interfaces ---

#[test]
fn solid_contains_query_options() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryOptions<K extends QueryKey>"));
}

#[test]
fn solid_contains_query_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface QueryResult<K extends QueryKey>"));
}

#[test]
fn solid_contains_mutation_options() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationOptions<K extends MutationKey>"));
}

#[test]
fn solid_contains_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("export interface MutationResult<K extends MutationKey>"));
}

// --- SolidJS reactivity ---

#[test]
fn solid_uses_create_signal() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("createSignal("));
}

#[test]
fn solid_uses_create_effect() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("createEffect("));
}

#[test]
fn solid_uses_on_cleanup() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("onCleanup("));
}

#[test]
fn solid_accessor_return_type() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("data: () => QueryOutput<K> | undefined"));
    assert!(output.contains("error: () => RpcError | undefined"));
    assert!(output.contains("isLoading: () => boolean"));
    assert!(output.contains("isSuccess: () => boolean"));
    assert!(output.contains("isError: () => boolean"));
}

// --- Void/non-void ---

#[test]
fn solid_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "time",
        None,
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains(
        "createQuery<K extends \"time\">(client: RpcClient, key: K, options?: QueryOptions<K> | (() => QueryOptions<K>)): QueryResult<K>"
    ));
}

#[test]
fn solid_non_void_query_overload() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
    assert!(output.contains(
        "createQuery<K extends \"hello\">(client: RpcClient, key: K, input: () => QueryInput<K>, options?: QueryOptions<K> | (() => QueryOptions<K>)): QueryResult<K>"
    ));
}

#[test]
fn solid_void_query_set() {
    let manifest = common::make_manifest(vec![
        common::make_query("time", None, Some(RustType::simple("String"))),
        common::make_query(
            "hello",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
    ]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidQueryKey = \"time\""));
    assert!(output.contains("type NonVoidQueryKey = \"hello\""));
}

#[test]
fn solid_no_void_query_type_when_all_non_void() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(!output.contains("type VoidQueryKey"));
}

// --- Conditional emission ---

#[test]
fn solid_queries_only_no_mutation() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("createQuery"));
    assert!(!output.contains("createMutation"));
    assert!(!output.contains("MutationKey"));
}

#[test]
fn solid_mutations_only_no_query() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("createMutation"));
    assert!(!output.contains("createQuery"));
    assert!(!output.contains("QueryKey"));
}

#[test]
fn solid_empty_manifest_returns_empty() {
    let manifest = common::make_manifest(vec![]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.is_empty());
}

// --- Result members ---

#[test]
fn solid_refetch_in_result() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("refetch:"));
}

#[test]
fn solid_reset_in_mutation_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("reset:"));
}

#[test]
fn solid_mutate_async_in_result() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("mutateAsync"));
}

// --- Custom import paths ---

#[test]
fn solid_custom_import_paths() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(
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
fn solid_type_helpers_emitted() {
    let manifest = common::make_test_manifest();
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type QueryKey = keyof Procedures[\"queries\"]"));
    assert!(output.contains("type MutationKey = keyof Procedures[\"mutations\"]"));
    assert!(output.contains("type QueryInput<K extends QueryKey>"));
    assert!(output.contains("type QueryOutput<K extends QueryKey>"));
    assert!(output.contains("type MutationInput<K extends MutationKey>"));
    assert!(output.contains("type MutationOutput<K extends MutationKey>"));
}

// --- User type imports ---

#[test]
fn solid_imports_user_types() {
    let manifest = Manifest {
        procedures: vec![common::make_query(
            "get_time",
            None,
            Some(RustType::simple("TimeResponse")),
        )],
        structs: vec![StructDef {
            name: "TimeResponse".to_string(),
            generics: vec![],
            fields: vec![common::field("timestamp", RustType::simple("u64"))],
            tuple_fields: vec![],
            source_file: PathBuf::from("api/time.rs"),
            docs: None,
            rename_all: None,
        }],
        enums: vec![],
    };
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("import type { Procedures, TimeResponse } from \"./rpc-types\""));
    assert!(output.contains("TimeResponse"));
}

// --- Polling cleanup ---

#[test]
fn solid_refetch_interval_cleanup() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("setInterval"));
    assert!(output.contains("clearInterval"));
    assert!(output.contains("onCleanup("));
    assert!(output.contains("controller.abort()"));
}

// --- Void mutation ---

#[test]
fn solid_void_mutation_key() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "reset",
        None,
        Some(RustType::simple("bool")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("type VoidMutationKey = \"reset\""));
}

// --- SolidJS-specific: enabled supports getter ---

#[test]
fn solid_enabled_supports_getter() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("enabled?: boolean | (() => boolean)"));
}

// --- SolidJS-specific: input is getter ---

#[test]
fn solid_non_void_query_uses_getter_input() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("input: () => QueryInput<K>"));
}

// --- SolidJS-specific: setData uses direct value (not updater function form) ---

#[test]
fn solid_set_data_uses_direct_value() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    // Direct value with Exclude<T, Function> cast — avoids Solid treating result as an updater
    assert!(output.contains("setData(result as Exclude<"));
    assert!(!output.contains("setData(() => result)"));
}

// --- SolidJS-specific: createMemo for derived state ---

#[test]
fn solid_uses_create_memo_for_derived() {
    let manifest = common::make_test_manifest();
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("const isSuccess = createMemo("));
    assert!(output.contains("const isError = createMemo("));
}

// --- SolidJS-specific: batch in reset ---

#[test]
fn solid_reset_uses_batch() {
    let manifest = common::make_manifest(vec![common::make_mutation(
        "create_item",
        Some(RustType::simple("CreateInput")),
        Some(RustType::simple("Item")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("reset: () => batch("));
}

// --- SolidJS-specific: isLoading starts as true when enabled ---

#[test]
fn solid_is_loading_initial_true() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("const [isLoading, setIsLoading] = createSignal(initialEnabled)"));
    assert!(output.contains("const initialOpts = resolveOptions()"));
    assert!(output.contains("const initialEnabled ="));
}

// --- AbortController & reactive options ---

#[test]
fn solid_abort_controller_in_effect() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("new AbortController()"));
    assert!(output.contains("controller.abort()"));
}

#[test]
fn solid_fetch_receives_signal() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("controller.signal"));
}

#[test]
fn solid_abort_guard_in_catch() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    // Generation counter guards replace signal?.aborted
    assert!(output.contains("gen !== generation"));
    assert!(output.contains("gen === generation"));
}

#[test]
fn solid_signal_merge() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("AbortSignal.any("));
}

#[test]
fn solid_options_accepts_getter() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("QueryOptions<K> | (() => QueryOptions<K>)"));
}

#[test]
fn solid_resolve_options_helper() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("resolveOptions"));
}

// --- VOID_QUERY_KEYS ---

#[test]
fn solid_void_query_keys_set() {
    let manifest = common::make_manifest(vec![
        common::make_query("time", None, Some(RustType::simple("String"))),
        common::make_query(
            "hello",
            Some(RustType::simple("String")),
            Some(RustType::simple("String")),
        ),
    ]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains(r#"const VOID_QUERY_KEYS: Set<QueryKey> = new Set(["time"])"#));
    assert!(output.contains("VOID_QUERY_KEYS.has(key)"));
}

#[test]
fn solid_void_query_keys_multiple() {
    let manifest = common::make_manifest(vec![
        common::make_query("time", None, Some(RustType::simple("String"))),
        common::make_query("version", None, Some(RustType::simple("String"))),
    ]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains(r#"new Set(["time", "version"])"#));
}

// --- Generation counter ---

#[test]
fn solid_generation_counter() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("let generation = 0"));
    assert!(output.contains("generation++"));
    assert!(output.contains("gen !== generation"));
}

// --- refetch race safety ---

#[test]
fn solid_refetch_has_local_controller() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    assert!(output.contains("const localController = new AbortController()"));
    assert!(output.contains("refetch: async ()"));
}

// --- enabled=false resets isLoading ---

#[test]
fn solid_disabled_resets_loading() {
    let manifest = common::make_manifest(vec![common::make_query(
        "hello",
        Some(RustType::simple("String")),
        Some(RustType::simple("String")),
    )]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    // When enabled becomes false in createEffect, isLoading must be reset
    assert!(output.contains("setIsLoading(false);\n      controller = undefined;"));
}

// --- insta snapshot tests ---

#[test]
fn snapshot_solid_full() {
    let manifest = common::make_test_manifest();
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_solid_queries_only() {
    let manifest = common::make_manifest(vec![
        common::make_query(
            "get_user",
            Some(RustType::simple("String")),
            Some(RustType::simple("User")),
        ),
        common::make_query("version", None, Some(RustType::simple("String"))),
    ]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}

#[test]
fn snapshot_solid_mutations_only() {
    let manifest = common::make_manifest(vec![
        common::make_mutation(
            "create_item",
            Some(RustType::simple("CreateInput")),
            Some(RustType::simple("Item")),
        ),
        common::make_mutation("reset", None, Some(RustType::simple("bool"))),
    ]);
    let output = generate_solid_file(&manifest, "./rpc-client", "./rpc-types", false);
    insta::assert_snapshot!(output);
}
