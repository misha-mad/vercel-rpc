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

## Usage

### Simple query (GET)

```rust
use metaxy::rpc_query;

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
use metaxy::rpc_query;

#[rpc_query]
async fn version() -> String {
    "1.0.0".to_string()
}
```

When there are no parameters, the handler does not require the `input` query
parameter.

### Mutation (POST)

```rust
use metaxy::rpc_mutation;
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

Mutations read input from the **JSON request body** instead of query parameters,
and only accept **POST** requests.

### Returning `Result`

```rust
use metaxy::rpc_query;

#[rpc_query]
async fn find_user(id: u32) -> Result<String, String> {
    if id == 0 {
        Err("user not found".into())
    } else {
        Ok(format!("user_{}", id))
    }
}
```

When the function returns `Result<T, E>`:
- `Ok(value)` is serialized as a normal success response (HTTP 200)
- `Err(error)` is serialized as a JSON error response (HTTP 400)

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
- Each function accepts **at most one** parameter. More than one parameter
  produces a compile error.
- Input types must implement `serde::Deserialize`.
- Output types (and `Ok` types in `Result`) must implement `serde::Serialize`.

## Related crates

- [`metaxy-cli`](https://crates.io/crates/metaxy-cli) — CLI that scans
  `#[rpc_query]` / `#[rpc_mutation]` functions and generates TypeScript types
  and a typed client.

## License

MIT OR Apache-2.0
