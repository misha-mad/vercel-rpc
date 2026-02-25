<div align="center">

# ⚡ vercel-rpc

**End-to-end typesafe RPC between Rust lambdas on Vercel and any TypeScript frontend**

[**Live Demo →** vercel-rpc.vercel.app](https://vercel-rpc.vercel.app)

[![CI](https://github.com/misha-mad/vercel-rpc/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/misha-mad/vercel-rpc/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/misha-mad/vercel-rpc/graph/badge.svg)](https://codecov.io/gh/misha-mad/vercel-rpc)
[![Rust Tests](https://img.shields.io/badge/rust_tests-passed-brightgreen?logo=rust)](./crates)
[![Vitest](https://img.shields.io/badge/vitest-passed-brightgreen?logo=vitest)](./demo/tests/integration)
[![Playwright](https://img.shields.io/badge/e2e-passed-brightgreen?logo=playwright)](./demo/tests/e2e)
[![TypeScript](https://img.shields.io/badge/types-auto--generated-blue?logo=typescript)](./demo/src/lib/rpc-types.ts)
[![Vercel](https://img.shields.io/badge/deploy-vercel-black?logo=vercel)](https://vercel.com)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-yellow.svg)](#license)

Write Rust functions → get a fully typed TypeScript client.

</div>

---

## Why?

Building serverless APIs with Rust on Vercel is fast — but keeping TypeScript types in sync is painful. **vercel-rpc** solves this:

- 🦀 **Write plain Rust functions** with `#[rpc_query]` / `#[rpc_mutation]`
- 🔄 **Auto-generate TypeScript types & client** from Rust source code
- ⚛️ **Framework hooks** — opt-in React, Vue 3, Svelte 5, and SolidJS reactive wrappers
- 👀 **Watch mode** — types regenerate on every save
- 🚀 **Deploy to Vercel** — each function becomes a serverless lambda
- 🛡️ **End-to-end type safety** — Rust types → TypeScript types, no manual sync

## How It Works

```
┌──────────────┐     scan     ┌─────────────┐    codegen   ┌──────────────────────┐
│  api/*.rs    │ ──────────▶  │   Manifest  │ ──────────▶  │  rpc-types.ts        │
│  #[rpc_query]│   (syn)      │  procedures │   (rust→ts)  │  rpc-client.ts       │
│  #[rpc_mut.] │              │  structs    │              │  rpc.svelte.ts (opt) │
│              │              │             │              │  rpc.react.ts  (opt) │
│              │              │             │              │  rpc.vue.ts    (opt) │
│              │              │             │              │  rpc.solid.ts  (opt) │
└──────────────┘              └─────────────┘              └──────────────────────┘
       │                                                           │
       │  deploy (vercel)                              import (ts) │
       ▼                                                           ▼
┌──────────────┐              HTTP (GET/POST)              ┌──────────────────┐
│ Vercel Lambda│ ◀───────────────────────────────────────  │  Your Frontend   │
│  /api/hello  │                                           │  rpc.query(...)  │
│  /api/time   │ ───────────────────────────────────────▶  │  fully typed! ✨ │
└──────────────┘              JSON response                └──────────────────┘
```

## Quick Start

### 1. Define a Rust lambda

```rust
// api/hello.rs
use vercel_rpc::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}
```

That's it. The macro generates a full Vercel-compatible handler with:
- Input parsing (query params for queries, JSON body for mutations)
- JSON serialization of the response
- CORS headers & OPTIONS preflight
- HTTP method validation (GET for queries, POST for mutations)
- Structured error responses for `Result<T, E>` return types

### 2. Generate TypeScript bindings

```bash
# One-time generation (from demo/)
cd demo
npm run generate

# Or directly with cargo (from project root)
cargo run -p vercel-rpc-cli -- generate --dir api --output demo/src/lib/rpc-types.ts --client-output demo/src/lib/rpc-client.ts
```

This produces two files (plus optional framework wrappers — see [Svelte 5](#svelte-5-reactive-wrappers-opt-in), [React](#react-hooks-opt-in), [Vue 3](#vue-3-composables-opt-in), and [SolidJS](#solidjs-primitives-opt-in) below):

**`src/lib/rpc-types.ts`** — type definitions:
```typescript
export interface TimeResponse {
  timestamp: number;
  message: string;
}

export type Procedures = {
  queries: {
    hello: { input: string; output: string };
    time: { input: void; output: TimeResponse };
  };
  mutations: {
  };
};
```

**`src/lib/rpc-client.ts`** — typed client with overloads:
```typescript
export interface RpcClient {
  query(key: "time"): Promise<TimeResponse>;
  query(key: "hello", input: string): Promise<string>;
}

export function createRpcClient(config: RpcClientConfig): RpcClient { /* ... */ }
```

### 3. Use in your frontend

The generated client uses standard `fetch()` and works with **any** TypeScript frontend — React, Vue, Svelte, Solid, or vanilla TS.

```typescript
import { createRpcClient } from "./rpc-client";

const rpc = createRpcClient({ baseUrl: "/api" });

const greeting = await rpc.query("hello", "World");
//                          ^ autocomplete ✨
//                                 ^ typed as string ✨
```

### 4. Watch mode (development)

```bash
cd demo
npm run dev
```

This runs the RPC watcher and Vite dev server in parallel. Every time you save a `.rs` file in `api/`, the TypeScript types and client are regenerated automatically:

```
  vercel-rpc watch mode
  api dir: api
  types:   src/lib/rpc-types.ts
  client:  src/lib/rpc-client.ts

  ✓ Generated 2 procedure(s), 1 struct(s), 0 enum(s) in 3ms
    → src/lib/rpc-types.ts
    → src/lib/rpc-client.ts
  Watching for changes in api

  [12:34:56] Changed: api/hello.rs
  ✓ Regenerated in 2ms
```

## Project Structure

See [docs/PROJECT-STRUCTURE.md](./docs/PROJECT-STRUCTURE.md) for the full annotated file tree.

## CLI Reference

### `rpc scan`

Scan Rust source files and print discovered procedures:

```bash
cargo run -p vercel-rpc-cli -- scan --dir api
```

```
Discovered 2 procedure(s), 1 struct(s), 0 enum(s):

  Query hello (String) -> String  [api/hello.rs]
  Query time (()) -> TimeResponse  [api/time.rs]

  struct TimeResponse {
    timestamp: u64,
    message: String,
  }
```

### `rpc generate`

Generate TypeScript types and client:

```bash
cargo run -p vercel-rpc-cli -- generate \
  --dir api \
  --output src/lib/rpc-types.ts \
  --client-output src/lib/rpc-client.ts \
  --types-import ./rpc-types
```

| Flag                    | Default                 | Description                                  |
|-------------------------|-------------------------|----------------------------------------------|
| `--dir`, `-d`           | `api`                   | Rust source directory                        |
| `--output`, `-o`        | `src/lib/rpc-types.ts`  | Types output path                            |
| `--client-output`, `-c` | `src/lib/rpc-client.ts` | Client output path                           |
| `--svelte-output`       | *(none)*                | Svelte 5 wrapper output path (opt-in)        |
| `--react-output`        | *(none)*                | React hooks output path (opt-in)             |
| `--vue-output`          | *(none)*                | Vue 3 composable output path (opt-in)        |
| `--solid-output`        | *(none)*                | SolidJS primitives output path (opt-in)      |
| `--types-import`        | `./rpc-types`           | Import path for types in client              |
| `--config`              | *(auto-discover)*       | Path to config file                          |
| `--branded-newtypes`    | `false`                 | Branded types for newtypes                   |
| `--bigint-type`         | *(none)*                | Map an integer type to `bigint` (repeatable) |
| `--no-config`           | `false`                 | Disable config file loading                  |

### `rpc watch`

Watch for changes and regenerate on save (same flags as `generate`):

```bash
cargo run -p vercel-rpc-cli -- watch --dir api
```

### Configuration file

Instead of passing flags every time, you can create an `rpc.config.toml` at the project root:

```toml
# rpc.config.toml — all fields are optional

[input]
dir = "api"
include = ["**/*.rs"]    # glob patterns for files to include
exclude = []             # glob patterns for files to exclude

[output]
types = "src/lib/rpc-types.ts"
client = "src/lib/rpc-client.ts"
svelte = "src/lib/rpc.svelte.ts"  # opt-in Svelte 5 wrappers
# react = "src/lib/rpc.react.ts" # opt-in React hooks
# vue   = "src/lib/rpc.vue.ts"   # opt-in Vue 3 composables
# solid = "src/lib/rpc.solid.ts" # opt-in SolidJS primitives

[output.imports]
types_path = "./rpc-types"
extension = ""               # suffix appended to import (e.g. ".js" for ESM)

[codegen]
preserve_docs = false        # forward Rust `///` doc comments as JSDoc
branded_newtypes = false     # branded types for single-field tuple structs
# bigint_types = ["i64", "u64", "i128", "u128"]  # map large integers to bigint

[codegen.naming]
fields = "preserve"          # "preserve" (default) or "camelCase"

[codegen.type_overrides]
# "chrono::DateTime" = "string"
# "uuid::Uuid" = "string"
# "serde_json::Value" = "unknown"

[watch]
debounce_ms = 200
```

The CLI auto-discovers the config by walking up from the current directory. CLI flags override config values per invocation. Use `--no-config` to ignore the file entirely.

### Preserving doc comments

Set `preserve_docs = true` in `[codegen]` to forward Rust `///` doc comments as JSDoc in the generated TypeScript. This gives you editor tooltips and inline documentation on the TypeScript side.

```rust
/// Returns the current server time.
#[rpc_query]
async fn time() -> TimeResponse { /* ... */ }

/// A timestamp with a human-readable message.
#[derive(Serialize)]
struct TimeResponse {
    timestamp: u64,
    message: String,
}
```

Generated output with `preserve_docs = true`:

```typescript
/** A timestamp with a human-readable message. */
export interface TimeResponse {
  timestamp: number;
  message: string;
}

export type Procedures = {
  queries: {
    /** Returns the current server time. */
    time: { input: void; output: TimeResponse };
  };
};
```

Doc comments are preserved on procedures, structs, and enums. Disabled by default (`preserve_docs = false`).

### Field naming

By default, struct field names are emitted as-is (`"preserve"`). Set `fields = "camelCase"` under `[codegen.naming]` to convert snake_case fields to camelCase in the generated TypeScript:

```toml
[codegen.naming]
fields = "camelCase"
```

```rust
#[derive(Serialize)]
struct ServiceStatus {
    uptime_secs: u64,
    version: String,
}
```

With `fields = "preserve"` (default):

```typescript
export interface ServiceStatus {
  uptime_secs: number;
  version: string;
}
```

With `fields = "camelCase"`:

```typescript
export interface ServiceStatus {
  uptimeSecs: number;
  version: string;
}
```

The transform also applies to struct variant fields in enums. Enum variant *names* and procedure names are not affected.

### Type overrides

Map external crate types (or any Rust type) to custom TypeScript types via `[codegen.type_overrides]`:

```toml
[codegen.type_overrides]
"chrono::DateTime" = "string"      # ISO 8601 strings
"chrono::NaiveDate" = "string"
"uuid::Uuid" = "string"
"serde_json::Value" = "unknown"
"rust_decimal::Decimal" = "string"
"url::Url" = "string"
```

Or via the CLI (repeatable):

```sh
rpc generate --type-override "chrono::DateTime=string" --type-override "uuid::Uuid=string"
```

Overrides are applied before code generation — every occurrence of the matched type (including inside `Vec<T>`, `Option<T>`, etc.) is replaced with the specified TypeScript type, and generic parameters are stripped.

**Matching:** override keys are matched against type names by their last path segment. For example, key `"chrono::DateTime"` matches both the fully-qualified `chrono::DateTime<Utc>` and the imported `DateTime<Utc>`. If you use fully-qualified paths in your Rust source, exact full-path matching takes priority.

> **Note:** The parser preserves the full type path when written explicitly (e.g. `field: chrono::DateTime<Utc>`), but imported names (via `use chrono::DateTime`) only retain the short name. Override keys always fall back to matching by the last segment, so `"chrono::DateTime" = "string"` works regardless of how the type is referenced.

### Serde attribute support

The codegen respects `#[serde(...)]` attributes so that generated TypeScript matches the actual JSON output. Supported attributes:

| Attribute                      | Level           | Effect                                                                                                                                                                  |
|--------------------------------|-----------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `rename_all = "..."`           | struct / enum   | Transforms all field or variant names (`camelCase`, `snake_case`, `PascalCase`, `SCREAMING_SNAKE_CASE`, `kebab-case`, `SCREAMING-KEBAB-CASE`, `lowercase`, `UPPERCASE`) |
| `rename = "..."`               | field / variant | Overrides the name of a single field or variant                                                                                                                         |
| `skip` / `skip_serializing`    | field           | Omits the field from the generated TypeScript interface                                                                                                                 |
| `default`                      | field           | Makes `Option<T>` fields optional: `field?: T \| null`                                                                                                                  |
| `flatten`                      | field           | Merges the field's type into the parent via TypeScript intersection (`& Type`)                                                                                          |
| `tag = "..."`                  | enum            | Internally tagged enum representation                                                                                                                                   |
| `tag = "...", content = "..."` | enum            | Adjacently tagged enum representation                                                                                                                                   |
| `untagged`                     | enum            | Untagged enum representation                                                                                                                                            |

**Priority:** field-level `rename` > container-level `rename_all` > `codegen.naming.fields` config > original name.

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UserProfile {
    display_name: String,        // → displayName: string
    #[serde(rename = "profile_url")]
    profile_url: String,         // → profile_url: string  (rename overrides rename_all)
    #[serde(skip)]
    internal_score: f64,         // omitted from TypeScript
    #[serde(default)]
    avatar_url: Option<String>,  // → avatarUrl?: string | null
}
```

Generated TypeScript:

```typescript
export interface UserProfile {
  displayName: string;
  profile_url: string;
  avatarUrl?: string | null;
}
```

Serde attributes on enums work the same way — `rename_all` transforms variant names, and per-variant `rename` overrides individual names.

### Client configuration

The generated `createRpcClient` accepts an `RpcClientConfig` object:

```typescript
import { createRpcClient } from "./rpc-client";

// Minimal — just a base URL
const rpc = createRpcClient({ baseUrl: "/api" });

// Custom fetch (e.g. SvelteKit SSR)
const rpc = createRpcClient({
  baseUrl: "/api",
  fetch: event.fetch,
});

// Static headers
const rpc = createRpcClient({
  baseUrl: "/api",
  headers: { Authorization: `Bearer ${token}` },
});

// Dynamic/async headers (e.g. rotating tokens)
const rpc = createRpcClient({
  baseUrl: "/api",
  headers: async () => {
    const token = await getAccessToken();
    return { Authorization: `Bearer ${token}` };
  },
});
```

| Option        | Type                                                                     | Description                                           |
|---------------|--------------------------------------------------------------------------|-------------------------------------------------------|
| `baseUrl`     | `string`                                                                 | Required. Base URL for RPC endpoints                  |
| `fetch`       | `typeof globalThis.fetch`                                                | Custom fetch function (SSR, testing)                  |
| `headers`     | `Record<string, string> \| () => Record<string, string> \| Promise<...>` | Static or async headers (auth tokens)                 |
| `onRequest`   | `(ctx: RequestContext) => void \| Promise<void>`                         | Hook before each fetch — can mutate headers           |
| `onResponse`  | `(ctx: ResponseContext) => void \| Promise<void>`                        | Hook after a successful response is parsed            |
| `onError`     | `(ctx: ErrorContext) => void \| Promise<void>`                           | Hook on network failure or non-ok HTTP status         |
| `retry`       | `RetryPolicy`                                                            | Retry policy (`attempts`, `delay`, `retryOn`)         |
| `timeout`     | `number`                                                                 | Per-request timeout in milliseconds                   |
| `serialize`   | `(input: unknown) => string`                                             | Custom serializer (e.g. superjson)                    |
| `deserialize` | `(text: string) => unknown`                                              | Custom deserializer                                   |
| `signal`      | `AbortSignal`                                                            | Abort signal for cancelling all requests              |
| `dedupe`      | `boolean`                                                                | Enable/disable query deduplication (default: `true`)  |

### Per-call options

Every `query()` and `mutate()` call accepts an optional trailing `CallOptions` argument to override `headers`, `timeout`, `signal`, or `dedupe` for a single request:

```typescript
// Override timeout for a slow query
const report = await rpc.query("slow_report", input, { timeout: 30_000 });

// Cancel a single request
const controller = new AbortController();
rpc.query("search", query, { signal: controller.signal });
```

### Request deduplication

Identical in-flight queries are automatically deduplicated — only one HTTP request is made and all callers share the same promise:

```typescript
// Both calls result in a single HTTP request
const [a, b] = await Promise.all([
  rpc.query("get_user", { id: 1 }),
  rpc.query("get_user", { id: 1 }),
]);
```

Dedup is on by default for queries. Disable globally via `dedupe: false` in config or per-call via `CallOptions`. Mutations are never deduplicated.

See the [rpc-cli README](./crates/rpc-cli/README.md#generated-client-features) for full details on lifecycle hooks, retry, timeout, serialization, signal, per-call options, and deduplication.

### Svelte 5 reactive wrappers (opt-in)

When `output.svelte` is configured, the CLI generates a `.svelte.ts` file with `createQuery` and `createMutation` helpers that wrap the `RpcClient` with Svelte 5 runes (`$state`, `$effect`):

```toml
# rpc.config.toml
[output]
svelte = "src/lib/rpc.svelte.ts"
```

```svelte
<script lang="ts">
  import { rpc } from '$lib/rpc';
  import { createQuery, createMutation } from '$lib/rpc.svelte';

  // Reactive query — auto-refetches when input changes
  const user = createQuery(rpc, "get_user", () => ({ id: userId }));

  // Mutation with lifecycle callbacks
  const updateName = createMutation(rpc, "update_profile", {
    onSuccess: () => alert("Saved!"),
  });
</script>

{#if user.isLoading}
  <Spinner />
{:else if user.isError}
  <ErrorBanner error={user.error} />
{:else}
  <p>Hello, {user.data.name}</p>
{/if}
```

See the [rpc-cli README](./crates/rpc-cli/README.md#svelte-5-reactive-wrappers) and [RFC-7](./docs/RFC/RFC-7.md) for full API details.

### React hooks (opt-in)

When `output.react` is configured, the CLI generates a `.ts` file with `useQuery` and `useMutation` hooks that wrap the `RpcClient` with React state (`useState`, `useEffect`):

```toml
# rpc.config.toml
[output]
react = "src/lib/rpc.react.ts"
```

```tsx
import { rpc } from './rpc';
import { useQuery, useMutation } from './rpc.react';

function UserProfile() {
  const [userId, setUserId] = useState(1);

  // Reactive query — auto-refetches when input changes
  const user = useQuery(rpc, "get_user", { id: userId });

  // Mutation with lifecycle callbacks
  const updateName = useMutation(rpc, "update_profile", {
    onSuccess: () => alert("Saved!"),
  });

  if (user.isLoading) return <Spinner />;
  if (user.isError) return <ErrorBanner error={user.error} />;
  return <p>Hello, {user.data.name}</p>;
}
```

See the [rpc-cli README](./crates/rpc-cli/README.md#react-hooks) and [RFC-8](./docs/RFC/RFC-8.md) for full API details.

### Vue 3 composables (opt-in)

When `output.vue` is configured, the CLI generates a `.ts` file with `useQuery` and `useMutation` composables that wrap the `RpcClient` with Vue 3 Composition API (`ref`, `computed`, `watch`):

```toml
# rpc.config.toml
[output]
vue = "src/lib/rpc.vue.ts"
```

```vue
<script setup lang="ts">
import { ref } from "vue";
import { rpc } from './rpc';
import { useQuery, useMutation } from './rpc.vue';

const userId = ref(1);

// Reactive query — auto-refetches when input changes
const user = useQuery(rpc, "get_user", () => ({ id: userId.value }));

// Mutation with lifecycle callbacks
const updateName = useMutation(rpc, "update_profile", {
  onSuccess: () => alert("Saved!"),
});
</script>

<template>
  <Spinner v-if="user.isLoading" />
  <ErrorBanner v-else-if="user.isError" :error="user.error" />
  <p v-else>Hello, {{ user.data?.name }}</p>
</template>
```

See the [rpc-cli README](./crates/rpc-cli/README.md#vue-3-composables) and [RFC-9](./docs/RFC/RFC-9.md) for full API details.

### SolidJS primitives (opt-in)

When `output.solid` is configured, the CLI generates a `.ts` file with `createQuery` and `createMutation` primitives that wrap the `RpcClient` with SolidJS reactivity (`createSignal`, `createEffect`, `createMemo`, `onCleanup`, `batch`):

```toml
# rpc.config.toml
[output]
solid = "src/lib/rpc.solid.ts"
```

```tsx
import { createSignal } from "solid-js";
import { rpc } from './rpc';
import { createQuery, createMutation } from './rpc.solid';

function UserProfile() {
  const [userId, setUserId] = createSignal(1);

  // Reactive query — auto-refetches when input changes
  const user = createQuery(rpc, "get_user", () => ({ id: userId() }));

  // Mutation with lifecycle callbacks
  const updateName = createMutation(rpc, "update_profile", {
    onSuccess: () => alert("Saved!"),
  });

  return (
    <Show when={!user.isLoading()} fallback={<Spinner />}>
      <Show when={!user.isError()} fallback={<ErrorBanner error={user.error()!} />}>
        <p>Hello, {user.data()!.name}</p>
      </Show>
    </Show>
  );
}
```

See the [rpc-cli README](./crates/rpc-cli/README.md#solidjs-primitives) and [RFC-10](./docs/RFC/RFC-10.md) for full API details.

## Rust Macros

### `#[rpc_query]` — GET endpoint

```rust
use vercel_rpc::rpc_query;

// No input
#[rpc_query]
async fn version() -> String {
    "1.0.0".to_string()
}

// With input (parsed from ?input= query param)
#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}

// With custom struct output
#[rpc_query]
async fn time() -> TimeResponse {
    TimeResponse { timestamp: 123, message: "now".into() }
}

// With Result return type (Err → 400 JSON error)
#[rpc_query]
async fn risky(id: u32) -> Result<Item, String> {
    if id == 0 { Err("invalid id".into()) } else { Ok(Item { id }) }
}
```

### Caching

Use the `cache` attribute on `#[rpc_query]` to generate `Cache-Control` HTTP headers. On Vercel this enables edge caching without infrastructure changes.

```rust
// CDN caches for 1 hour, browser always revalidates
#[rpc_query(cache = "1h")]
async fn get_settings() -> Settings { /* ... */ }
// → Cache-Control: public, max-age=0, s-maxage=3600

// CDN caches 5 min, serves stale up to 1 hour while revalidating
#[rpc_query(cache = "5m", stale = "1h")]
async fn get_feed() -> Vec<Post> { /* ... */ }
// → Cache-Control: public, max-age=0, s-maxage=300, stale-while-revalidate=3600

// Browser-only cache, no CDN
#[rpc_query(cache = "private, 10m")]
async fn get_profile() -> Profile { /* ... */ }
// → Cache-Control: private, max-age=600
```

Duration shorthand: `30s`, `5m`, `1h`, `1d`. The macro parses these at compile time and emits the appropriate header values. Error responses never receive cache headers. Mutations do not support caching.

### Initialization

Use the `init` attribute to run a function once at cold start. This enables logger setup, dotenv loading, DB connection pools, and HTTP clients in serverless lambdas.

**Side-effects only** (logger, env loading):

```rust
async fn setup() {
    tracing_subscriber::fmt().try_init().ok();
    dotenv::dotenv().ok();
}

#[rpc_query(init = "setup")]
async fn get_data() -> Data { /* ... */ }
```

**With state injection** (DB pool, HTTP client):

```rust
struct AppState {
    pool: sqlx::PgPool,
    http: reqwest::Client,
}

async fn setup() -> AppState {
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    AppState { pool, http: reqwest::Client::new() }
}

#[rpc_query(init = "setup")]
async fn get_user(id: u32, state: &AppState) -> User {
    state.pool.query("...").await
}
```

The init function must be `async`. State is stored in a `OnceLock` and injected as `&T` — the macro distinguishes state (`&T`) from input (`T`) by reference syntax. Parameter order does not matter.

**Combined with cache:**

```rust
#[rpc_query(init = "setup", cache = "1h")]
async fn get_user(id: u32, state: &AppState) -> User { /* ... */ }
```

Mutations support `init` (but not `cache`):

```rust
#[rpc_mutation(init = "setup")]
async fn create_order(input: OrderInput, state: &AppState) -> Order { /* ... */ }
```

### Timeout

Use the `timeout` attribute to enforce a per-procedure server-side timeout. If the handler does not complete within the specified duration, the request returns a `504` error with `"Handler timed out"`.

```rust
#[rpc_query(timeout = "30s")]
async fn slow_report(input: ReportParams) -> Report { /* ... */ }

#[rpc_mutation(timeout = "5m")]
async fn long_import(input: ImportData) -> ImportResult { /* ... */ }

// Combined with cache and init
#[rpc_query(timeout = "30s", cache = "1h", init = "setup")]
async fn expensive(id: u32, state: &AppState) -> Report { /* ... */ }
```

Duration shorthand: `30s`, `5m`, `1h`, `1d`. The timeout is also forwarded to the generated TypeScript client as a per-procedure default, slotting into the resolution chain: `callOptions?.timeout ?? PROCEDURE_TIMEOUTS[procedure] ?? config.timeout`.

### `#[rpc_mutation]` — POST endpoint

```rust
use vercel_rpc::rpc_mutation;

#[rpc_mutation]
async fn create_item(input: CreateInput) -> Item {
    // input is parsed from the JSON request body
    Item { id: 1, name: input.name }
}
```

### Enum & Struct support

Structs and enums with `#[derive(Serialize)]` are automatically picked up and converted to TypeScript:

```rust
#[derive(Serialize)]
struct UserProfile {
    name: String,
    age: u32,
}

#[derive(Serialize)]
enum Status {
    Active,
    Inactive,
    Banned,
}

#[derive(Serialize)]
enum ApiResult {
    Ok(String),                          // tuple variant
    NotFound,                            // unit variant
    Error { code: u32, message: String } // struct variant
}
```

Generated TypeScript:

```typescript
export interface UserProfile {
  name: string;
  age: number;
}

export type Status = "Active" | "Inactive" | "Banned";

export type ApiResult = { Ok: string } | "NotFound" | { Error: { code: number; message: string } };
```

#### Enum tagging strategies

All four serde enum representations are supported:

| Strategy           | Serde attribute                      | TypeScript output                |
|--------------------|--------------------------------------|----------------------------------|
| External (default) | *(none)*                             | `{ Variant: data }`              |
| Internal           | `#[serde(tag = "type")]`             | `{ type: "Variant"; ...fields }` |
| Adjacent           | `#[serde(tag = "t", content = "c")]` | `{ t: "Variant"; c: data }`      |
| Untagged           | `#[serde(untagged)]`                 | `data` (no wrapper)              |

```rust
// Internally tagged
#[derive(Serialize)]
#[serde(tag = "type")]
enum Shape {
    Circle { radius: f64 },
    Rect { w: f64, h: f64 },
}
// → { type: "Circle"; radius: number } | { type: "Rect"; w: number; h: number }

// Adjacently tagged
#[derive(Serialize)]
#[serde(tag = "t", content = "c")]
enum Event {
    Click { x: i32, y: i32 },
    Scroll(f64),
}
// → { t: "Click"; c: { x: number; y: number } } | { t: "Scroll"; c: number }

// Untagged
#[derive(Serialize)]
#[serde(untagged)]
enum StringOrInt {
    Str(String),
    Int(i32),
}
// → string | number
```

#### Generic types

Generic structs and enums produce generic TypeScript interfaces and types. Generic parameters are also preserved in procedure signatures:

```rust
#[derive(Serialize)]
struct Paginated<T> {
    items: Vec<T>,
    total: u64,
    page: u32,
}

#[rpc_query]
async fn list_users() -> Paginated<User> { /* ... */ }
```

Generated TypeScript:

```typescript
export interface Paginated<T> {
  items: T[];
  total: number;
  page: number;
}

export type Procedures = {
  queries: {
    list_users: { input: void; output: Paginated<User> };
  };
};
```

#### Tuple structs & branded newtypes

Single-field tuple structs (newtypes) produce type aliases. Multi-field tuple structs produce TypeScript tuples:

```rust
#[derive(Serialize)]
struct UserId(String);

#[derive(Serialize)]
struct Pair(String, i32);
```

```typescript
// Default:
export type UserId = string;
export type Pair = [string, number];
```

Enable `codegen.branded_newtypes` for nominal type safety on newtypes — prevents accidental mixing of structurally identical types:

```toml
[codegen]
branded_newtypes = true
```

```typescript
// With branded_newtypes = true:
export type UserId = string & { readonly __brand: "UserId" };
export type Pair = [string, number];  // multi-field tuples are unaffected
```

Also available as `--branded-newtypes` CLI flag.

### Generated handler features

Every macro-annotated function automatically gets:

| Feature             | Description                                              |
|---------------------|----------------------------------------------------------|
| **CORS**            | `Access-Control-Allow-Origin: *` on all responses        |
| **Preflight**       | `OPTIONS` → `204 No Content` with CORS headers           |
| **Method check**    | `405 Method Not Allowed` for wrong HTTP method           |
| **Input parsing**   | Query param (GET) or JSON body (POST)                    |
| **Error handling**  | `Result<T, E>` → `Ok` = 200, `Err` = 400 with JSON error |
| **Caching**         | `cache = "1h"` → `Cache-Control` header on success responses |
| **Timeout**         | `timeout = "30s"` → `504` if handler exceeds duration     |
| **Response format** | `{ "result": { "type": "response", "data": ... } }`      |

## Type Mapping

| Rust                                        | TypeScript                                                |
|---------------------------------------------|-----------------------------------------------------------|
| `String`, `&str`, `char`                    | `string`                                                  |
| `i8`..`i128`, `u8`..`u128`, `f32`, `f64`    | `number` (or `bigint` via `bigint_types`)                 |
| `bool`                                      | `boolean`                                                 |
| `()`                                        | `void`                                                    |
| `Vec<T>`, `HashSet<T>`, `BTreeSet<T>`       | `T[]`                                                     |
| `Option<T>`                                 | `T \| null`                                               |
| `HashMap<K, V>`, `BTreeMap<K, V>`           | `Record<K, V>`                                            |
| `Box<T>`, `Arc<T>`, `Rc<T>`, `Cow<T>`       | `T` (transparent wrappers)                                |
| `(A, B, C)`                                 | `[A, B, C]`                                               |
| `[T; N]`                                    | `T[]`                                                     |
| `Result<T, E>`                              | `T` (error handled at runtime)                            |
| Custom structs                              | `interface` with same fields                              |
| Newtype structs (`UserId(String)`)          | `type UserId = string` (or branded)                       |
| Tuple structs (`Pair(A, B)`)                | `type Pair = [A, B]`                                      |
| Generic structs (`Paginated<T>`)            | `interface Paginated<T>` (generic preserved)              |
| Enums (unit variants)                       | `"A" \| "B" \| "C"` (string union)                        |
| Enums (tuple variants)                      | `{ A: string } \| { B: number }` (tagged union)           |
| Enums (struct variants)                     | `{ A: { x: number; y: number } }` (tagged union)          |
| Enums (mixed)                               | Combination of all above                                  |
| Type overrides (`[codegen.type_overrides]`) | Configurable mapping (e.g. `chrono::DateTime` → `string`) |

## npm Scripts

See CONTRIBUTING.md for development scripts and setup instructions.

## Testing

Detailed testing strategy and commands are described in CONTRIBUTING.md.

## Deploy to Vercel

Each `.rs` file in `api/` becomes a serverless function at `/api/<name>`.

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy
vercel
```

> **Note:** The demo uses `demo/` as the Vercel Root Directory. In your own project, place `api/` at your project root or configure the Root Directory in Vercel's project settings accordingly.

## Sponsors

<div align="center">
  <em>You could be the first sponsor! ❤️</em>
</div>

<p align="center">If you find this project useful, consider <a href="https://github.com/sponsors/misha-mad">sponsoring</a> to support its development.</p>

## License

MIT OR Apache-2.0

---

<sub>This project is not affiliated with or endorsed by Vercel Inc. "Vercel" is a trademark of Vercel Inc.</sub>
