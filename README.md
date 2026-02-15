<div align="center">

# ⚡ vercel-rpc

**End-to-end typesafe RPC between Rust lambdas on Vercel and SvelteKit**

[![Rust Tests](https://img.shields.io/badge/rust_tests-60_passed-brightgreen?logo=rust)](./crates)
[![Vitest](https://img.shields.io/badge/vitest-12_passed-brightgreen?logo=vitest)](./tests/integration)
[![Playwright](https://img.shields.io/badge/e2e-8_passed-brightgreen?logo=playwright)](./tests/e2e)
[![TypeScript](https://img.shields.io/badge/types-auto--generated-blue?logo=typescript)](./src/lib/rpc-types.ts)
[![Vercel](https://img.shields.io/badge/deploy-vercel-black?logo=vercel)](https://vercel.com)
[![License: MIT](https://img.shields.io/badge/license-MIT-yellow.svg)](#license)

Write Rust functions → get a fully typed TypeScript client. Zero config, zero boilerplate.

</div>

---

## Why?

Building serverless APIs with Rust on Vercel is fast — but keeping TypeScript types in sync is painful. **vercel-rpc** solves this:

- 🦀 **Write plain Rust functions** with `#[rpc_query]` / `#[rpc_mutation]`
- 🔄 **Auto-generate TypeScript types & client** from Rust source code
- 👀 **Watch mode** — types regenerate on every save
- 🚀 **Deploy to Vercel** — each function becomes a serverless lambda
- 🛡️ **End-to-end type safety** — Rust types → TypeScript types, no manual sync

## How It Works

```
┌─────────────┐     scan      ┌─────────────┐    codegen    ┌──────────────────┐
│  api/*.rs    │ ──────────▶  │   Manifest   │ ──────────▶  │  rpc-types.ts    │
│  #[rpc_query]│   (syn)      │  procedures  │   (rust→ts)  │  rpc-client.ts   │
│  #[rpc_mut.] │              │  structs     │              │  Typed RpcClient │
└─────────────┘              └─────────────┘              └──────────────────┘
       │                                                           │
       │  deploy (vercel)                          import (svelte) │
       ▼                                                           ▼
┌─────────────┐              HTTP (GET/POST)       ┌──────────────────┐
│ Vercel Lambda│ ◀──────────────────────────────── │   SvelteKit App  │
│  /api/hello  │                                   │  rpc.query(...)  │
│  /api/time   │ ──────────────────────────────▶  │  fully typed! ✨  │
└─────────────┘              JSON response         └──────────────────┘
```

## Quick Start

### 1. Define a Rust lambda

```rust
// api/hello.rs
use vercel_rpc_macro::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}
```

That's it. The macro generates the full Vercel-compatible handler with:
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

This produces two files:

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

export function createRpcClient(baseUrl: string): RpcClient;
```

### 3. Use in SvelteKit

```typescript
// demo/src/lib/client.ts
import { createRpcClient } from "./rpc-client";
export const rpc = createRpcClient("/api");
```

```svelte
<!-- demo/src/routes/+page.svelte -->
<script lang="ts">
  import { rpc } from "$lib/client";

  let greeting = $state("");

  async function sayHello() {
    greeting = await rpc.query("hello", "World");
    //                  ^ autocomplete ✨
    //                         ^ typed as string ✨
  }
</script>

<button onclick={sayHello}>Say Hello</button>
<p>{greeting}</p>
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

```
svelte-rust/
├── crates/
│   ├── rpc-macro/                # Proc-macro crate
│   │   └── src/lib.rs            #   #[rpc_query] / #[rpc_mutation]
│   └── rpc-cli/                  # CLI crate (binary: `rpc`)
│       └── src/
│           ├── main.rs           #   CLI entry (scan / generate / watch)
│           ├── model.rs          #   Manifest, Procedure, RustType, StructDef, EnumDef
│           ├── parser/           #   Rust source → Manifest (via syn)
│           │   ├── extract.rs    #     File scanning & procedure extraction
│           │   └── types.rs      #     syn::Type → RustType conversion
│           ├── codegen/          #   Manifest → TypeScript
│           │   ├── typescript.rs #     RustType → TS type mapping + rpc-types.ts
│           │   └── client.rs     #     RpcClient interface + rpc-client.ts
│           └── watch.rs          #   File watcher with debounce
├── demo/                         # SvelteKit demo application + Rust lambdas
│   ├── api/                      # Rust lambdas (each file = one endpoint)
│   │   ├── hello.rs              #   GET /api/hello?input="name"
│   │   └── time.rs               #   GET /api/time
│   ├── Cargo.toml                # Rust package for demo lambdas
│   ├── src/
│   │   ├── lib/
│   │   │   ├── rpc-types.ts      # ← auto-generated types
│   │   │   ├── rpc-client.ts     # ← auto-generated client
│   │   │   └── client.ts         #   RPC client instance (manual)
│   │   └── routes/               # SvelteKit pages
│   ├── tests/
│   │   ├── integration/          # Vitest: codegen pipeline tests
│   │   └── e2e/                  # Playwright: UI + API tests
│   ├── package.json              # Node scripts
│   ├── svelte.config.js          # SvelteKit config
│   ├── vite.config.ts            # Vite config + API mock plugin
│   └── tsconfig.json             # TypeScript config
├── Cargo.toml                    # Rust workspace (crates + demo)
├── vercel.json                   # Vercel config
└── README.md
```

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

| Flag | Default | Description |
|------|---------|-------------|
| `--dir`, `-d` | `api` | Rust source directory |
| `--output`, `-o` | `src/lib/rpc-types.ts` | Types output path |
| `--client-output`, `-c` | `src/lib/rpc-client.ts` | Client output path |
| `--types-import` | `./rpc-types` | Import path for types in client |

### `rpc watch`

Watch for changes and regenerate on save (same flags as `generate`):

```bash
cargo run -p vercel-rpc-cli -- watch --dir api
```

## Rust Macros

### `#[rpc_query]` — GET endpoint

```rust
use vercel_rpc_macro::rpc_query;

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

### `#[rpc_mutation]` — POST endpoint

```rust
use vercel_rpc_macro::rpc_mutation;

#[rpc_mutation]
async fn create_item(input: CreateInput) -> Item {
    // input is parsed from JSON request body
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

### Generated handler features

Every macro-annotated function automatically gets:

| Feature | Description |
|---------|-------------|
| **CORS** | `Access-Control-Allow-Origin: *` on all responses |
| **Preflight** | `OPTIONS` → `204 No Content` with CORS headers |
| **Method check** | `405 Method Not Allowed` for wrong HTTP method |
| **Input parsing** | Query param (GET) or JSON body (POST) |
| **Error handling** | `Result<T, E>` → `Ok` = 200, `Err` = 400 with JSON error |
| **Response format** | `{ "result": { "type": "response", "data": ... } }` |

## Type Mapping

| Rust | TypeScript |
|------|-----------|
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
| Enums (unit variants) | `"A" \| "B" \| "C"` (string union) |
| Enums (tuple variants) | `{ A: string } \| { B: number }` (tagged union) |
| Enums (struct variants) | `{ A: { x: number; y: number } }` (tagged union) |
| Enums (mixed) | Combination of all above |

## npm Scripts

All npm scripts run from the `demo/` directory:

```bash
cd demo
npm install
```

| Script | Description |
|--------|-------------|
| `npm run dev` | Watch mode + Vite dev server |
| `npm run build` | Generate types + Vite build |
| `npm run generate` | One-time TypeScript generation |
| `npm run test` | Rust unit tests + Vitest integration |
| `npm run test:e2e` | Playwright browser tests |
| `npm run test:rust` | Rust tests only |
| `npm run test:all` | Full test suite (Rust + Vitest + Playwright) |

## Testing

The project has **80 tests** across three layers:

```bash
# Run everything
npm run test:all
```

| Layer | Count | What's tested |
|-------|-------|---------------|
| **Rust unit** | 60 | Type parsing, struct/enum extraction, TS codegen, client codegen |
| **Vitest integration** | 12 | Full codegen pipeline, TypeScript compilation, idempotency |
| **Playwright e2e** | 8 | UI rendering, typed queries, API response format |

## Deploy to Vercel

Since the SvelteKit demo lives in `demo/`, you need to configure Vercel's **Root Directory**:

1. Go to your Vercel project → **Settings** → **General**
2. Set **Root Directory** to `demo`
3. Vercel will auto-detect SvelteKit and run `npm run build` from `demo/`
4. Rust lambdas in `demo/api/` are compiled as serverless functions automatically

```bash
# Install Vercel CLI
npm i -g vercel

# Deploy (set root directory on first deploy)
vercel
```

> **Note:** With Root Directory set to `demo`, Vercel detects `demo/api/` as the serverless functions directory. So `demo/api/hello.rs` → `/api/hello`.

Each `.rs` file in `api/` becomes a serverless function at `/api/<name>`.

## Tech Stack

| Component | Technology |
|-----------|-----------|
| **Frontend** | SvelteKit 2, TypeScript 5 |
| **Backend** | Rust, Vercel Runtime |
| **Macros** | `syn`, `quote`, `proc-macro2` |
| **CLI parser** | `syn` (AST), `clap` (args) |
| **File watching** | `notify` + `notify-debouncer-mini` |
| **Testing** | `cargo test`, Vitest, Playwright |
| **Deploy** | Vercel (serverless Rust) |

## License

MIT

---

<sub>This project is not affiliated with or endorsed by Vercel Inc. "Vercel" is a trademark of Vercel Inc.</sub>
