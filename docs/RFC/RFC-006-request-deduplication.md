# RFC-006: Request Deduplication

- **Status:** Implemented
- **Topic:** Automatic deduplication of identical in-flight queries
- **Date:** February 2026

## 1. Summary

When multiple callers issue the same query with the same input concurrently, only one HTTP request is made. All callers receive the same promise. This eliminates redundant network requests that commonly occur when several UI components mount simultaneously and request the same data.

## 2. Motivation

In component-based UIs, it is common for multiple components to independently fetch the same data:

```typescript
// NavBar.svelte
const user = await rpc.query("current_user");

// Sidebar.svelte
const user = await rpc.query("current_user");

// ProfileCard.svelte
const user = await rpc.query("current_user");
```

Without deduplication, this fires three identical HTTP requests. With dedup, a single request is made and all three callers share the result.

| Scenario                                          | Without dedup             | With dedup                   |
|---------------------------------------------------|---------------------------|------------------------------|
| 5 components request the same user on mount       | 5 identical HTTP requests | 1 request, 5 shared promises |
| Dashboard with 3 widgets showing the same metrics | 3 requests                | 1 request                    |
| Parent and child both fetch config                | 2 requests                | 1 request                    |

Every major data-fetching library supports this: SWR, TanStack Query, tRPC, Apollo Client, and URQL all dedup identical in-flight requests. The roadmap lists this as the next Phase 2 deliverable.

## 3. Design Principles

1. **Queries only** — mutations are never deduplicated. Mutations have side effects and must execute every time.
2. **Automatic by default** — dedup is enabled with no configuration required. Callers don't need to know about it.
3. **Opt-out at any level** — disable globally via `RpcClientConfig.dedupe` or per-call via `CallOptions.dedupe`.
4. **First caller drives the request** — the first caller's `CallOptions` (headers, timeout) are used for the actual HTTP request. Subsequent callers piggyback on the in-flight promise.
5. **Per-caller signal respect** — each caller's `AbortSignal` is honored individually without cancelling the shared request for other callers.
6. **Zero new dependencies** — uses a plain `Map` and platform APIs only.

## 4. Dedup Key

The dedup key is derived from the procedure name and serialized input:

```typescript
function dedupKey(procedure: string, input: unknown, config: RpcClientConfig): string {
  const serialized = input === undefined
    ? ""
    : config.serialize
      ? config.serialize(input)
      : JSON.stringify(input);
  return `${procedure}:${serialized}`;
}
```

This means two calls are considered identical when they target the same procedure with the same serialized input. Per-call headers, timeout, and signal are **not** part of the key — these are execution details, not identity.

## 5. Configuration

### 5.1 Client-Level

A new optional `dedupe` field on `RpcClientConfig`:

```typescript
export interface RpcClientConfig {
  // ... existing fields ...

  /**
   * Enable request deduplication for queries.
   * When true, identical in-flight queries share a single HTTP request.
   * @default true
   */
  dedupe?: boolean;
}
```

Default is `true`. Set to `false` to disable dedup globally.

### 5.2 Per-Call

A new optional `dedupe` field on `CallOptions`:

```typescript
export interface CallOptions {
  headers?: Record<string, string>;
  timeout?: number;
  signal?: AbortSignal;

  /**
   * Override the client-level deduplication setting for this call.
   * Set to false to force a fresh request even if an identical query is in-flight.
   */
  dedupe?: boolean;
}
```

Per-call `dedupe` overrides the client-level setting. This allows forcing a fresh request for a specific call:

```typescript
// Force a fresh request, bypassing any in-flight dedup
const freshUser = await rpc.query("current_user", { dedupe: false });
```

### 5.3 Resolution

```typescript
const shouldDedupe = callOptions?.dedupe ?? config.dedupe ?? true;
```

## 6. In-Flight Map

The client factory maintains a `Map` of in-flight query promises, scoped to the client instance:

```typescript
export function createRpcClient(config: RpcClientConfig): RpcClient {
  const inflight = new Map<string, Promise<unknown>>();

  return {
    query(key: QueryKey, ...args: unknown[]): Promise<unknown> {
      // ... dedup logic here ...
    },
    mutate(key: MutationKey, ...args: unknown[]): Promise<unknown> {
      // ... no dedup, direct rpcFetch ...
    },
  } as RpcClient;
}
```

Each client instance has its own map. Creating a new client starts with an empty map. This is intentional — different clients may have different base URLs, headers, or auth contexts.

## 7. Query Method Implementation

The `query()` method is the only place where dedup logic lives:

