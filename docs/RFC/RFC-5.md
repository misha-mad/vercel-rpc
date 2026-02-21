# RFC-5: Per-Call Options

- **Status:** Implemented
- **Topic:** Optional per-call `signal`, `headers`, and `timeout` for `query()` and `mutate()`
- **Date:** February 2026

## 1. Summary

Add an optional trailing `CallOptions` argument to every `query()` and `mutate()` overload. This lets callers override `signal`, `headers`, and `timeout` on a per-request basis without creating a separate client instance.

## 2. Motivation

`RpcClientConfig` (RFC-4) sets global defaults for the entire client — timeout, headers, abort signal, etc. But real applications need per-call granularity:

| Scenario                                                | Global config insufficient because…             |
|---------------------------------------------------------|-------------------------------------------------|
| Cancel a single long-polling query on component unmount | `config.signal` aborts *all* in-flight requests |
| Add an idempotency key to one mutation                  | `config.headers` applies to every call          |
| Allow a slow report to take 30s while most calls use 5s | `config.timeout` is one value                   |

The roadmap already describes this feature under Phase 2 — Client. Every major RPC/HTTP client (tRPC, ky, ofetch) supports per-call overrides.

## 3. Design Principles

1. **Non-breaking** — the new argument is optional. Every existing call site compiles unchanged.
2. **Merge, don't replace** — per-call headers merge with (and override) client-level headers. Per-call signal is combined with client-level signal via `AbortSignal.any()`.
3. **Minimal surface** — only the three fields that make sense per-call. Hooks, retry, and serialization stay client-level.
4. **Zero new dependencies** — uses platform APIs only.

## 4. `CallOptions` Interface

```typescript
export interface CallOptions {
  /** Override or extend headers for this single call. */
  headers?: Record<string, string>;
  /** Timeout in ms for this single call (overrides client-level timeout). */
  timeout?: number;
  /** AbortSignal for cancelling this single call. */
  signal?: AbortSignal;
}
```

## 5. Updated Overload Signatures

### 5.1 Queries

```typescript
export interface RpcClient {
  // Void-input query — options only
  query(key: "time"): Promise<TimeResponse>;
  query(key: "time", options: CallOptions): Promise<TimeResponse>;

  // Non-void-input query — input required, options optional
  query(key: "hello", input: string): Promise<string>;
  query(key: "hello", input: string, options: CallOptions): Promise<string>;
}
```

### 5.2 Mutations

```typescript
export interface RpcClient {
  // Void-input mutation
  mutate(key: "reset"): Promise<boolean>;
  mutate(key: "reset", options: CallOptions): Promise<boolean>;

  // Non-void-input mutation
  mutate(key: "create_item", input: CreateInput): Promise<Item>;
  mutate(key: "create_item", input: CreateInput, options: CallOptions): Promise<Item>;
}
```

Each procedure gets two overloads: one without `CallOptions` (preserving the current API) and one with.

## 6. Updated `rpcFetch` Signature

```typescript
async function rpcFetch(
  config: RpcClientConfig,
  method: "GET" | "POST",
  procedure: string,
  input?: unknown,
  callOptions?: CallOptions,    // ← new parameter
): Promise<unknown>
```

## 7. Merge Semantics

### 7.1 Headers

Per-call headers merge on top of client-level headers. Per-call values win on conflict:

```typescript
const customHeaders = typeof config.headers === "function"
  ? await config.headers()
  : config.headers;
const baseHeaders: Record<string, string> = {
  ...customHeaders,
  ...callOptions?.headers,   // ← per-call wins
};
```

### 7.2 Timeout

Per-call timeout **replaces** the client-level timeout (not added to it):

```typescript
const effectiveTimeout = callOptions?.timeout ?? config.timeout;
```

### 7.3 Signal

Per-call signal is **combined** with the client-level signal. If both are present, `AbortSignal.any()` merges them so that aborting either one cancels the request:

