# RFC-11: Cold-Start Initialization via `init` Attribute

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

**Side-effects only** (no state):

```rust
fn setup() {
    tracing_subscriber::init();
    dotenv::dotenv().ok();
}
```

**With state** (returns a value):

```rust
fn setup() -> AppState {
    tracing_subscriber::init();
    AppState {
        pool: Pool::new(&std::env::var("DATABASE_URL").unwrap()),
        http: reqwest::Client::new(),
    }
}
```

### 3.3 State Injection

When the init function returns a value, the handler receives it as a parameter typed `&State`. The macro detects the state parameter by matching its type against the init function's return type.

```rust
struct AppState {
    pool: Pool,
    http: reqwest::Client,
}

fn setup() -> AppState {
    tracing_subscriber::init();
    AppState {
        pool: Pool::new("postgres://..."),
        http: reqwest::Client::new(),
    }
}

#[rpc_query(init = "setup")]
async fn get_user(id: u32, state: &AppState) -> User {
    state.pool.query("SELECT ...").await
}
```

State detection uses the same pattern as the existing `Headers` parameter — the macro checks whether a parameter's type matches the init function's return type by name. Parameter order does not matter:

```rust
// All of these work:
async fn handler(input: Query, state: &AppState) -> Data { ... }
async fn handler(state: &AppState, input: Query) -> Data { ... }
async fn handler(state: &AppState) -> Data { ... }  // no input
```

### 3.4 Generated Code

For `init = "setup"` where `setup() -> AppState`:

```rust
static __RPC_STATE: std::sync::OnceLock<AppState> = std::sync::OnceLock::new();

fn main() -> Result<(), Error> {
    __RPC_STATE.set(setup()).expect("init already called");

    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            vercel_runtime::run(service_fn(__rpc_handler)).await
        })
}

async fn __rpc_handler(req: Request) -> Result<Response<Value>, Error> {
    let state = __RPC_STATE.get().expect("init not called");
    // ... parse input ...
    let __raw_result = get_user(__input, state).await;
    // ...
}
```

For side-effects only (`setup()` with no return value / returns `()`):

```rust
fn main() -> Result<(), Error> {
    setup();  // no OnceLock needed

    tokio::runtime::Builder::new_current_thread()
        // ...
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
    pub pool: Pool,
    pub http: reqwest::Client,
}

pub fn setup() -> AppState {
    tracing_subscriber::init();
    AppState {
        pool: Pool::new(&std::env::var("DATABASE_URL").unwrap()),
        http: reqwest::Client::new(),
    }
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

When `init_fn` is `Some(path)`, the macro needs to know the return type to:
1. Determine whether to create a `OnceLock<T>` static
2. Match the state parameter in the handler signature

**Approach:** The state parameter is detected by exclusion — any `&T` parameter that is not `Headers` and not the input type is treated as the state reference. The `T` is used as the `OnceLock` type parameter.

```rust
// In build_handler, after separating typed_params:
let mut state_param = None;

for param in &typed_params {
    if is_headers_type(&param.ty) {
        headers_param = Some(*param);
    } else if is_ref_type(&param.ty) && init_fn.is_some() {
        state_param = Some(*param);
    } else {
        input_param = Some(*param);
    }
}
```

Where `is_ref_type` checks for `&T` syntax:

```rust
fn is_ref_type(ty: &Type) -> bool {
    matches!(ty, Type::Reference(_))
}
```

### 5.2 Conditional Code Generation

| `init_fn` | State param | Generated code |
|-----------|-------------|----------------|
| `None` | — | Current behavior (no change) |
| `Some(path)` | `None` | Call `path()` in `main()` before runtime, no `OnceLock` |
| `Some(path)` | `Some(&T)` | `OnceLock<T>` static, `path()` stored in `main()`, `state` passed to handler |

### 5.3 Error Cases

| Condition | Error |
|-----------|-------|
| State param without `init` | `"state parameter requires init = \"...\" attribute"` |
| `init` path is empty | `"init function path cannot be empty"` |
| Multiple state params | `"RPC handlers accept at most one state parameter"` |
| State param is not a reference | `"state parameter must be a reference (&T)"` |

## 6. Supported Parameter Combinations

The handler can have up to three parameters in any order:

| Parameters | Example |
|------------|---------|
| None | `async fn health() -> Status` |
| Input only | `async fn get(id: u32) -> User` |
| State only | `async fn list(state: &AppState) -> Vec<Item>` |
| Headers only | `async fn auth(headers: Headers) -> Token` |
| Input + State | `async fn get(id: u32, state: &AppState) -> User` |
| Input + Headers | `async fn get(id: u32, headers: Headers) -> User` |
| State + Headers | `async fn list(state: &AppState, headers: Headers) -> Vec<Item>` |
| Input + State + Headers | `async fn get(id: u32, state: &AppState, headers: Headers) -> User` |

## 7. CLI Impact

**None.** The `rpc-cli` scanner (`detect_rpc_kind`) already ignores attribute arguments — it only checks the attribute path. No changes needed.

## 8. Test Plan

### Unit Tests — Attribute Parsing

| Test | Description |
|------|-------------|
| `parse_attrs_init_only` | `init = "setup"` parses correctly |
| `parse_attrs_init_and_cache` | `init = "setup", cache = "1h"` parses both |
| `parse_attrs_init_empty_rejected` | `init = ""` is an error |
| `parse_attrs_init_non_string_rejected` | `init = 42` is an error |
| `parse_attrs_duplicate_init_rejected` | `init = "a", init = "b"` is an error |
| `mutation_rejects_cache_with_init` | `#[rpc_mutation(init = "s", cache = "1h")]` errors |

### Unit Tests — Handler Generation

| Test | Description |
|------|-------------|
| `init_side_effects_only` | Calls init fn in main, no OnceLock |
| `init_with_state` | Creates OnceLock, passes state to handler |
| `init_with_state_and_input` | State and input both passed correctly |
| `init_with_state_and_headers` | State and headers both passed correctly |
| `init_with_all_three_params` | Input, state, and headers all work |
| `state_without_init_rejected` | `&T` param without `init` attr is an error |
| `init_compatible_with_cache` | `init` + `cache` both generate correctly |
| `init_compatible_with_mutation` | `init` works on `#[rpc_mutation]` |

## 9. Backward Compatibility

- **No breaking changes.** `init` is entirely optional.
- Existing `#[rpc_query]` and `#[rpc_mutation]` without `init` work identically.
- Existing `#[rpc_query(cache = "1h")]` is unaffected.
- `parse_cache_attrs` is renamed to `parse_handler_attrs` (internal, no public API change).

## 10. Future Extensions

- **Async init** — `async fn setup() -> AppState` for async initialization (e.g. running migrations). Would require the init call to be inside the tokio runtime block.
- **`timeout` attribute** — per-procedure timeout, parsed alongside `init` and `cache`.
- **`idempotent` attribute** — marks mutations as safe to retry, flows into the generated client.