```typescript
query(key: QueryKey, ...args: unknown[]): Promise<unknown> {
  // 1. Resolve input and callOptions (existing void/non-void branching)
  const [input, callOptions] = VOID_QUERIES.has(key)
    ? [undefined, args[0] as CallOptions | undefined]
    : [args[0], args[1] as CallOptions | undefined];

  // 2. Check if dedup is enabled
  const shouldDedupe = callOptions?.dedupe ?? config.dedupe ?? true;

  if (shouldDedupe) {
    const key_ = dedupKey(key, input, config);

    // 3. Return in-flight promise if one exists
    const existing = inflight.get(key_);
    if (existing) {
      return wrapWithSignal(existing, callOptions?.signal);
    }

    // 4. Create new request, store in map, clean up on settlement
    const promise = rpcFetch(config, "GET", key, input, callOptions)
      .finally(() => inflight.delete(key_));
    inflight.set(key_, promise);

    return wrapWithSignal(promise, callOptions?.signal);
  }

  // 5. Dedup disabled — direct call
  return rpcFetch(config, "GET", key, input, callOptions);
}
```

The `mutate()` method remains unchanged — it always calls `rpcFetch` directly.

## 8. Signal Handling

When multiple callers share a promise, each may have their own `AbortSignal`. The shared HTTP request should not be cancelled when a single caller aborts — only that caller's promise should reject.

### 8.1 `wrapWithSignal` Helper

```typescript
function wrapWithSignal<T>(promise: Promise<T>, signal?: AbortSignal): Promise<T> {
  if (!signal) return promise;
  if (signal.aborted) return Promise.reject(signal.reason);

  return new Promise<T>((resolve, reject) => {
    const onAbort = () => reject(signal.reason);
    signal.addEventListener("abort", onAbort, { once: true });
    promise.then(
      (value) => { signal.removeEventListener("abort", onAbort); resolve(value); },
      (error) => { signal.removeEventListener("abort", onAbort); reject(error); },
    );
  });
}
```

### 8.2 Behavior Matrix

| First caller signal | Second caller signal | HTTP request | First caller result | Second caller result |
|---------------------|----------------------|--------------|---------------------|----------------------|
| none                | none                 | completes    | resolved            | resolved             |
| aborts              | none                 | aborts (*)   | rejected            | rejected             |
| none                | aborts               | completes    | resolved            | rejected (abort)     |
| aborts              | aborts               | aborts (*)   | rejected            | rejected             |

(*) The first caller's signal is passed to `rpcFetch`, so aborting it cancels the actual HTTP request. This is the expected "first caller drives the request" behavior. If this is undesirable for a use case, the caller should omit their signal and manage cancellation separately.

### 8.3 Trade-Off: First-Caller Signal Cancels Shared Request

When the first caller aborts, the shared HTTP request is cancelled because their signal was forwarded to `rpcFetch`. This means piggybacking callers also receive a rejection. This is a deliberate trade-off:

- **Pro:** Simple implementation, no signal aggregation complexity.
- **Pro:** Matches the mental model — the "real" request belongs to whoever initiated it.
- **Con:** A component unmounting could cancel a shared request.

**Mitigation:** Callers that may unmount early and share data with long-lived components should omit per-call signals and rely on the client-level signal (or no signal) instead. Alternatively, set `dedupe: false` on the short-lived call.

## 9. Usage Examples

### 9.1 Automatic Dedup (Default)

```typescript
// Three components mount simultaneously — one HTTP request
const [a, b, c] = await Promise.all([
  rpc.query("current_user"),
  rpc.query("current_user"),
  rpc.query("current_user"),
]);
// a, b, c all receive the same result
```

### 9.2 Different Input = Different Requests

```typescript
// Two different users — two HTTP requests (different keys)
const [alice, bob] = await Promise.all([
  rpc.query("user", { id: 1 }),
  rpc.query("user", { id: 2 }),
]);
```

### 9.3 Opt-Out Per Call

```typescript
// Force a fresh request, even if an identical query is in-flight
const fresh = await rpc.query("current_user", { dedupe: false });
```

### 9.4 Opt-Out Globally

```typescript
const rpc = createRpcClient({
  baseUrl: "/api/rpc",
  dedupe: false, // disable for all queries
});
```

### 9.5 Dedup with Per-Caller Cancellation

```typescript
// Component A: long-lived, no signal
const userA = rpc.query("current_user");

// Component B: short-lived, has signal
const controller = new AbortController();
const userB = rpc.query("current_user", { signal: controller.signal });

// Component B unmounts → only userB rejects, the shared request continues
controller.abort();

await userA; // ✓ resolves normally
await userB; // ✗ rejects with AbortError
```

### 9.6 Sequential Calls Are Not Deduplicated

```typescript
// First call completes and is removed from the in-flight map
const a = await rpc.query("current_user");

// Second call is a new request (no in-flight promise exists)
const b = await rpc.query("current_user");
```

