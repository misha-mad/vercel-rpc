# Roadmap

This document outlines the planned features and improvements for vercel-rpc, organized into phases.

## Phase 1 — Foundation

### ~~Configuration File (`rpc.config.toml`)~~ ✅

> Implemented in [RFC-2](./RFC-2.md). Full config file with CLI flag parity.

Replace CLI-only configuration with a project-level config file. The CLI flags remain as overrides.

```toml
# rpc.config.toml

[input]
dir = "api"
include = ["**/*.rs"]
exclude = ["**/mod.rs", "**/lib.rs", "**/_*.rs"]

[output]
types = "src/lib/generated/rpc-types.ts"
client = "src/lib/generated/rpc-client.ts"

[output.types_import]
path = "./rpc-types"
extension = ".js"                # ESM-compatible imports: "./rpc-types.js"

[codegen]
client_style = "factory"         # "class" | "factory" | "hooks"
preserve_docs = true             # forward Rust doc-comments as JSDoc
barrel_export = true             # generate index.ts re-exporting everything

[codegen.naming]
types = "PascalCase"
fields = "camelCase"             # snake_case → camelCase in generated TS
procedures = "camelCase"

[watch]
debounce_ms = 200
clear_screen = true
```

**Resolution order:** CLI flags > `rpc.config.toml` > built-in defaults.

Parse with the `toml` crate. Discover the config by walking up from `--dir` to the workspace root.

### Serde Attribute Support

The parser currently ignores serde attributes. This leads to a mismatch between what Rust serializes at runtime and what the generated TypeScript types describe.

#### `rename_all` on structs and enums

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserProfile {
    first_name: String,       // → firstName: string
    last_name: String,        // → lastName: string
}
```

Supported `rename_all` values: `camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `kebab-case`, `SCREAMING-KEBAB-CASE`.

#### `rename` on fields and variants

```rust
#[derive(Serialize)]
struct Config {
    #[serde(rename = "apiKey")]
    api_key: String,          // → apiKey: string
}
```

#### `skip` and `skip_serializing`

```rust
#[derive(Serialize)]
struct Internal {
    pub name: String,
    #[serde(skip)]
    secret: String,           // omitted from generated TS interface
}
```

#### `default` on fields

Fields with `#[serde(default)]` that are also `Option<T>` become optional in TypeScript:

```rust
#[derive(Serialize, Deserialize)]
struct Params {
    query: String,
    #[serde(default)]
    page: Option<u32>,        // → page?: number | null
}
```

### Expanded Primitive and Wrapper Types

| Rust                        | TypeScript               | Notes                                      |
|-----------------------------|--------------------------|--------------------------------------------|
| `HashSet<T>`, `BTreeSet<T>` | `T[]`                    | serde serializes sets as arrays            |
| `Box<T>`, `Arc<T>`, `Rc<T>` | `T`                      | transparent wrappers, unwrap to inner type |
| `Cow<'_, T>`                | `T`                      |                                            |
| `[T; N]` (fixed-size array) | `[T, T, ..., T]` (tuple) | alternatively `T[]` via config             |

Implementation: extend `RustType` enum and the `syn::Type → RustType` conversion in `parser/types.rs`, then update the TS emitter in `codegen/typescript.rs`.

---

## Phase 2 — Client

### `RpcClientConfig`

The current `createRpcClient(baseUrl)` accepts only a string. Expand it to a full configuration object:

```typescript
export interface RpcClientConfig {
  baseUrl: string;

  // Custom fetch implementation (for SSR, testing, service workers)
  fetch?: typeof globalThis.fetch;

  // Static or dynamic headers (e.g. auth tokens)
  headers?:
    | Record<string, string>
    | (() => Record<string, string> | Promise<Record<string, string>>);

  // Lifecycle hooks
  onRequest?: (ctx: RequestContext) => void | Promise<void>;
  onResponse?: (ctx: ResponseContext) => void | Promise<void>;
  onError?: (error: RpcError) => void | Promise<void>;

  // Retry policy
  retry?: {
    attempts: number;
    delay: number | ((attempt: number) => number);
    retryOn?: number[];
  };

  // Request timeout in milliseconds
  timeout?: number;

  // Custom serialization (for Date, BigInt, etc.)
  serialize?: (input: unknown) => string;
  deserialize?: (text: string) => unknown;

  // AbortController integration
  signal?: AbortSignal;
}

export function createRpcClient(config: RpcClientConfig | string): RpcClient;
```

