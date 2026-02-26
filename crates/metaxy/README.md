# metaxy

[![Crates.io](https://img.shields.io/crates/v/metaxy.svg)](https://crates.io/crates/metaxy)
[![docs.rs](https://docs.rs/metaxy/badge.svg)](https://docs.rs/metaxy)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/metaxy.svg)](https://github.com/misha-mad/metaxy/blob/main/LICENSE-MIT)

End-to-end typesafe RPC between Rust lambdas on Vercel and any TypeScript frontend.

This is the main entry point — a facade crate that re-exports
[`metaxy-macro`](https://crates.io/crates/metaxy-macro) proc macros
together with all runtime dependencies.

## Quick Start

```toml
[dependencies]
metaxy = "0.1"
serde = { version = "1", features = ["derive"] }
```

```rust
use metaxy::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
```

## Documentation

Full documentation: **[metaxy-demo.vercel.app/docs](https://metaxy-demo.vercel.app/docs/getting-started)**

## Related crates

- [`metaxy-macro`](https://crates.io/crates/metaxy-macro) — the proc macros
- [`metaxy-cli`](https://crates.io/crates/metaxy-cli) — TypeScript codegen CLI

## License

MIT OR Apache-2.0
