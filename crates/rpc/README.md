# vercel-rpc

[![Crates.io](https://img.shields.io/crates/v/vercel-rpc.svg)](https://crates.io/crates/vercel-rpc)
[![docs.rs](https://docs.rs/vercel-rpc/badge.svg)](https://docs.rs/vercel-rpc)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/vercel-rpc.svg)](https://github.com/misha-mad/vercel-rpc/blob/main/LICENSE-MIT)

End-to-end typesafe RPC between Rust lambdas on Vercel and any TypeScript frontend.

This is the main entry point for the project — a facade crate that re-exports
the [`vercel-rpc-macro`](https://crates.io/crates/vercel-rpc-macro) proc macros
together with all runtime dependencies they need.

## Installation

```toml
[dependencies]
vercel-rpc = "0.1"
serde = { version = "1", features = ["derive"] }
```

That's it — no need to add `vercel_runtime`, `serde_json`, `tokio`, `url`, or
`http-body-util` manually.

## Usage

### Query (GET)

```rust
use vercel_rpc::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
```

The macro generates a full Vercel handler that:
- Reads `name` from the `?input=<JSON>` query parameter
- Returns `{ "result": { "type": "response", "data": "Hello, World!" } }`
- Responds to `OPTIONS` with CORS preflight headers
- Rejects non-GET methods with `405`

### Query without input

```rust
use vercel_rpc::rpc_query;

#[rpc_query]
async fn version() -> String {
    "1.0.0".to_string()
}
```

### Mutation (POST)

```rust
use vercel_rpc::rpc_mutation;
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUserInput {
    name: String,
    email: String,
}

#[rpc_mutation]
async fn create_user(input: CreateUserInput) -> String {
    format!("Created user {}", input.name)
}
```

Mutations read input from the **JSON request body** and only accept **POST**.

### Returning `Result`

```rust
use vercel_rpc::rpc_query;

#[rpc_query]
async fn find_user(id: u32) -> Result<String, String> {
    if id == 0 {
        Err("user not found".into())
    } else {
        Ok(format!("user_{}", id))
    }
}
```

- `Ok(value)` → HTTP 200 success response
- `Err(error)` → HTTP 400 error response

## Response format

**Success** (HTTP 200):
```json
{
  "result": {
    "type": "response",
    "data": <value>
  }
}
```

**Error** (HTTP 400):
```json
{
  "error": {
    "type": "error",
    "message": "<error description>"
  }
}
```

## Constraints

- Functions **must** be `async`.
- Each function accepts **at most one** parameter.
- Input types must implement `serde::Deserialize`.
- Output types (and `Ok` types in `Result`) must implement `serde::Serialize`.

## Re-exports

For convenience, the crate re-exports `serde::{Serialize, Deserialize}`, so you
can write `use vercel_rpc::Serialize;` if you prefer.

## Related crates

- [`vercel-rpc-macro`](https://crates.io/crates/vercel-rpc-macro) — the proc
  macros themselves (re-exported by this crate).
- [`vercel-rpc-cli`](https://crates.io/crates/vercel-rpc-cli) — CLI that scans
  your `#[rpc_query]` / `#[rpc_mutation]` functions and generates TypeScript
  types and a typed client.

## License

MIT OR Apache-2.0