Key use cases:
- **SSR in SvelteKit** — pass `fetch` from the `load` function so cookies and headers are forwarded.
- **Authentication** — dynamic `headers` callback that reads the current token.
- **Observability** — `onRequest`/`onResponse` hooks for logging, metrics, tracing.
- **Resilience** — automatic retries with exponential backoff for transient failures.

### Per-Call Options

Every `query` and `mutate` call accepts an optional trailing options argument:

```typescript
interface CallOptions {
  signal?: AbortSignal;
  headers?: Record<string, string>;
  timeout?: number;
}

const data = await client.query("hello", "world", {
  signal: controller.signal,
  timeout: 5000,
});
```

### Request Deduplication

When multiple components call the same query with the same input simultaneously, only one HTTP request should be made. Subsequent callers receive the same in-flight promise.

```typescript
// Both calls result in a single HTTP request
const [a, b] = await Promise.all([
  client.query("user", { id: 1 }),
  client.query("user", { id: 1 }),
]);
// a === b (same reference)
```

Implementation: maintain a `Map<string, Promise>` keyed by `procedure + serialized input`. Insert on the first call, delete it on settlement. This applies to queries only — mutations are never deduplicated.

### ~~JSDoc from Doc-Comments~~ ✅

> Implemented via `codegen.preserve_docs` config option and `--preserve-docs` CLI flag.

Forward Rust `///` doc-comments to the generated TypeScript as JSDoc:

```rust
/// User profile returned by the /me endpoint.
#[derive(Serialize)]
struct UserProfile {
    /// Display name chosen during onboarding.
    name: String,
}
```

Generated output:

```typescript
/** User profile returned by the /me endpoint. */
export interface UserProfile {
  /** Display name chosen during onboarding. */
  name: string;
}
```

Implementation: `syn` already exposes doc attributes as `#[doc = "..."]`. Collect them in `extract.rs` and thread through the model to the codegen layer.

---

## Phase 3 — Developer Experience

### Svelte Reactive Wrappers

Generate an optional `rpc-svelte.ts` file with reactive primitives built on Svelte 5 runes:

```typescript
import type { RpcClient } from './rpc-client';
import type { Procedures } from './rpc-types';

type QueryKey = keyof Procedures['queries'];
type MutationKey = keyof Procedures['mutations'];

export function createQuery<K extends QueryKey>(
  client: RpcClient,
  key: K,
  input: () => Procedures['queries'][K]['input'],
  options?: {
    enabled?: boolean;
    refetchInterval?: number;
    placeholderData?: Procedures['queries'][K]['output'];
    onSuccess?: (data: Procedures['queries'][K]['output']) => void;
    onError?: (error: RpcError) => void;
  },
): {
  readonly data: Procedures['queries'][K]['output'] | undefined;
  readonly error: RpcError | undefined;
  readonly isLoading: boolean;
  readonly isError: boolean;
  refetch: () => Promise<void>;
};

export function createMutation<K extends MutationKey>(
  client: RpcClient,
  key: K,
  options?: {
    onSuccess?: (data: Procedures['mutations'][K]['output']) => void;
    onError?: (error: RpcError) => void;
    onSettled?: () => void;
  },
): {
  mutate: (input: Procedures['mutations'][K]['input']) => Promise<void>;
  mutateAsync: (input: Procedures['mutations'][K]['input']) => Promise<Procedures['mutations'][K]['output']>;
  readonly data: Procedures['mutations'][K]['output'] | undefined;
  readonly error: RpcError | undefined;
  readonly isLoading: boolean;
};
```

This is opt-in via `codegen.client_style = "hooks"` in the config.

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

| Phase | Focus      | Key Deliverables                                                               |
|-------|------------|--------------------------------------------------------------------------------|
| **1** | Foundation | ~~Config file~~ ✅, serde attributes, expanded type support                     |
| **2** | Client     | Client config, per-call options, request deduplication, ~~JSDoc generation~~ ✅ |
| **3** | DX         | Svelte hooks, enum representations, generics, branded types, flatten           |
| **4** | Ecosystem  | External crate mappings, macro metadata, server-side caching, batch requests   |
