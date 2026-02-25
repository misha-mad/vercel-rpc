# RFC-12: Idempotent Mutations via `idempotent` Attribute

- **Status:** Proposed
- **Topic:** Safe client-side retries for mutations
- **Date:** February 2026

## 1. Summary

Add an optional `idempotent` flag to `#[rpc_mutation]` that marks a mutation as safe to retry on failure. The flag flows into the generated TypeScript client as metadata, enabling the retry logic to automatically retry idempotent mutations while skipping non-idempotent ones.

## 2. Motivation

The generated RPC client supports configurable retry via `RpcClientConfig.retry`:

```typescript
const rpc = createRpcClient({
  baseUrl: "/api",
  retry: { attempts: 3, delay: 1000 },
});
```

Currently, **all** requests (queries and mutations) are retried when retry is configured. This is correct for queries (GET requests are inherently safe to retry), but dangerous for mutations — retrying a `create_order` or `charge_payment` could cause duplicate side effects.

However, some mutations **are** safe to retry:

- `delete_item(id)` — deleting an already-deleted item is a no-op
- `set_status(id, status)` — setting the same status twice is idempotent
- `upsert_user(input)` — insert-or-update is safe to repeat

There is currently no way to express this distinction. Users must either:
1. Enable retry and accept the risk for all mutations, or
2. Disable retry globally and lose the benefit for queries and safe mutations

### Goal

Let developers mark specific mutations as `idempotent` at the Rust source level. The generated client uses this metadata to only retry mutations that are explicitly marked safe, while queries are always retryable.

## 3. Design

### 3.1 Attribute Syntax

A bare flag on `#[rpc_mutation]`:

```rust
#[rpc_mutation(idempotent)]
async fn delete_item(id: u32) -> bool { /* ... */ }

#[rpc_mutation(idempotent)]
async fn set_status(input: StatusUpdate) -> Status { /* ... */ }
```

Combined with other attributes:

```rust
#[rpc_mutation(idempotent, init = "setup")]
async fn upsert_user(input: UserInput, state: &AppState) -> User { /* ... */ }

#[rpc_mutation(idempotent, timeout = "30s")]
async fn sync_data(input: SyncInput) -> SyncResult { /* ... */ }
```

Rejected on queries (queries are inherently idempotent via GET):

```rust
// Compile error: "idempotent is only valid on mutations"
#[rpc_query(idempotent)]
async fn get_user(id: u32) -> User { /* ... */ }
```

### 3.2 Server-Side Behavior

**None.** The `idempotent` attribute is purely metadata — it does not change how the handler executes. The mutation handler generates the exact same code with or without `idempotent`. The attribute's only purpose is to flow into the generated TypeScript client.

### 3.3 Client-Side Behavior

The generated client emits an `IDEMPOTENT_MUTATIONS` set alongside `PROCEDURE_TIMEOUTS`:

```typescript
const IDEMPOTENT_MUTATIONS: Set<string> = new Set(["delete_item", "set_status"]);
```

The `rpcFetch` retry logic changes from:

```typescript
const canRetry = retryOn.includes(res.status) && attempt < maxAttempts;
```

To:

```typescript
const canRetry = retryOn.includes(res.status) && attempt < maxAttempts
  && (method === "GET" || IDEMPOTENT_MUTATIONS.has(procedure));
```

This applies to both HTTP error retries and network error retries. The effect:

| Procedure type           | Retry configured | Retried? |
|--------------------------|------------------|----------|
| Query (GET)              | Yes              | Always   |
| Mutation (idempotent)    | Yes              | Yes      |
| Mutation (not idempotent)| Yes              | Never    |
| Any                      | No               | Never    |

### 3.4 Breaking Change

This is a **behavioral change**: mutations that were previously retried will no longer be retried unless marked `idempotent`. However, the default retry config is `undefined` (no retries), so only users who explicitly configured retry are affected. Those users likely want this safety guardrail.