Dedup only applies to concurrent in-flight requests. Once a promise settles, it is removed from the map. This is not a cache.

## 10. Codegen Changes (`client.rs`)

| Area                        | Change                                                                                       |
|-----------------------------|----------------------------------------------------------------------------------------------|
| `RpcClientConfig`           | Add optional `dedupe?: boolean` field                                                        |
| `CallOptions`               | Add optional `dedupe?: boolean` field                                                        |
| New constant                | `DEDUP_KEY_FN` — the `dedupKey` helper function                                              |
| New constant                | `WRAP_WITH_SIGNAL_FN` — the `wrapWithSignal` helper function                                 |
| `generate_client_factory()` | Add `inflight` map; wrap `query()` body with dedup logic                                     |
| `generate_client_factory()` | Extract input/callOptions resolution into local variables for both dedup and non-dedup paths |
| `mutate()` body             | No changes — mutations are never deduplicated                                                |

### 10.1 Conditional Emission

The dedup helpers (`dedupKey`, `wrapWithSignal`, `inflight` map) are **always emitted** when the manifest contains at least one query. If there are no queries, the dedup code is omitted entirely (no dead code).

## 11. Test Plan

### Unit tests (codegen)

| Test                                 | Description                                                    |
|--------------------------------------|----------------------------------------------------------------|
| `contains_dedupe_config_field`       | `RpcClientConfig` includes `dedupe?: boolean`                  |
| `contains_call_options_dedupe_field` | `CallOptions` includes `dedupe?: boolean`                      |
| `contains_dedup_key_function`        | Output includes `dedupKey` helper function                     |
| `contains_wrap_with_signal_function` | Output includes `wrapWithSignal` helper function               |
| `factory_contains_inflight_map`      | `createRpcClient` body declares `inflight` Map                 |
| `query_body_contains_dedup_logic`    | `query()` method checks `shouldDedupe` and uses `inflight` map |
| `mutate_body_has_no_dedup`           | `mutate()` method does not reference `inflight` or dedup       |
| `dedup_key_uses_config_serialize`    | `dedupKey` body references `config.serialize`                  |
| `wrap_with_signal_handles_aborted`   | `wrapWithSignal` checks `signal.aborted` early                 |
| `dedup_omitted_when_no_queries`      | When manifest has only mutations, dedup code is not emitted    |
| `snapshot_client_with_dedup`         | Insta snapshot of full client with dedup enabled               |

### Snapshot tests

Update existing client snapshots to reflect:
- New `dedupe` field in `RpcClientConfig` and `CallOptions`
- `dedupKey` and `wrapWithSignal` helpers in output
- `inflight` map in factory body
- Dedup logic in `query()` method

## 12. Backward Compatibility

- **No breaking changes.** Dedup is enabled by default but is purely an optimization — the observable behavior (resolved values) is identical.
- New `dedupe` fields on `RpcClientConfig` and `CallOptions` are optional. Existing code that omits them gets dedup automatically.
- The `inflight` map is internal to the factory closure — no new exports or public API surface beyond the two config fields.

## 13. Interactions with Existing Features

### 13.1 Per-Call Options (RFC-005)

- `CallOptions.headers` and `CallOptions.timeout` on the **first** caller are used for the shared request. Subsequent callers' headers/timeout are ignored (they piggyback on the in-flight promise).
- `CallOptions.signal` on subsequent callers is respected via `wrapWithSignal` — each caller can independently cancel their participation.
- `CallOptions.dedupe` provides per-call opt-out.

### 13.2 Retry Policy (RFC-004)

Retry logic lives inside `rpcFetch` and is unaffected. If the first attempt fails and retries are configured, all deduplicated callers wait for the full retry sequence. The promise in the `inflight` map represents the final settled result (after all retries).

### 13.3 Lifecycle Hooks (RFC-004)

`onRequest`, `onResponse`, and `onError` hooks fire once per actual HTTP request, not once per deduplicated caller. This is correct — hooks observe real network activity.

### 13.4 Custom Serializer

If `config.serialize` is provided, it is used for both the fetch body and the dedup key. This ensures that the dedup key matches exactly what would be sent over the wire.

## 14. Future Extensions

These are out of scope but the design accommodates them:

- **TTL-based caching** — extend the `inflight` map to keep settled promises for a configurable duration. The dedup key infrastructure is directly reusable.
- **Shared signal aggregation** — track all callers' signals and only abort the shared request when all have aborted. Adds complexity with marginal benefit.
- **Dedup for mutations with `idempotent` flag** — if macro-level metadata (Phase 4) marks a mutation as idempotent, dedup could optionally apply. The `inflight` map pattern extends naturally.
