# vercel-rpc-macro

[![Crates.io](https://img.shields.io/crates/v/vercel-rpc-macro.svg)](https://crates.io/crates/vercel-rpc-macro)
[![docs.rs](https://docs.rs/vercel-rpc-macro/badge.svg)](https://docs.rs/vercel-rpc-macro)
[![codecov](https://codecov.io/gh/misha-mad/vercel-rpc/graph/badge.svg?flag=rpc-macro)](https://codecov.io/gh/misha-mad/vercel-rpc)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/vercel-rpc-macro.svg)](https://github.com/misha-mad/vercel-rpc/blob/main/LICENSE-MIT)

Procedural macros that turn plain async Rust functions into
[Vercel](https://vercel.com) serverless lambda handlers with JSON serialization,
CORS, and error handling — all in one attribute.

Part of the [vercel-rpc](https://github.com/misha-mad/vercel-rpc) project.

## Installation

The macros generate code that depends on several runtime crates.
Add **all** of the following to your `Cargo.toml` — they are **not** pulled in
transitively by `vercel-rpc-macro`:

```toml
[dependencies]
vercel-rpc-macro = "0.1"

# Runtime dependencies required by the generated code
vercel_runtime = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros"] }
url = "2"
http-body-util = "0.1"
```

## Usage

### Simple query (GET)

```rust
use vercel_rpc_macro::rpc_query;

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
use vercel_rpc_macro::rpc_query;

#[rpc_query]
async fn version() -> String {
    "1.0.0".to_string()
}
```

When there are no parameters, the handler does not require the `input` query
parameter.

### Mutation (POST)

```rust
use vercel_rpc_macro::rpc_mutation;
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
use vercel_rpc_macro::rpc_query;

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

- [`vercel-rpc-cli`](https://crates.io/crates/vercel-rpc-cli) — CLI that scans
  `#[rpc_query]` / `#[rpc_mutation]` functions and generates TypeScript types
  and a typed client.

## License

MIT OR Apache-2.0