**Migration:** If you have `retry` configured and rely on mutation retries, add `idempotent` to those mutations:

```rust
// Before: retried automatically (unsafe)
#[rpc_mutation]
async fn upsert_user(input: UserInput) -> User { /* ... */ }

// After: explicitly marked safe to retry
#[rpc_mutation(idempotent)]
async fn upsert_user(input: UserInput) -> User { /* ... */ }
```

Mutations without the flag are never retried, even when `retry` is configured. This is the safe default — accidental duplicate side effects are worse than a missed retry.

## 4. Attribute Parsing Changes

### 4.1 Mixed Meta Types

The current parser uses `Punctuated<MetaNameValue, Comma>`, which only handles `key = "value"` pairs. The `idempotent` flag is a bare identifier (no value). The parser must be updated to handle both.

Change the parser item type from `MetaNameValue` to `Meta`, then match on each variant:

```rust
for meta in &parsed {
    match meta {
        // Bare flag: idempotent
        syn::Meta::Path(path) => {
            let key = path.get_ident().ok_or_else(|| ...)?;
            if key == "idempotent" {
                if idempotent {
                    return Err(syn::Error::new_spanned(key, "duplicate `idempotent` attribute"));
                }
                idempotent = true;
            } else {
                return Err(syn::Error::new_spanned(key, format!("unknown attribute `{key}`")));
            }
        }
        // Key-value: cache = "1h", init = "setup", timeout = "30s"
        syn::Meta::NameValue(nv) => {
            // ... existing logic ...
        }
        syn::Meta::List(_) => {
            return Err(syn::Error::new_spanned(meta, "expected `key = \"value\"` or bare flag"));
        }
    }
}
```

### 4.2 `HandlerAttrs` Extension

```rust
struct HandlerAttrs {
    cache_config: Option<CacheConfig>,
    init_fn: Option<String>,
    timeout_secs: Option<u64>,
    idempotent: bool,
}
```

### 4.3 Rejection on Queries

In `rpc_query`, after parsing attrs:

```rust
if attrs.idempotent {
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "idempotent is only valid on mutations (queries are inherently idempotent)",
    ));
}
```

## 5. CLI Changes

### 5.1 Model

Add `idempotent: bool` to `Procedure`:

```rust
pub struct Procedure {
    // ... existing fields ...
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub idempotent: bool,
}
```

### 5.2 Parser

In `extract.rs`, add `extract_idempotent()` that checks for a bare `idempotent` path in **mutation** attribute args only:

```rust
fn extract_idempotent(attrs: &[Attribute]) -> bool {
    for attr in attrs {
        if !attr.path().is_ident(RPC_MUTATION_ATTR) {
            continue;
        }
        let Ok(parsed) = attr.parse_args_with(
            Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated,
        ) else { continue; };
        for meta in &parsed {
            if let syn::Meta::Path(path) = meta
                && path.is_ident("idempotent")
            {
                return true;
            }
        }
    }
    false
}
```

> **Note:** The CLI only looks at `#[rpc_mutation]` attributes for `idempotent`. If someone writes `#[rpc_query(idempotent)]`, the CLI ignores it — it never reaches the parser because it filters on `RPC_MUTATION_ATTR`. This is consistent with the macro, which rejects `idempotent` on queries at compile time. In normal usage the macro runs first (at `cargo build`), so invalid combinations never reach the CLI. The CLI being lenient here is intentional: it scans raw source files and should not abort on code that hasn't been compiled yet (e.g. during `metaxy watch` while the user is still editing).

### 5.3 Client Codegen

#### `IDEMPOTENT_MUTATIONS` set

Emitted after `PROCEDURE_TIMEOUTS`, before `FETCH_HELPER`:

```typescript
const IDEMPOTENT_MUTATIONS: Set<string> = new Set(["delete_item", "set_status"]);
```

When no mutations are idempotent, emit an empty set rather than omitting the declaration. This keeps `rpcFetch` unconditional — it always references `IDEMPOTENT_MUTATIONS` without checking whether the const exists:

