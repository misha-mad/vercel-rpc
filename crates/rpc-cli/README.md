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

export function createRpcClient(baseUrl: string): RpcClient;
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

[watch]
debounce_ms = 200                    # file watcher debounce interval (ms)
```

`include` and `exclude` accept glob patterns matched against file paths relative to `dir`. A file must match at least one `include` pattern and no `exclude` pattern to be scanned. When both match, `exclude` wins.

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

| Flag | Short | Default | Description |
|------|-------|---------|-------------|
| `--dir` | `-d` | `api` | Rust source directory to scan |
| `--output` | `-o` | `src/lib/rpc-types.ts` | Output path for TypeScript types |
| `--client-output` | `-c` | `src/lib/rpc-client.ts` | Output path for TypeScript client |
| `--types-import` | | `./rpc-types` | Import path for types in the client file |
| `--config` | | *(auto-discover)* | Path to config file |
| `--no-config` | | `false` | Disable config file loading |

## What gets scanned

The parser recognizes:

- **Functions** annotated with `#[rpc_query]` or `#[rpc_mutation]` — extracted
  as RPC procedures with their input/output types.
- **Structs** with `#[derive(Serialize)]` — converted to TypeScript interfaces.
- **Enums** with `#[derive(Serialize)]` — converted to TypeScript union types
  (unit variants become string literals, tuple/struct variants become tagged
  objects).

## Type mapping

| Rust | TypeScript |
|------|------------|
| `String`, `&str`, `char` | `string` |
| `i8`..`i128`, `u8`..`u128`, `f32`, `f64` | `number` |
| `bool` | `boolean` |
| `()` | `void` |
| `Vec<T>` | `T[]` |
| `Option<T>` | `T \| null` |
| `HashMap<K, V>`, `BTreeMap<K, V>` | `Record<K, V>` |
| `(A, B, C)` | `[A, B, C]` |
| `Result<T, E>` | `T` (error handled at runtime) |
| Custom structs | `interface` with same fields |
| Enums (unit variants) | `"A" \| "B"` |
| Enums (tuple variants) | `{ A: string } \| { B: number }` |
| Enums (struct variants) | `{ A: { x: number } }` |

## Generated client features

The generated `rpc-client.ts` includes:

- **`RpcClient` interface** with typed overloads for every procedure — full
  autocomplete and type checking.
- **`createRpcClient(baseUrl)`** factory function.
- **`RpcError` class** with `status` and `data` fields for structured error
  handling.
- **`rpcFetch` helper** — uses `GET` with `?input=<JSON>` for queries and
  `POST` with JSON body for mutations. Unwraps the `result.data` envelope
  automatically.

## Related crates

- [`vercel-rpc-macro`](https://crates.io/crates/vercel-rpc-macro) — procedural
  macros (`#[rpc_query]`, `#[rpc_mutation]`) that generate Vercel lambda
  handlers from plain async functions.

## License

MIT OR Apache-2.0
