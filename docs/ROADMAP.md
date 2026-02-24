# Roadmap

This document outlines the planned features and improvements for vercel-rpc, organized into phases.

## Phase 1 — Foundation

### ~~Configuration File (`rpc.config.toml`)~~ ✅

> Implemented in [RFC-2](./RFC/RFC-2.md). Full config file with CLI flag parity.

### ~~Serde Attribute Support~~ ✅

> Implemented in [RFC-3](./RFC/RFC-3.md). Supports `rename_all`, `rename`, `skip`/`skip_serializing`, and `default` on structs, enums, fields, and variants.

### ~~Expanded Primitive and Wrapper Types~~ ✅

> Implemented in [PR #41](https://github.com/misha-mad/vercel-rpc/pull/41). Sets map to `T[]`, smart pointers and `Cow` unwrap to inner type, fixed-size arrays already map to `T[]`.

---

## Phase 2 — Client

### ~~`RpcClientConfig` (first iteration)~~ ✅

> Implemented in [PR #43](https://github.com/misha-mad/vercel-rpc/pull/43). `createRpcClient` now accepts an `RpcClientConfig` object with `baseUrl`, optional `fetch` override (SSR, testing), and optional static/async `headers` (auth tokens).

### ~~`RpcClientConfig` — extended options~~ ✅

> Implemented in [RFC-4](./RFC/RFC-4.md). Lifecycle hooks (`onRequest`, `onResponse`, `onError`) in [PR #46](https://github.com/misha-mad/vercel-rpc/pull/46), retry policy and timeout in [PR #47](https://github.com/misha-mad/vercel-rpc/pull/47), custom serialize/deserialize in [PR #48](https://github.com/misha-mad/vercel-rpc/pull/48), and abort signal in [PR #49](https://github.com/misha-mad/vercel-rpc/pull/49).

### ~~Per-Call Options~~ ✅ → [RFC-5](./RFC/RFC-5.md)

> Implemented in RFC-5. Every `query()` and `mutate()` overload accepts an optional trailing `CallOptions` argument with per-request `signal`, `headers`, and `timeout` overrides.

### ~~Request Deduplication~~ ✅ → [RFC-6](./RFC/RFC-6.md)

> Implemented in RFC-6. Identical in-flight queries are automatically deduplicated via an `inflight` Map. Callers share the same promise; per-caller `AbortSignal` is wrapped independently. Mutations are never deduplicated. Controlled by `dedupe` option at both config and per-call level (defaults to `true`).

### ~~JSDoc from Doc-Comments~~ ✅

> Implemented via `codegen.preserve_docs` config option and `--preserve-docs` CLI flag.

---

## Phase 3 — Developer Experience

### ~~Framework Reactive Wrappers (Svelte 5, React)~~ ✅ → [RFC-7](./RFC/RFC-7.md), [RFC-8](./RFC/RFC-8.md)

> **Svelte 5** — Implemented in RFC-7. Optional reactive wrapper file (`rpc.svelte.ts`) with `createQuery` and `createMutation` helpers that wrap `RpcClient` with `$state` / `$effect` runes. Opt-in via `output.svelte` config field or `--svelte-output` CLI flag.
>
> **React** — Implemented in RFC-8. Optional hook file (`rpc.react.ts`) with `useQuery` and `useMutation` hooks that wrap `RpcClient` with `useState` / `useEffect`. Opt-in via `output.react` config field or `--react-output` CLI flag.

### ~~Framework Reactive Wrappers (Vue 3)~~ ✅ → [RFC-9](./RFC/RFC-9.md)

> **Vue 3** — Implemented in RFC-9. Optional composable file (`rpc.vue.ts`) with `useQuery` and `useMutation` using Vue 3 Composition API (`ref`, `computed`, `watch`, `onScopeDispose`). Opt-in via `output.vue` config field or `--vue-output` CLI flag.

### ~~Framework Reactive Wrappers (SolidJS)~~ ✅ → [RFC-10](./RFC/RFC-10.md)

> **SolidJS** — Implemented in RFC-10. Optional primitives file (`rpc.solid.ts`) with `createQuery` and `createMutation` using Solid reactivity (`createSignal`, `createEffect`, `createMemo`, `onCleanup`, `batch`). Opt-in via `output.solid` config field or `--solid-output` CLI flag.

### ~~Reactive Options for Framework Wrappers~~ ✅

> Implemented across all four framework wrappers. Overloads accept `options?: QueryOptions<K> | (() => QueryOptions<K>)`, with a `resolveOptions()` helper called inside the effect for reactive tracking.

### ~~Query Race Condition Handling (AbortController)~~ ✅

> Implemented across all four framework wrappers. Each effect cycle creates an `AbortController`, passes `signal` to `fetchData`, merges with user-provided `callOptions.signal` via `AbortSignal.any()`, and guards state updates with `signal?.aborted` checks. Cleanup aborts the controller and clears intervals. Manual `refetch()` does not pass a signal.

### ~~Serde Enum Representations~~ ✅

> Implemented in [PR #68](https://github.com/misha-mad/vercel-rpc/pull/68). All four serde enum tagging strategies are supported: externally tagged (default), internally tagged (`#[serde(tag = "...")]`), adjacently tagged (`#[serde(tag = "...", content = "...")]`), and untagged (`#[serde(untagged)]`). `#[serde(default)]` on `Option<T>` fields in enum struct variants is also handled correctly.

### ~~Generics~~ ✅

> Implemented: generic structs/enums produce generic TypeScript interfaces/types. Generic parameters are preserved in procedure signatures (e.g. `Paginated<User>`).

### ~~Newtype Branded Types~~ ✅

> Implemented via `codegen.branded_newtypes` config option and `--branded-newtypes` CLI flag. Single-field tuple structs (newtypes) produce type aliases; multi-field tuple structs produce TS tuples. With branded mode enabled, newtypes get nominal type safety via `& { readonly __brand: "TypeName" }`.

### ~~`#[serde(flatten)]`~~ ✅

> Implemented: `#[serde(flatten)]` on struct fields produces TypeScript intersection types (`{ id: number } & Metadata`). Works in top-level structs and all four enum struct-variant tagging strategies.

---

## Phase 4 — Ecosystem

### ~~External Crate Type Mappings~~ ✅

> Implemented via `[codegen.type_overrides]` config section and `--type-override` CLI flag. Override keys are matched by the last path segment, with exact full-path matching taking priority. The parser preserves fully-qualified type paths for disambiguation.

#### ~~BigInt option~~ ✅

> Implemented via `codegen.bigint_types` config option and `--bigint-type` CLI flag. Entries are merged into the type override machinery as `"bigint"` mappings, with explicit `type_overrides` taking priority.

~~Large integer types can optionally map to `bigint` instead of `number`:~~

```toml
[codegen]
bigint_types = ["i64", "u64", "i128", "u128"]
```

### Macro-Level Metadata

Extend `#[rpc_query]` and `#[rpc_mutation]` with optional metadata attributes:

```rust
#[rpc_query(cache = "1h")]
async fn get_settings() -> Settings { ... }

#[rpc_query(timeout = "30s")]
async fn slow_report(input: ReportParams) -> Report { ... }

#[rpc_mutation(idempotent)]
async fn create_order(input: Order) -> OrderResult { ... }
```

These attributes flow into the generated manifest and can influence both server and client behavior.

#### ~~Server-Side Caching via `cache`~~ ✅

> Implemented via `cache` and `stale` attributes on `#[rpc_query]`. The macro parses duration shorthand (`30s`, `5m`, `1h`, `1d`) at compile time and emits `Cache-Control` headers on success responses. Supports public (CDN) and private (browser-only) caching, with optional `stale-while-revalidate`. Mutations never receive cache headers.

```rust
#[rpc_query(cache = "1h")]
async fn get_settings() -> Settings { ... }
// → Cache-Control: public, max-age=0, s-maxage=3600

#[rpc_query(cache = "5m", stale = "1h")]
async fn get_feed() -> Vec<Post> { ... }
// → Cache-Control: public, max-age=0, s-maxage=300, stale-while-revalidate=3600

#[rpc_query(cache = "private, 10m")]
async fn get_profile() -> Profile { ... }
// → Cache-Control: private, max-age=600
```

#### ~~Cold-Start Initialization via `init`~~ ✅ → [RFC-11](./RFC/RFC-11.md)

> Implemented via `init` attribute on `#[rpc_query]` / `#[rpc_mutation]`. The init function runs once at cold start and can return shared state stored in a `OnceLock`, injected into the handler as a `&T` parameter. Supports side-effects only (logger, dotenv) and state injection (DB pool, HTTP client). Compatible with `cache` on queries. Mutations support `init` but not `cache`.

#### ~~Per-Procedure Timeout via `timeout`~~ ✅

> Implemented via `timeout` attribute on `#[rpc_query]` / `#[rpc_mutation]`. The macro wraps the handler with `tokio::time::timeout()`, returning a `504` error on expiration. The CLI extracts the timeout value and emits a `PROCEDURE_TIMEOUTS` map in the generated TypeScript client, used as a per-procedure default in the timeout resolution chain: `callOptions?.timeout ?? PROCEDURE_TIMEOUTS[procedure] ?? config.timeout`.

```rust
#[rpc_query(timeout = "30s")]
async fn slow_report(input: ReportParams) -> Report { ... }

#[rpc_mutation(timeout = "5m")]
async fn long_import(input: ImportData) -> ImportResult { ... }
```

#### ~~Idempotent Mutations~~ ✅ → [RFC-12](./RFC/RFC-12.md)

> Implemented via `idempotent` bare flag on `#[rpc_mutation]`. The generated client emits an `IDEMPOTENT_MUTATIONS` set and only retries mutations explicitly marked idempotent, while queries (GET) are always retryable. Prevents accidental duplicate side effects when retry is configured.

```rust
#[rpc_mutation(idempotent)]
async fn delete_item(id: u32) -> bool { ... }

#[rpc_mutation(idempotent, timeout = "30s")]
async fn upsert_user(input: UserInput) -> User { ... }
```

---

## Phase 5 — Streaming & Request Access

### Streaming Procedures via `#[rpc_stream]`

A new procedure type alongside `#[rpc_query]` and `#[rpc_mutation]` that enables HTTP streaming responses. Built on top of `vercel_runtime::axum::stream_response` and Axum's streaming primitives.

```rust
#[rpc_stream]
async fn chat(input: ChatInput, tx: StreamSender) {
    for token in generate_tokens(&input.prompt) {
        tx.send(token).await;
    }
}
```

**Key design points:**
- New `StreamSender` type (re-exported from `vercel_rpc`) wraps the Axum/hyper `Bytes` channel
- Handler receives typed input (deserialized as usual) plus a `StreamSender` for emitting chunks
- Generated TypeScript client gets a `stream()` method returning an `AsyncIterable` or `ReadableStream`
- Framework wrappers get `createStream` / `useStream` primitives with reactive state
- Requires `vercel_runtime` with `axum` feature and `VercelLayer` for the streaming entry point
- Streaming routes live in a separate Axum-based binary, coexisting with existing per-file lambdas

**Streaming formats to support:**
- Raw chunked (`text/plain`) — general purpose
- SSE (`text/event-stream`) — real-time events, LLM token streaming
- JSON Lines (`application/x-ndjson`) — structured streaming data

---

## Summary

| Phase | Focus      | Key Deliverables                                                                                                                                                                          |
|-------|------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| **1** | Foundation | ~~Config file~~ ✅, ~~serde attributes~~ ✅, ~~expanded type support~~ ✅                                                                                                                    |
| **2** | Client     | ~~Client config (v1)~~ ✅, ~~client config (extended)~~ ✅, ~~per-call options~~ ✅, ~~request deduplication~~ ✅, ~~JSDoc generation~~ ✅                                                     |
| **3** | DX         | ~~Framework wrappers (Svelte 5, React, Vue 3, SolidJS)~~ ✅, ~~reactive options~~ ✅, ~~AbortController~~ ✅, ~~enum representations~~ ✅, ~~generics~~ ✅, ~~branded types~~ ✅, ~~flatten~~ ✅ |
| **4** | Ecosystem  | ~~External crate mappings~~ ✅, ~~BigInt option~~ ✅, ~~macro metadata~~ ✅, ~~server-side caching~~ ✅, ~~init/state~~ ✅, ~~timeout~~ ✅, ~~idempotent mutations~~ ✅                          |
| **5** | Streaming  | `#[rpc_stream]` procedure type, SSE/chunked/NDJSON formats, framework stream primitives                                                                                                   |
