# vercel-rpc-cli

[![Crates.io](https://img.shields.io/crates/v/vercel-rpc-cli.svg)](https://crates.io/crates/vercel-rpc-cli)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/vercel-rpc-cli.svg)](https://github.com/misha-mad/vercel-rpc/blob/main/LICENSE-MIT)

CLI that scans Rust lambda source files annotated with `#[rpc_query]` /
`#[rpc_mutation]` and generates TypeScript type definitions and a fully typed
RPC client.

Part of the [vercel-rpc](https://github.com/misha-mad/vercel-rpc) project.

## Installation

```bash
cargo install vercel-rpc-cli
```

This installs the `rpc` binary.

## Commands

### `rpc scan`

Parse Rust source files and print discovered procedures, structs, and enums:

```bash
rpc scan --dir api
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

Also outputs a JSON manifest for tooling consumption.

### `rpc generate`

Generate TypeScript types and a typed client from Rust source files:

```bash
rpc generate \
  --dir api \
  --output src/lib/rpc-types.ts \
  --client-output src/lib/rpc-client.ts \
  --types-import ./rpc-types
```

This produces two files:

**`rpc-types.ts`** — TypeScript interfaces and a `Procedures` type map:

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
  mutations: {};
};
```

**`rpc-client.ts`** — a typed `RpcClient` with method overloads:

```typescript
export interface RpcClient {
  query(key: "time"): Promise<TimeResponse>;
  query(key: "hello", input: string): Promise<string>;
}

export function createRpcClient(config: RpcClientConfig): RpcClient;
```

### `rpc watch`

Watch for `.rs` file changes and regenerate automatically (same flags as
`generate`):

```bash
rpc watch --dir api
```

```
  vercel-rpc watch mode
  api dir: api
  types:   src/lib/rpc-types.ts
  client:  src/lib/rpc-client.ts

  ✓ Generated 2 procedure(s), 1 struct(s) in 3ms
    → src/lib/rpc-types.ts
    → src/lib/rpc-client.ts
  Watching for changes in api
```

Changes are debounced (200 ms by default, configurable via `rpc.config.toml`).
Press Ctrl+C to stop.

Use `--clear-screen` to clear the terminal before each regeneration cycle:

```bash
rpc watch --dir api --clear-screen
```

## Configuration

The CLI can be configured with an optional `rpc.config.toml` file. Place it at your project root (next to `Cargo.toml` or `package.json`). All fields are optional — defaults match the CLI flags below.

```toml
# rpc.config.toml

[input]
dir = "api"                          # Rust source directory to scan
include = ["**/*.rs"]                # glob patterns for files to include
exclude = []                         # glob patterns for files to exclude

[output]
types = "src/lib/rpc-types.ts"       # generated types file path
client = "src/lib/rpc-client.ts"     # generated client file path

[output.imports]
types_path = "./rpc-types"           # import specifier used in client file
extension = ""                       # suffix appended to import (e.g. ".js" for ESM)

[codegen]
preserve_docs = false                # forward Rust `///` doc comments as JSDoc

[codegen.naming]
fields = "preserve"                  # "preserve" (default) or "camelCase"

[watch]
debounce_ms = 200                    # file watcher debounce interval (ms)
clear_screen = false                 # clear terminal before each regeneration
```

`include` and `exclude` accept glob patterns matched against file paths relative to `dir`. A file must match at least one `include` pattern and no `exclude` pattern to be scanned. When both match, `exclude` wins.

### Preserving doc comments

When `preserve_docs = true` in `[codegen]`, Rust `///` doc comments are forwarded as JSDoc (`/** ... */`) in the generated TypeScript files. This is useful for editor tooltips and documentation.

Given this Rust source:

```rust
/// Returns the current server time.
#[rpc_query]
async fn time() -> TimeResponse {
    // ...
}

/// A timestamp with a human-readable message.
#[derive(Serialize)]
struct TimeResponse {
    timestamp: u64,
    message: String,
}

/// Possible request statuses.
#[derive(Serialize)]
enum Status {
    Active,
    Inactive,
}
```

With `preserve_docs = true`, the generated `rpc-types.ts` includes:

```typescript
/** A timestamp with a human-readable message. */
export interface TimeResponse {
  timestamp: number;
  message: string;
}

/** Possible request statuses. */
export type Status = "Active" | "Inactive";

export type Procedures = {
  queries: {
    /** Returns the current server time. */
    time: { input: void; output: TimeResponse };
  };
  mutations: {};
};
```

And the generated `rpc-client.ts` includes JSDoc on overloads:

```typescript
export interface RpcClient {
  /** Returns the current server time. */
  query(key: "time"): Promise<TimeResponse>;
}
```

Multi-line doc comments are preserved as multi-line JSDoc:

```rust
/// Greet a user by name.
/// Returns a personalized greeting string.
#[rpc_query]
async fn hello(name: String) -> String { /* ... */ }
```

```typescript
/**
 * Greet a user by name.
 * Returns a personalized greeting string.
 */
export interface RpcClient {
  // ...
}
```

With `preserve_docs = false` (the default), doc comments are silently ignored and no JSDoc is emitted.

### Field naming

The `[codegen.naming]` section controls how struct field names appear in the generated TypeScript.

| Value                  | Behavior                        | Example                       |
|------------------------|---------------------------------|-------------------------------|
| `"preserve"` (default) | Keep Rust field names as-is     | `uptime_secs` → `uptime_secs` |
| `"camelCase"`          | Convert snake_case to camelCase | `uptime_secs` → `uptimeSecs`  |

```toml
[codegen.naming]
fields = "camelCase"
```

Given this Rust source:

```rust
#[derive(Serialize)]
struct ServiceStatus {
    uptime_secs: u64,
    api_version: String,
}

#[derive(Serialize)]
enum Event {
    Click { page_x: i32, page_y: i32 },
}
```

With `fields = "preserve"` (default):

```typescript
export interface ServiceStatus {
  uptime_secs: number;
  api_version: string;
}

export type Event = { Click: { page_x: number; page_y: number } };
```

With `fields = "camelCase"`:

```typescript
export interface ServiceStatus {
  uptimeSecs: number;
  apiVersion: string;
}

export type Event = { Click: { pageX: number; pageY: number } };
```

The transform applies to struct interface fields and struct variant fields in enums. Enum variant names and procedure names are not affected.

### Config discovery

The CLI walks up from the current directory looking for `rpc.config.toml`. If no file is found, built-in defaults are used.

```bash
# Use a specific config file
rpc generate --config ./custom-config.toml

# Disable config file loading entirely
rpc generate --no-config --dir api
```

### Resolution order

Values are resolved with this priority (highest first):

```
CLI flag  >  rpc.config.toml  >  built-in default
```

A config file sets project-level defaults; CLI flags override them per invocation.

## Flags

| Flag              | Short | Default                 | Commands              | Description                                           |
|-------------------|-------|-------------------------|-----------------------|-------------------------------------------------------|
| `--dir`           | `-d`  | `api`                   | scan, generate, watch | Rust source directory to scan                         |
| `--include`       |       | `**/*.rs`               | scan, generate, watch | Glob pattern for files to include (repeatable)        |
| `--exclude`       |       | *(none)*                | scan, generate, watch | Glob pattern for files to exclude (repeatable)        |
| `--output`        | `-o`  | `src/lib/rpc-types.ts`  | generate, watch       | Output path for TypeScript types                      |
| `--client-output` | `-c`  | `src/lib/rpc-client.ts` | generate, watch       | Output path for TypeScript client                     |
| `--types-import`  |       | `./rpc-types`           | generate, watch       | Import path for types in the client file              |
| `--extension`     |       | `""`                    | generate, watch       | Suffix appended to types import (e.g. `.js` for ESM)  |
| `--preserve-docs` |       | `false`                 | generate, watch       | Forward Rust doc comments as JSDoc                    |
| `--fields`        |       | `preserve`              | generate, watch       | Field naming: `preserve` or `camelCase`               |
| `--debounce-ms`   |       | `200`                   | watch                 | File watcher debounce interval in milliseconds        |
| `--clear-screen`  |       | `false`                 | watch                 | Clear terminal before each regeneration               |
| `--config`        |       | *(auto-discover)*       | *(global)*            | Path to config file                                   |
| `--no-config`     |       | `false`                 | *(global)*            | Disable config file loading                           |

## What gets scanned

The parser recognizes:

- **Functions** annotated with `#[rpc_query]` or `#[rpc_mutation]` — extracted
  as RPC procedures with their input/output types.
- **Structs** with `#[derive(Serialize)]` — converted to TypeScript interfaces.
- **Enums** with `#[derive(Serialize)]` — converted to TypeScript union types
  (unit variants become string literals, tuple/struct variants become tagged
  objects).

## Type mapping

| Rust                                     | TypeScript                       |
|------------------------------------------|----------------------------------|
| `String`, `&str`, `char`                 | `string`                         |
| `i8`..`i128`, `u8`..`u128`, `f32`, `f64` | `number`                         |
| `bool`                                   | `boolean`                        |
| `()`                                     | `void`                           |
| `Vec<T>`, `HashSet<T>`, `BTreeSet<T>`    | `T[]`                            |
| `Option<T>`                              | `T \| null`                      |
| `HashMap<K, V>`, `BTreeMap<K, V>`        | `Record<K, V>`                   |
| `Box<T>`, `Arc<T>`, `Rc<T>`, `Cow<T>`    | `T` (transparent wrappers)       |
| `(A, B, C)`                              | `[A, B, C]`                      |
| `[T; N]`                                 | `T[]`                            |
| `Result<T, E>`                           | `T` (error handled at runtime)   |
| Custom structs                           | `interface` with same fields     |
| Enums (unit variants)                    | `"A" \| "B"`                     |
| Enums (tuple variants)                   | `{ A: string } \| { B: number }` |
| Enums (struct variants)                  | `{ A: { x: number } }`           |

## Generated client features

The generated `rpc-client.ts` includes:

- **`RpcClient` interface** with typed overloads for every procedure — full
  autocomplete and type checking.
- **`createRpcClient(config)`** factory function accepting `RpcClientConfig` with `baseUrl`, optional `fetch`, `headers`, lifecycle hooks, retry, and timeout.
- **`RpcError` class** with `status` and `data` fields for structured error
  handling.
- **`rpcFetch` helper** — uses `GET` with `?input=<JSON>` for queries and
  `POST` with JSON body for mutations. Unwraps the `result.data` envelope
  automatically.

### Lifecycle hooks

`RpcClientConfig` supports three optional hooks that run at different stages of each request:

| Hook         | When it runs                                    | Context type      |
|--------------|-------------------------------------------------|-------------------|
| `onRequest`  | Before the fetch call — can mutate headers      | `RequestContext`  |
| `onResponse` | After a successful response is parsed           | `ResponseContext` |
| `onError`    | On network failure or non-ok HTTP status        | `ErrorContext`    |

All hooks can be synchronous or return a `Promise`.

**Context types:**

```typescript
interface RequestContext {
  procedure: string;
  method: "GET" | "POST";
  url: string;
  headers: Record<string, string>;  // mutable — changes apply to the request
  input?: unknown;
}

interface ResponseContext {
  procedure: string;
  method: "GET" | "POST";
  url: string;
  response: Response;
  data: unknown;
  duration: number;  // milliseconds
}

interface ErrorContext {
  procedure: string;
  method: "GET" | "POST";
  url: string;
  error: unknown;  // RpcError for HTTP errors, native Error for network failures
  attempt: number;   // 1-based attempt number
  willRetry: boolean; // whether the request will be retried
}
```

**Example — logging and auth token:**

```typescript
const client = createRpcClient({
  baseUrl: "/api",
  onRequest(ctx) {
    ctx.headers["Authorization"] = `Bearer ${getToken()}`;
  },
  onResponse(ctx) {
    console.log(`${ctx.procedure} completed in ${ctx.duration}ms`);
  },
  onError(ctx) {
    if (!ctx.willRetry) {
      console.error(`${ctx.procedure} failed on attempt ${ctx.attempt}:`, ctx.error);
    }
  },
});
```

### Retry

Automatic retries are configured via `retry`:

```typescript
interface RetryPolicy {
  attempts: number;                            // max retries (excluding initial request)
  delay: number | ((attempt: number) => number); // fixed ms or backoff function
  retryOn?: number[];                          // HTTP status codes (default: [408, 429, 500, 502, 503, 504])
}
```

A request is retried when a network error occurs or the response status is in `retryOn`, up to `attempts` additional tries. On each retry the full `onRequest` hook runs again, so dynamic headers (e.g. refreshed auth tokens) are re-evaluated.

```typescript
// Fixed delay
const client = createRpcClient({
  baseUrl: "/api",
  retry: { attempts: 3, delay: 1000 },
});

// Exponential backoff: 1s, 2s, 4s
const client = createRpcClient({
  baseUrl: "/api",
  retry: { attempts: 3, delay: (n) => 1000 * 2 ** (n - 1) },
});
```

### Timeout

Per-request timeout in milliseconds. Uses `AbortController` internally — the request is aborted if it doesn't complete within the limit. Timeout applies to each individual attempt when combined with retry.

```typescript
const client = createRpcClient({
  baseUrl: "/api",
  timeout: 10_000,
});
```

## Related crates

- [`vercel-rpc-macro`](https://crates.io/crates/vercel-rpc-macro) — procedural
  macros (`#[rpc_query]`, `#[rpc_mutation]`) that generate Vercel lambda
  handlers from plain async functions.

## License

MIT OR Apache-2.0
