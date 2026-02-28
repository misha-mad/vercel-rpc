# RFC-011: Cold-Start Initialization via `init` Attribute

- **Status:** Proposed
- **Topic:** Handler initialization and shared state injection
- **Date:** February 2026

## 1. Summary

Add an optional `init` parameter to `#[rpc_query]` and `#[rpc_mutation]` that specifies a function to run once at cold start. The init function can return a state value that is stored in a `OnceLock` static and injected into the handler as a parameter.

## 2. Motivation

Currently the macro generates a complete `main()` function with no user control over cold-start initialization. This blocks common serverless patterns:

```rust
// Users CANNOT do this today:
fn main() {
    tracing_subscriber::init();        // logger
    dotenv::dotenv().ok();             // env loading
    let pool = Pool::new("...");       // DB connection pool
    let client = reqwest::Client::new(); // HTTP client
    // ... then run handler
}
```

### What works today

Connection pools and HTTP clients can use `std::sync::LazyLock`:

```rust
static DB: LazyLock<Pool> = LazyLock::new(|| Pool::new("..."));

#[rpc_query]
async fn get_user(id: u32) -> User {
    DB.query("SELECT ...").await  // initialized on first call
}
```

### What doesn't work

Early setup that must run **before** the first request — logger initialization, panic hooks, dotenv loading — cannot be placed in a `LazyLock`. There is no way to inject code into the generated `main()`.

### Goal

Provide a single, composable mechanism that handles both early side-effects and state initialization, without exposing `main()` or adding boilerplate.

## 3. Design

### 3.1 `init` Attribute

A new optional `init` parameter on both `#[rpc_query]` and `#[rpc_mutation]`:

```rust
#[rpc_query(init = "setup")]
async fn get_data() -> Data { ... }
```

The value is a path to a function that runs once before `vercel_runtime::run`. Compatible with `cache`:

```rust
#[rpc_query(init = "setup", cache = "1h")]
async fn get_data() -> Data { ... }
```

### 3.2 Init Function Signatures

Init functions must be `async fn`. This is required because the macro always generates `.await` on the init call inside `block_on`. Sync code works fine inside async functions with zero overhead, and the primary use case (DB pools) requires async anyway.

**Side-effects only** (no state):

```rust
async fn setup() {
    tracing_subscriber::fmt().try_init().ok();  // try_init — safe if called twice (e.g. tests)
    dotenv::dotenv().ok();
}
```

**With state:**

```rust
async fn setup() -> AppState {
    tracing_subscriber::fmt().try_init().ok();
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").unwrap()).await.unwrap();
    AppState { pool, http: reqwest::Client::new() }
}
```

> **Note on panics:** If `setup()` panics (e.g. `.unwrap()` on a failed DB connect), the process exits before `vercel_runtime::run` starts. Vercel will see a non-zero exit code with a bare backtrace and no HTTP context. For better diagnostics, prefer returning `Result` and logging before panicking, or use `.expect("descriptive message")`.

### 3.3 State Injection

When the init function returns a value, the handler receives it as a parameter typed `&State`. The macro detects the state parameter by its type syntax:

> **Convention:** state is always `&T` (a reference), input is always an owned type `T`. This is how the macro distinguishes them — no ambiguity.

```rust
#[rpc_query(init = "setup")]
async fn get_user(id: u32, state: &AppState) -> User {
//                 ^^^^^          ^^^^^^^^
//                 owned → input  &ref → state
    state.pool.query("SELECT ...").await
}
```

This convention is enforced: a `&T` parameter (that is not `Headers`) is only accepted when `init` is present. Without `init`, a `&T` parameter is a compile error. Parameter order does not matter:

```rust
// All of these work:
async fn handler(input: Query, state: &AppState) -> Data { ... }
async fn handler(state: &AppState, input: Query) -> Data { ... }
async fn handler(state: &AppState) -> Data { ... }  // no input
```

### 3.4 Generated Code

Since init functions are always `async`, the macro always generates `.await` inside `block_on`. One consistent code path, no asyncness detection needed.

**With state** — `async fn setup() -> AppState`:

```rust
static __RPC_STATE: std::sync::OnceLock<AppState> = std::sync::OnceLock::new();

fn main() -> Result<(), Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            __RPC_STATE.set(setup().await).expect("BUG: OnceLock already set");
            vercel_runtime::run(service_fn(__rpc_handler)).await
        })
}

async fn __rpc_handler(req: Request) -> Result<Response<Value>, Error> {
    let state = __RPC_STATE.get().expect("BUG: init not called");
    // ... parse input ...
    let __raw_result = get_user(__input, state).await;
    // ...
}
```

