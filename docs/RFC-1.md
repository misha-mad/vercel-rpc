# RFC-1: VRS-RPC (Vercel-Rust-SvelteKit RPC)

- **Status:** Draft
- **Author:** OSS Maintainer / Lead Architect
- **Topic:** Type-safe, file-based RPC for Rust Lambdas on Vercel
- **Date:** February 2026

## 1. Executive Summary

VRS-RPC is a lightweight, zero-overhead RPC framework designed specifically for projects using SvelteKit and Rust Lambdas hosted on Vercel.

Unlike monolithic RPC solutions (like rspc), VRS-RPC treats every Rust file in the `api/` directory as an independent serverless function. It provides a seamless Developer Experience (DX) by using static analysis to synchronize Rust types with a TypeScript client in real-time.

## 2. Motivation

The current implementation using rspc introduces several bottlenecks:

- **Monolithic Router:** All requests hit a single lambda, increasing cold start impact and complicating Vercel's routing.
- **Runtime Overhead:** Heavy dependency on a centralized router logic.
- **DX Friction:** Lack of a native "watch-and-generate" mode that fits perfectly into the SvelteKit/Vite ecosystem without full Rust recompilation.

## 3. Architecture & Components

### A. The `rpc-macro` (Transformation Layer)

A procedural macro crate that handles the boilerplate of serverless communication.

- **Responsibility:** Transforms a standard `async fn` into a `vercel_runtime::run` entry point with CORS, method validation, input parsing, and JSON response wrapping.
- **Pure code generation:** The macro operates entirely at compile time via AST transformation. It does **not** emit any metadata or manifest files — type extraction is handled independently by `rpc-cli`.

### B. The `rpc-cli` (Code Generation & Watcher)

A CLI tool written in Rust that bridges the gap between the Backend and Frontend.

- **Static Analysis:** Uses the `syn` crate to parse `api/*.rs` files without executing them. Extracts function signatures, struct definitions, and enum variants directly from the AST in memory.
- **Watch Mode:** Uses the `notify` + `notify-debouncer-mini` crates to detect changes in Rust files (200ms debounce) and instantly regenerate TypeScript definitions.
- **Output:** Generates `rpc-types.ts` (interfaces, enum union types, `Procedures` map) and `rpc-client.ts` (typed fetch wrapper with `RpcError` class).

## 4. Proposed Technical Specification

### Workspace Structure

```
vercel-rpc/
├── crates/
│   ├── rpc-macro/              # Proc-macro: #[rpc_query] / #[rpc_mutation]
│   │   └── src/lib.rs
│   └── rpc-cli/                # CLI binary: scan / generate / watch
│       └── src/
│           ├── main.rs         # CLI entry (clap)
│           ├── model.rs        # Manifest, Procedure, RustType, StructDef, EnumDef
│           ├── parser/         # Rust source -> Manifest (via syn)
│           │   ├── extract.rs  # File scanning & procedure extraction
│           │   └── types.rs    # syn::Type -> RustType conversion
│           ├── codegen/        # Manifest -> TypeScript
│           │   ├── typescript.rs  # RustType -> TS type mapping + rpc-types.ts
│           │   └── client.rs      # RpcClient interface + rpc-client.ts
│           └── watch.rs        # File watcher with debounce (notify-debouncer-mini)
├── demo/                       # SvelteKit demo application + Rust lambdas
│   ├── api/                    # Every file is a standalone Vercel Lambda
│   │   ├── hello.rs            # #[rpc_query]  — String -> String
│   │   ├── time.rs             # #[rpc_query]  — () -> TimeResponse
│   │   ├── status.rs           # #[rpc_query]  — () -> ServiceStatus (with enum)
│   │   ├── math.rs             # #[rpc_query]  — MathInput -> Result<MathResult, String>
│   │   ├── stats.rs            # #[rpc_query]  — Vec<f64> -> Result<Stats, String>
│   │   └── echo.rs             # #[rpc_mutation] — EchoInput -> EchoOutput
│   ├── src/
│   │   ├── lib/
│   │   │   ├── rpc-types.ts    # AUTO-GENERATED: Type definitions
│   │   │   ├── rpc-client.ts   # AUTO-GENERATED: Typed fetch client + RpcError
│   │   │   └── client.ts       # MANUAL: RpcClient instance configuration
│   │   └── routes/             # SvelteKit pages
│   ├── tests/
│   │   ├── integration/        # Vitest: codegen pipeline tests
│   │   └── e2e/                # Playwright: UI + API tests
│   ├── Cargo.toml              # Rust package for demo lambdas
│   └── package.json            # Vite/SvelteKit scripts
├── Cargo.toml                  # Workspace root (members: rpc-macro, rpc-cli, demo)
└── vercel.json                 # Vercel deployment config
```

### Rust Syntax Example

```rust
// api/hello.rs
use vercel_rpc_macro::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
```

### Generated TypeScript Client

```typescript
// rpc-types.ts
export type Procedures = {
  queries: {
    hello: { input: string; output: string };
  };
  mutations: {};
};

// rpc-client.ts
export class RpcError extends Error { ... }
export function createRpcClient(baseUrl: string): RpcClient;

// Usage in SvelteKit component
import { rpc } from '$lib/client';
const res = await rpc.query("hello", "World");
```

## 5. Implementation Roadmap

| Phase   | Task             | Deliverable                                                    |
|---------|------------------|----------------------------------------------------------------|
| Phase 1 | CLI Parser       | A tool that extracts `fn` signatures using `syn`.              |
| Phase 2 | TS Generator     | Mapping Rust primitives/structs/enums to TS types.             |
| Phase 3 | Watch Mode       | `rpc-cli watch` with `notify-debouncer-mini` (200ms debounce). |
| Phase 4 | Proc-Macro       | `#[rpc_query]` / `#[rpc_mutation]` attribute logic.            |
| Phase 5 | Integration      | `npm run dev` orchestration via shell background (`&`).        |

## 6. Design Decisions & Trade-offs

- **Static Analysis over Runtime Reflection:** By parsing Rust files with `syn`, we generate types instantly without waiting for `cargo build`. The macro and the CLI are fully independent — the macro transforms code, the CLI analyzes it.
- **File-Based Routing:** We lean into Vercel's native architecture. Each procedure is its own lambda, allowing for better scaling and granular logs.
- **Type Sharing:** While `specta` is an option for complex types, the initial version focuses on `serde`-compatible structures for simplicity.
