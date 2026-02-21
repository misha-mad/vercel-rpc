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

### ~~Framework Reactive Wrappers~~ ✅ → [RFC-7](./RFC/RFC-7.md), [RFC-8](./RFC/RFC-8.md)

> **Svelte 5** — Implemented in RFC-7. Optional reactive wrapper file (`rpc.svelte.ts`) with `createQuery` and `createMutation` helpers that wrap `RpcClient` with `$state` / `$effect` runes. Opt-in via `output.svelte` config field or `--svelte-output` CLI flag.
>
> **React** — Implemented in RFC-8. Optional hook file (`rpc.react.ts`) with `useQuery` and `useMutation` hooks that wrap `RpcClient` with `useState` / `useEffect`. Opt-in via `output.react` config field or `--react-output` CLI flag.

### Query Race Condition Handling (AbortController)

When input changes rapidly (e.g. `userId: 1 → 2 → 3`), multiple fetch requests fire in parallel and responses may arrive out of order. A late response from an earlier input can overwrite the correct data:

```typescript
// userId changes: 1 → 2 → 3
// Requests fire in parallel, responses may arrive: 3, 1, 2
// setData(user2) overwrites the correct user3
```

The fix is to abort stale requests via `AbortController` in the `useEffect`/`$effect` cleanup:

```typescript
const fetchData = useCallback(async (signal: AbortSignal) => {
  // ... pass signal via callOptions
}, [...]);

useEffect(() => {
  const controller = new AbortController();
  fetchData(controller.signal);
  return () => controller.abort();
}, [fetchData]);
```

This is a known pattern — TanStack Query also didn't ship abort-on-stale in early versions. The current implementation is sufficient for most use cases.

### Serde Enum Representations

Serde supports four enum tagging strategies. Currently, only the default (externally tagged) is handled.

#### Internally tagged

```rust
#[derive(Serialize)]
#[serde(tag = "type")]
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
}
```

```typescript
type Shape =
  | { type: "Circle"; radius: number }
  | { type: "Rect"; w: number; h: number };
```

#### Adjacently tagged

```rust
#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
enum Event {
    Click { x: i32, y: i32 },
    Scroll(f64),
}
```

```typescript
type Event =
  | { t: "Click"; c: { x: number; y: number } }
  | { t: "Scroll"; c: number };
```

#### Untagged

```rust
#[derive(Serialize)]
#[serde(untagged)]
enum StringOrInt {
    Str(String),
    Int(i32),
}
```

```typescript
type StringOrInt = string | number;
```

### Generics

Support generic structs that produce generic TypeScript interfaces:

```rust
#[derive(Serialize)]
struct Paginated<T> {
    items: Vec<T>,
    total: u64,
    page: u32,
}
```

```typescript
export interface Paginated<T> {
  items: T[];
  total: number;
  page: number;
}
```

When used in a procedure signature like `Paginated<User>`, the codegen emits `Paginated<User>` in the procedures map.

### Newtype Branded Types

Single-field tuple structs (newtypes) can optionally generate branded types for nominal type safety:

```rust
#[derive(Serialize)]
struct UserId(String);
```

```typescript
// With branded types enabled (codegen.branded_newtypes = true):
export type UserId = string & { readonly __brand: unique symbol };

// Without (default):
export type UserId = string;
```

### `#[serde(flatten)]`

```rust
#[derive(Serialize)]
struct Full {
    id: u64,
    #[serde(flatten)]
    meta: Metadata,
}
```

```typescript
export type Full = { id: number } & Metadata;
```

---

## Phase 4 — Ecosystem

### External Crate Type Mappings

Common crate types mapped to TypeScript, configurable in `rpc.config.toml`:

```toml
[codegen.type_overrides]
"chrono::NaiveDate" = "string"
"chrono::DateTime" = "string"      # ISO 8601
"uuid::Uuid" = "string"
"serde_json::Value" = "unknown"
"rust_decimal::Decimal" = "string"
"url::Url" = "string"
```

The parser would match these by the path segments in the type. Users can add their own overrides for any custom type.

#### BigInt option

Large integer types can optionally map to `bigint` instead of `number`:

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

#### Server-Side Caching via `cache`

The `cache` attribute generates `Cache-Control` HTTP headers in the macro-expanded handler. On Vercel, this automatically enables edge caching without any infrastructure setup.

```rust
#[rpc_query(cache = "1h")]
async fn get_settings() -> Settings { ... }
// → Cache-Control: public, max-age=3600, s-maxage=3600

#[rpc_query(cache = "5m", stale = "1h")]
async fn get_feed() -> Vec<Post> { ... }
// → Cache-Control: public, max-age=300, s-maxage=300, stale-while-revalidate=3600

#[rpc_query(cache = "private, 10m")]
async fn get_profile() -> Profile { ... }
// → Cache-Control: private, max-age=600
```

Duration shorthand: `30s`, `5m`, `1h`, `1d`. The macro parses these at compile time and emits the appropriate header values. Mutations never set cache headers.

#### Other metadata
- `timeout` — sets per-procedure server-side and client-side timeouts.
- `idempotent` — enables safe client-side retries for mutations.

### Batch Requests

Allow multiple RPC calls in a single HTTP round-trip:

```typescript
const [greeting, time] = await client.batch((b) => [
  b.query("hello", "world"),
  b.query("time"),
]);
```

This requires a batch endpoint on the Rust side that dispatches to individual handlers. The batch endpoint could be auto-generated by the macro crate or provided as a standalone handler.

---

## Summary

| Phase | Focus      | Key Deliverables                                                                                                                      |
|-------|------------|---------------------------------------------------------------------------------------------------------------------------------------|
| **1** | Foundation | ~~Config file~~ ✅, ~~serde attributes~~ ✅, ~~expanded type support~~ ✅                                                                |
| **2** | Client     | ~~Client config (v1)~~ ✅, ~~client config (extended)~~ ✅, ~~per-call options~~ ✅, ~~request deduplication~~ ✅, ~~JSDoc generation~~ ✅ |
| **3** | DX         | ~~Framework wrappers (Svelte 5, React)~~ ✅, enum representations, generics, branded types, flatten                                    |
| **4** | Ecosystem  | External crate mappings, macro metadata, server-side caching, batch requests                                                          |