**Side-effects only** — `async fn setup()`:

```rust
fn main() -> Result<(), Error> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            setup().await;
            vercel_runtime::run(service_fn(__rpc_handler)).await
        })
}
```

### 3.5 Shared Initialization Across Lambdas

On Vercel, each `api/*.rs` file becomes a separate lambda binary. Only files in `api/` are deployed as lambdas — shared code lives outside:

```
api/
  get_data.rs       → lambda /api/get_data
  get_user.rs       → lambda /api/get_user
  create_order.rs   → lambda /api/create_order
src/
  lib.rs            → shared code (NOT a lambda)
  state.rs
```

```rust
// src/lib.rs
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub http: reqwest::Client,
}

pub async fn setup() -> AppState {
    tracing_subscriber::fmt().try_init().ok();
    let pool = sqlx::PgPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("failed to connect to database");
    AppState { pool, http: reqwest::Client::new() }
}
```

```rust
// api/get_user.rs
use my_crate::{AppState, setup};

#[rpc_query(init = "setup")]
async fn get_user(id: u32, state: &AppState) -> User {
    state.pool.query("...").await
}
```

```rust
// api/create_order.rs
use my_crate::{AppState, setup};

#[rpc_mutation(init = "setup")]
async fn create_order(input: OrderInput, state: &AppState) -> Order {
    state.pool.query("...").await
}
```

Each lambda calls `setup()` independently at its own cold start. The init code is shared at the source level, not at runtime.

## 4. Attribute Parsing Changes

### 4.1 `rpc_query`

`parse_cache_attrs` is extended to also accept `init`:

```rust
#[rpc_query]                              // no init, no cache
#[rpc_query(cache = "1h")]                // cache only
#[rpc_query(init = "setup")]              // init only
#[rpc_query(init = "setup", cache = "1h")] // both
```

### 4.2 `rpc_mutation`

`rpc_mutation` gains the same `init` parameter:

```rust
#[rpc_mutation]                           // no init (existing)
#[rpc_mutation(init = "setup")]           // with init
```

Note: `cache` remains query-only. Mutations reject `cache` and `stale`.

### 4.3 Rename `parse_cache_attrs`

With `init` support, `parse_cache_attrs` becomes `parse_handler_attrs` and returns a broader `HandlerAttrs` struct:

```rust
struct HandlerAttrs {
    cache_config: Option<CacheConfig>,
    init_fn: Option<String>,
}
```

## 5. `build_handler` Changes

### 5.1 State Type Detection

The macro classifies each parameter by its type syntax:

| Type syntax         | Classification | Example            |
|---------------------|----------------|--------------------|
| `Headers` (by name) | Headers param  | `headers: Headers` |
| `&T` (reference)    | State param    | `state: &AppState` |
| `T` (owned)         | Input param    | `id: u32`          |

This is unambiguous — a parameter's role is determined entirely by its type, not by position or name. The `&T` convention for state is natural: the handler borrows from the `OnceLock`, it doesn't own the state.

```rust
// In build_handler, after separating typed_params:
let mut state_param = None;

for param in &typed_params {
    if is_headers_type(&param.ty) {
        headers_param = Some(*param);
    } else if is_ref_type(&param.ty) {
        state_param = Some(*param);
    } else {
        input_param = Some(*param);
    }
}
```

Where `is_ref_type` checks for `&T` syntax (shared, immutable reference only):

```rust
fn is_ref_type(ty: &Type) -> bool {
    matches!(ty, Type::Reference(r) if r.mutability.is_none())
}
```

`&mut T` is explicitly rejected — `OnceLock` only provides shared access:

```rust
if let Type::Reference(r) = &param.ty {
    if r.mutability.is_some() {
        return Err(syn::Error::new_spanned(
            &param.ty,
            "state parameter must be a shared reference (&T), not &mut T",
        ));
    }
}
```

A `&T` parameter without `init` is also a compile error — this prevents silent misclassification.

### 5.2 Conditional Code Generation

| `init_fn`    | State param | Generated code                                                               |
|--------------|-------------|------------------------------------------------------------------------------|
| `None`       | —           | Current behavior (no change)                                                 |
| `Some(path)` | `None`      | Call `path()` in `main()` before runtime, no `OnceLock`                      |
| `Some(path)` | `Some(&T)`  | `OnceLock<T>` static, `path()` stored in `main()`, `state` passed to handler |

### 5.3 Error Cases

