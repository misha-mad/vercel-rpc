//! CLI tool for the [vercel-rpc](https://github.com/misha-mad/vercel-rpc) project.
//!
//! Scans Rust lambda source files for `#[rpc_query]` / `#[rpc_mutation]`
//! functions and `#[derive(Serialize)]` types, then generates TypeScript type
//! definitions and a fully typed RPC client.
//!
//! # Binary
//!
//! The installed binary is called `rpc` and provides three subcommands:
//!
//! - **`rpc scan`** — parse a directory and print discovered procedures as
//!   human-readable text plus a JSON manifest.
//! - **`rpc generate`** — produce `rpc-types.ts` (interfaces + `Procedures`
//!   type) and `rpc-client.ts` (typed `RpcClient` + `createRpcClient` factory).
//! - **`rpc watch`** — same as `generate`, but re-runs automatically whenever
//!   a `.rs` file changes (configurable debounce).
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────┐  scan   ┌──────────┐  codegen  ┌────────────────┐
//! │  api/*.rs   │ ──────► │ Manifest │ ────────► │ rpc-types.ts   │
//! │  attributes │  (syn)  │          │ (fmt)     │ rpc-client.ts  │
//! └─────────────┘         └──────────┘           └────────────────┘
//! ```
//!
//! - [`parser`] — walks the source directory, parses each `.rs` file with
//!   `syn`, and builds a [`model::Manifest`].
//! - [`codegen::typescript`] — converts the manifest into a `rpc-types.ts`
//!   file with TypeScript interfaces, enum types, and a `Procedures` map.
//! - [`codegen::client`] — converts the manifest into a `rpc-client.ts` file
//!   with a typed `RpcClient` interface and `createRpcClient` factory.
//! - [`watch`] — wraps `generate` in a file-watcher loop with debouncing.

pub mod codegen;
pub mod commands;
pub mod config;
pub mod model;
pub mod parser;
pub mod watch;