```typescript
const signals: AbortSignal[] = [];
if (config.signal) signals.push(config.signal);
if (callOptions?.signal) signals.push(callOptions.signal);
if (effectiveTimeout) {
  const tc = new AbortController();
  setTimeout(() => tc.abort(), effectiveTimeout);
  signals.push(tc.signal);
}
if (signals.length > 0) {
  init.signal = signals.length === 1 ? signals[0] : AbortSignal.any(signals);
}
```

## 8. Factory Implementation

The factory methods forward `callOptions` to `rpcFetch`:

```typescript
export function createRpcClient(config: RpcClientConfig): RpcClient {
  return {
    query(key: QueryKey, ...args: unknown[]): Promise<unknown> {
      const hasInput = args.length > 0 && !(args[0] && typeof args[0] === "object" && "signal" in args[0] && !("input" in args[0]));
      // ...
    },
  } as RpcClient;
}
```

However, detecting whether the second argument is `input` or `CallOptions` at runtime is fragile. A cleaner approach: the codegen already knows which procedures have void input. For void-input procedures, `args[0]` is always `CallOptions`. For non-void-input procedures, `args[0]` is input and `args[1]` is `CallOptions`:

```typescript
export function createRpcClient(config: RpcClientConfig): RpcClient {
  return {
    query(key: QueryKey, ...args: unknown[]): Promise<unknown> {
      return rpcFetch(config, "GET", key, args[0], args[1] as CallOptions | undefined);
    },
    mutate(key: MutationKey, ...args: unknown[]): Promise<unknown> {
      return rpcFetch(config, "POST", key, args[0], args[1] as CallOptions | undefined);
    },
  } as RpcClient;
}
```

For **void-input** procedures, the overloads are:
- `query(key: "time"): Promise<TimeResponse>` — no args
- `query(key: "time", options: CallOptions): Promise<TimeResponse>` — `args[0]` is options

This means void-input procedures pass `CallOptions` as `args[0]` (the `input` parameter of `rpcFetch`). `rpcFetch` must distinguish `CallOptions` from real input. Two solutions:

**Option A — Separate factory branches.** Generate two distinct proxy objects based on whether the manifest has mixed void/non-void procedures. This adds complexity.

**Option B — Shift void-input call options.** For void-input overloads, the codegen emits `rpcFetch(config, "GET", key, undefined, args[0])`. For non-void, it emits `rpcFetch(config, "GET", key, args[0], args[1])`. This requires splitting the factory body per-procedure-kind, which is already partially the case (queries vs mutations each have their own branch).

**Chosen: Option B.** The factory already has separate `query` and `mutate` branches. We split further: if _all_ procedures of a kind have the same void/non-void shape, emit one branch. If mixed, we use a lookup set. Since the manifest is known at codegen time, this is straightforward:

```typescript
// Generated when there are both void-input and non-void-input queries:
const VOID_QUERIES: Set<string> = new Set(["time", "version"]);

export function createRpcClient(config: RpcClientConfig): RpcClient {
  return {
    query(key: QueryKey, ...args: unknown[]): Promise<unknown> {
      if (VOID_QUERIES.has(key)) {
        return rpcFetch(config, "GET", key, undefined, args[0] as CallOptions | undefined);
      }
      return rpcFetch(config, "GET", key, args[0], args[1] as CallOptions | undefined);
    },
    mutate(key: MutationKey, ...args: unknown[]): Promise<unknown> {
      if (VOID_MUTATIONS.has(key)) {
        return rpcFetch(config, "POST", key, undefined, args[0] as CallOptions | undefined);
      }
      return rpcFetch(config, "POST", key, args[0], args[1] as CallOptions | undefined);
    },
  } as RpcClient;
}
```

When all queries are non-void (or all void), the set and `if` branch are omitted entirely.

## 9. Usage Examples

### 9.1 Per-Request Cancellation