| Condition                 | Error                                                                |
|---------------------------|----------------------------------------------------------------------|
| `&T` param without `init` | `"state parameter requires init = \"...\" attribute"`                |
| `&mut T` param            | `"state parameter must be a shared reference (&T), not &mut T"`      |
| `init` path is empty      | `"init function path cannot be empty"`                               |
| Multiple `&T` params      | `"RPC handlers accept at most one state parameter"`                  |
| Multiple owned params     | `"RPC handlers accept at most one input parameter"` (existing error) |

## 6. Supported Parameter Combinations

The handler can have up to three parameters in any order. Each parameter is classified by type syntax, not by position or name:

| Type syntax         | Role               | Max count |
|---------------------|--------------------|-----------|
| `Headers` (by name) | HTTP headers       | 1         |
| `&T` (shared ref)   | State from `init`  | 1         |
| `T` (owned)         | Deserialized input | 1         |

Valid combinations:

| Parameters              | Example                                                             |
|-------------------------|---------------------------------------------------------------------|
| None                    | `async fn health() -> Status`                                       |
| Input only              | `async fn get(id: u32) -> User`                                     |
| State only              | `async fn list(state: &AppState) -> Vec<Item>`                      |
| Headers only            | `async fn auth(headers: Headers) -> Token`                          |
| Input + State           | `async fn get(id: u32, state: &AppState) -> User`                   |
| Input + Headers         | `async fn get(id: u32, headers: Headers) -> User`                   |
| State + Headers         | `async fn list(state: &AppState, headers: Headers) -> Vec<Item>`    |
| Input + State + Headers | `async fn get(id: u32, state: &AppState, headers: Headers) -> User` |

Multiple owned parameters are rejected — `async fn get(a: u32, b: String)` produces `"RPC handlers accept at most one input parameter"` (existing behavior, unchanged).

## 7. CLI Impact

**None.** The `rpc-cli` scanner (`detect_rpc_kind`) already ignores attribute arguments — it only checks the attribute path. No changes needed.

## 8. Test Plan

### Unit Tests — Attribute Parsing

Tested via `parse_handler_attrs_inner(proc_macro2::TokenStream)`, same pattern as existing `parse_cache_attrs_inner` tests.

| Test                                   | Description                                        |
|----------------------------------------|----------------------------------------------------|
| `parse_attrs_init_only`                | `init = "setup"` parses correctly                  |
| `parse_attrs_init_and_cache`           | `init = "setup", cache = "1h"` parses both         |
| `parse_attrs_init_empty_rejected`      | `init = ""` is an error                            |
| `parse_attrs_init_non_string_rejected` | `init = 42` is an error                            |
| `parse_attrs_duplicate_init_rejected`  | `init = "a", init = "b"` is an error               |
| `mutation_rejects_cache_with_init`     | `#[rpc_mutation(init = "s", cache = "1h")]` errors |

### Unit Tests — Handler Generation

Tested via `build_handler(func, kind, attrs)` with generated token stream assertions.

| Test                            | Description                                         |
|---------------------------------|-----------------------------------------------------|
| `init_side_effects_only`        | Generates `.await` inside `block_on`, no `OnceLock` |
| `init_with_state`               | Creates `OnceLock`, passes `&state` to handler      |
| `init_with_state_and_input`     | State and input both passed correctly               |
| `init_with_state_and_headers`   | State and headers both passed correctly             |
| `init_with_all_three_params`    | Input, state, and headers all work                  |
| `init_compatible_with_cache`    | `init` + `cache` both generate correctly            |
| `init_compatible_with_mutation` | `init` works on `#[rpc_mutation]`                   |
| `init_call_inside_block_on`     | Init call is inside `block_on` (supports async)     |

### Unit Tests — Error Cases

These test `build_handler` returning `Err(syn::Error)` — macro expansion errors, caught at compile time.

| Test                             | Description                                       |
|----------------------------------|---------------------------------------------------|
| `state_without_init_rejected`    | `&T` param without `init` attr → error            |
| `mut_state_rejected`             | `&mut T` param → `"must be a shared reference"`   |
| `multiple_state_params_rejected` | Two `&T` params → `"at most one state parameter"` |

## 9. Backward Compatibility

- **No breaking changes.** `init` is entirely optional.
- Existing `#[rpc_query]` and `#[rpc_mutation]` without `init` work identically.
- Existing `#[rpc_query(cache = "1h")]` is unaffected.
- `parse_cache_attrs` is renamed to `parse_handler_attrs` (internal, no public API change).

## 10. Future Extensions

- **`timeout` attribute** — per-procedure timeout, parsed alongside `init` and `cache`.
- **`idempotent` attribute** — marks mutations as safe to retry, flows into the generated client.
