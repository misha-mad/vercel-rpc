# metaxy-macro

[![Crates.io](https://img.shields.io/crates/v/metaxy-macro.svg)](https://crates.io/crates/metaxy-macro)
[![docs.rs](https://docs.rs/metaxy-macro/badge.svg)](https://docs.rs/metaxy-macro)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/metaxy-macro.svg)](https://github.com/misha-mad/metaxy/blob/main/LICENSE-MIT)

Procedural macros that turn plain async Rust functions into
[Vercel](https://vercel.com) serverless lambda handlers with JSON serialization,
CORS, and error handling — all in one attribute.

Part of the [metaxy](https://github.com/misha-mad/metaxy) project.

## Installation

Use the [`metaxy`](https://crates.io/crates/metaxy) facade crate which
re-exports these macros together with all runtime dependencies:

```toml
[dependencies]
metaxy = "0.1"
serde = { version = "1", features = ["derive"] }
```

## Quick Start

```rust
use metaxy::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
```

## Documentation

Full documentation: **[metaxy-demo.vercel.app/docs](https://metaxy-demo.vercel.app/docs/getting-started)**

- [Procedures](https://metaxy-demo.vercel.app/docs/procedures/queries) — queries, mutations, streaming
- [Macro Attributes](https://metaxy-demo.vercel.app/docs/macros/attributes) — cache, stale, init, timeout, idempotent

## License

MIT OR Apache-2.0