```typescript
const IDEMPOTENT_MUTATIONS: Set<string> = new Set([]);
```

#### `rpcFetch` retry guard

Update the two retry decision points in `FETCH_HELPER`:

```typescript
// HTTP error retry (line ~171)
const canRetry = retryOn.includes(res.status) && attempt < maxAttempts
  && (method === "GET" || IDEMPOTENT_MUTATIONS.has(procedure));

// Network error retry (line ~183)
const willRetry = attempt < maxAttempts
  && (method === "GET" || IDEMPOTENT_MUTATIONS.has(procedure));
```

## 6. Test Plan

### 6.1 Macro — Attribute Parsing

| Test                                       | Description                                              |
|--------------------------------------------|----------------------------------------------------------|
| `parse_attrs_idempotent_only`              | `idempotent` parses correctly, sets `idempotent: true`   |
| `parse_attrs_idempotent_with_init`         | `idempotent, init = "setup"` both parsed                 |
| `parse_attrs_idempotent_with_timeout`      | `idempotent, timeout = "30s"` both parsed                |
| `parse_attrs_idempotent_with_all`          | `idempotent, init = "setup", timeout = "30s"` all parsed |
| `parse_attrs_duplicate_idempotent`         | `idempotent, idempotent` is rejected                     |
| `parse_attrs_idempotent_rejects_value`     | `idempotent = "true"` is rejected (must be bare flag)    |
| `parse_attrs_idempotent_rejects_bool`      | `idempotent = true` (unquoted bool) is rejected          |

### 6.2 Macro — Query Rejection

| Test                                  | Description                                    |
|---------------------------------------|------------------------------------------------|
| `query_rejects_idempotent`            | `#[rpc_query(idempotent)]` is a compile error  |

### 6.3 Macro — Codegen

| Test                                  | Description                                           |
|---------------------------------------|-------------------------------------------------------|
| `mutation_idempotent_no_codegen_diff` | `idempotent` does not change generated handler code   |

### 6.4 CLI — Extraction

| Test                                  | Description                                            |
|---------------------------------------|--------------------------------------------------------|
| `extracts_idempotent_mutation`        | `#[rpc_mutation(idempotent)]` → `idempotent: true`     |
| `non_idempotent_mutation`             | `#[rpc_mutation]` → `idempotent: false`                |
| `query_idempotent_ignored`            | `#[rpc_query(idempotent)]` has no effect (CLI is lenient) |
| `idempotent_with_timeout`             | `#[rpc_mutation(idempotent, timeout = "30s")]` extracts both |

### 6.5 CLI — Client Codegen

| Test                                       | Description                                              |
|--------------------------------------------|----------------------------------------------------------|
| `idempotent_mutations_set_emitted`         | `IDEMPOTENT_MUTATIONS` contains marked procedures        |
| `idempotent_mutations_empty_when_none`     | Empty set emitted when no idempotent mutations exist     |
| `retry_guard_checks_idempotent`            | `rpcFetch` retry logic includes idempotent check         |

### 6.6 Snapshot Updates

All 6 client snapshot files will need updating due to:
- New `IDEMPOTENT_MUTATIONS` const
- Updated retry logic in `rpcFetch`

## 7. Backward Compatibility

- **Macro:** No breaking changes. `idempotent` is optional; existing code compiles identically.
- **Client:** **Behavioral change.** Mutations that were previously retried (when `config.retry` was set) will no longer be retried unless marked `idempotent`. This is intentional — it adds safety guardrails. Users who want the old behavior can mark their mutations `idempotent`.

## 8. Future Extensions

- **`idempotent` on queries** — could be extended in the future if query semantics diverge (e.g. queries with side effects). Currently unnecessary since all queries use GET.
- **`idempotencyKey`** — automatic idempotency key generation/forwarding via headers for exactly-once semantics at the server level.