```typescript
function SearchResults() {
  let controller = new AbortController();

  async function onInput(query: string) {
    controller.abort();                // cancel previous
    controller = new AbortController();
    const results = await rpc.query("search", query, {
      signal: controller.signal,
    });
    // ...
  }
}
```

### 9.2 Idempotency Key on Mutation

```typescript
await rpc.mutate("create_order", orderData, {
  headers: { "Idempotency-Key": crypto.randomUUID() },
});
```

### 9.3 Extended Timeout for Slow Report

```typescript
const report = await rpc.query("generate_report", params, {
  timeout: 30_000,
});
```

### 9.4 Options on Void-Input Call

```typescript
const controller = new AbortController();
const time = await rpc.query("time", { signal: controller.signal });
```

## 10. Codegen Changes (`client.rs`)

| Area                            | Change                                                                                                |
|---------------------------------|-------------------------------------------------------------------------------------------------------|
| New constant                    | `CALL_OPTIONS_INTERFACE` — the `CallOptions` interface literal                                        |
| `generate_client_file()`        | Emit `CallOptions` interface after `RpcClientConfig`                                                  |
| `FETCH_HELPER`                  | Add `callOptions?: CallOptions` parameter; merge headers, resolve effective timeout, combine signals  |
| `generate_query_overloads()`    | Emit two overloads per procedure: without and with `CallOptions`                                      |
| `generate_mutation_overloads()` | Same — two overloads per procedure                                                                    |
| `generate_client_factory()`     | Emit `VOID_QUERIES` / `VOID_MUTATIONS` sets when mixed void/non-void exists; route `args` accordingly |

## 11. Test Plan

### Unit tests (codegen)

| Test                                   | Description                                                                 |
|----------------------------------------|-----------------------------------------------------------------------------|
| `contains_call_options_interface`      | Output includes `CallOptions` interface with `signal`, `headers`, `timeout` |
| `query_overload_with_options`          | Non-void query has overload with trailing `CallOptions`                     |
| `query_void_overload_with_options`     | Void-input query has overload with `CallOptions` as only argument           |
| `mutation_overload_with_options`       | Non-void mutation has overload with trailing `CallOptions`                  |
| `mutation_void_overload_with_options`  | Void-input mutation has overload with `CallOptions`                         |
| `fetch_helper_accepts_call_options`    | `rpcFetch` signature includes `callOptions?: CallOptions`                   |
| `fetch_helper_merges_call_headers`     | Body references `callOptions?.headers`                                      |
| `fetch_helper_uses_call_timeout`       | Body references `callOptions?.timeout`                                      |
| `fetch_helper_uses_call_signal`        | Body references `callOptions?.signal`                                       |
| `void_query_set_generated`             | When mixed void/non-void queries exist, `VOID_QUERIES` set is emitted       |
| `void_query_set_omitted_when_all_same` | When all queries are non-void, no set is emitted                            |
| `snapshot_client_with_call_options`    | Insta snapshot of full client with mixed void/non-void procedures           |

### Snapshot tests

Update existing client snapshots (`snapshot_client_with_methods`, `snapshot_full_client`, etc.) to reflect new overloads and `CallOptions` interface.

## 12. Backward Compatibility

- **No breaking changes.** All new overloads add an optional trailing parameter. Existing call sites that omit `CallOptions` continue to match the original overload.
- `CallOptions` is a new exported type — no conflict with existing types.
- `rpcFetch` gains an optional parameter — existing internal calls are unaffected.

## 13. Future Extensions

These are out of scope but the design accommodates them:

- **Per-call retry override** — add `retry?: RetryPolicy` to `CallOptions`. Omitted now to keep the interface focused.
- **Per-call serialize/deserialize** — unlikely to be needed per-call but the pattern extends naturally.
- **Request deduplication** — the next roadmap item. `CallOptions.signal` integration means dedup can respect per-call cancellation.
