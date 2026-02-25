# RFC-4: Extended `RpcClientConfig`

- **Status:** Implemented
- **Topic:** Lifecycle hooks, retry, timeout, serialization, and abort support for the generated RPC client
- **Date:** February 2026

## 1. Summary

Extend the generated `RpcClientConfig` interface with runtime options that cover real-world production needs: lifecycle hooks (`onRequest`, `onResponse`, `onError`), automatic retry with configurable policy, per-request timeout, custom serialization/deserialization, and `AbortSignal` integration. All new fields are optional — the existing zero-config usage remains unchanged.

## 2. Motivation

The current `RpcClientConfig` supports three fields: `baseUrl`, an optional `fetch` override, and optional `headers`. This is enough for a demo but falls short in production:

| Need                                  | Current workaround                   | Problem                                                         |
|---------------------------------------|--------------------------------------|-----------------------------------------------------------------|
| Auth token injection                  | `headers` callback                   | Works, but no access to request context (procedure name, input) |
| Request/response logging              | Wrap `fetch`                         | Boilerplate, loses type context                                 |
| Retry on transient errors             | Manual try/catch loops in app code   | Duplicated across every call site                               |
| Timeout enforcement                   | `AbortController` at every call site | Verbose, easy to forget                                         |
| Date/BigInt round-tripping            | Not possible                         | `JSON.parse` loses type fidelity                                |
| Cancellation (route changes, unmount) | Not possible                         | No `signal` pass-through                                        |

Every modern RPC/HTTP client (tRPC, ky, ofetch, wretch) offers these primitives. Without them, users either wrap `createRpcClient` in bespoke middleware or abandon the generated client entirely.

## 3. Design Principles

1. **Additive only** — every new field is optional with a sensible default. Existing code compiles and works without changes.
2. **Composable** — hooks receive context objects, not raw `Request`/`Response`, so they compose cleanly (e.g. logging + auth + metrics can stack).
3. **No external dependencies** — the generated client remains dependency-free; all features use platform APIs (`AbortController`, `setTimeout`, `fetch`).
4. **Codegen-friendly** — the `client.rs` codegen emits these types as string literals; no runtime library is required.

## 4. Extended `RpcClientConfig`

```typescript
export interface RpcClientConfig {
  /** Base URL for all RPC calls (e.g. "/api" or "https://example.com/api"). */
  baseUrl: string;

  /** Custom fetch implementation (useful for SSR, testing, or polyfills). */
  fetch?: typeof globalThis.fetch;

  /** Static headers or an async factory (e.g. for auth tokens). */
  headers?:
    | Record<string, string>
    | (() => Record<string, string> | Promise<Record<string, string>>);

  // --- New fields below ---

  /** Called before every request. Can modify headers or log. */
  onRequest?: (ctx: RequestContext) => void | Promise<void>;

  /** Called after a successful response. Useful for logging or metrics. */
  onResponse?: (ctx: ResponseContext) => void | Promise<void>;

  /** Called when a request fails (network error or non-2xx). */
  onError?: (ctx: ErrorContext) => void | Promise<void>;

  /** Automatic retry policy for failed requests. */
  retry?: RetryPolicy;

  /** Request timeout in milliseconds. Default: none (no timeout). */
  timeout?: number;

  /** Custom serializer replacing `JSON.stringify` for request bodies. */
  serialize?: (input: unknown) => string;

  /** Custom deserializer replacing `res.json()` for response bodies. */
  deserialize?: (text: string) => unknown;

  /** AbortSignal for cancelling all requests made by this client. */
  signal?: AbortSignal;
}
```

## 5. Supporting Types

### 5.1 `RequestContext`

```typescript
export interface RequestContext {
  /** Procedure name (e.g. "hello", "echo"). */
  procedure: string;
  /** HTTP method ("GET" for queries, "POST" for mutations). */
  method: "GET" | "POST";
  /** Full request URL including query params. */
  url: string;
  /** Mutable headers — hooks can add/override entries. */
  headers: Record<string, string>;
  /** The serialized input (if any). */
  input?: unknown;
}
```

### 5.2 `ResponseContext`

```typescript
export interface ResponseContext {
  /** Procedure name. */
  procedure: string;
  /** HTTP method. */
  method: "GET" | "POST";
  /** Full request URL. */
  url: string;
  /** The raw `Response` object. */
  response: Response;
  /** Parsed response data (after deserialization). */
  data: unknown;
  /** Round-trip duration in milliseconds. */
  duration: number;
}
```

### 5.3 `ErrorContext`

```typescript
export interface ErrorContext {
  /** Procedure name. */
  procedure: string;
  /** HTTP method. */
  method: "GET" | "POST";
  /** Full request URL. */
  url: string;
  /** The error (RpcError for HTTP failures, TypeError for network errors). */
  error: RpcError | Error;
  /** Current attempt number (1-based). Includes initial request. */
  attempt: number;
  /** Whether the request will be retried. */
  willRetry: boolean;
}
```

### 5.4 `RetryPolicy`

```typescript
export interface RetryPolicy {
  /** Maximum number of retry attempts (excluding the initial request). Default: 0. */
  attempts: number;
  /** Delay in ms between retries, or a function of attempt number for backoff. */
  delay: number | ((attempt: number) => number);
  /**
   * HTTP status codes that trigger a retry.
   * Default: [408, 429, 500, 502, 503, 504].
   */
  retryOn?: number[];
}
```

## 6. Behavior Specification

### 6.1 Hook Execution Order

```
onRequest  →  fetch  →  onResponse (success)
                    ↘  onError (failure) → retry? → onRequest → ...
```

1. **`onRequest`** fires before `fetch`. The `headers` object on `RequestContext` is mutable — any entries added here are included in the request.
2. On success (2xx), **`onResponse`** fires with parsed data and timing.
3. On failure, **`onError`** fires. If retry policy applies, the cycle repeats from `onRequest`.
4. If all retries are exhausted, the error is thrown to the caller.

### 6.2 Timeout

When `timeout` is set:

```typescript
const controller = new AbortController();
const timeoutId = setTimeout(() => controller.abort(), config.timeout);
// ... merge controller.signal with config.signal if both present
// ... pass merged signal to fetch
clearTimeout(timeoutId);
```

A timeout produces an `RpcError` with `status: 0` and message `"Request timeout after {timeout}ms"`.

### 6.3 Retry

Retry fires when:
1. The response status is in `retryOn` (default: `[408, 429, 500, 502, 503, 504]`)
2. A network error occurs (e.g. `TypeError: Failed to fetch`)
3. `attempt <= policy.attempts` (i.e., `attempts: 3` means up to 3 retries after the initial request, 4 total)

Delay is either a fixed number or a function:

```typescript
// Fixed: 1000ms between each retry
retry: { attempts: 3, delay: 1000 }

// Exponential backoff: 1s, 2s, 4s
retry: { attempts: 3, delay: (n) => 1000 * 2 ** (n - 1) }

// Exponential backoff with jitter
retry: { attempts: 3, delay: (n) => 1000 * 2 ** (n - 1) + Math.random() * 200 }
```

### 6.4 Serialization

When `serialize` is provided, it replaces `JSON.stringify` for request bodies:

```typescript
// Before (current)
init.body = JSON.stringify(input);

// After
init.body = config.serialize ? config.serialize(input) : JSON.stringify(input);
```

When `deserialize` is provided, it replaces `res.json()`:

```typescript
// Before (current)
const json = await res.json();

// After
const text = await res.text();
const json = config.deserialize ? config.deserialize(text) : JSON.parse(text);
```

This enables round-tripping types that `JSON` loses (e.g. `Date`, `BigInt`, `Map`) via libraries like `superjson` or `devalue`.

### 6.5 AbortSignal

`config.signal` is forwarded to `fetch` as `init.signal`. When both `signal` and `timeout` are set, signals are merged using `AbortSignal.any()`:

```typescript
const signals: AbortSignal[] = [];
if (config.signal) signals.push(config.signal);
if (config.timeout) {
  const tc = new AbortController();
  setTimeout(() => tc.abort(), config.timeout);
  signals.push(tc.signal);
}
if (signals.length > 0) {
  init.signal = signals.length === 1 ? signals[0] : AbortSignal.any(signals);
}
```

## 7. Usage Examples

### 7.1 Auth Token Injection

```typescript
const rpc = createRpcClient({
  baseUrl: "/api",
  onRequest: (ctx) => {
    const token = getAuthToken();
    if (token) {
      ctx.headers["Authorization"] = `Bearer ${token}`;
    }
  },
});
```

### 7.2 Request Logging

```typescript
const rpc = createRpcClient({
  baseUrl: "/api",
  onRequest: (ctx) => {
    console.log(`→ ${ctx.method} ${ctx.procedure}`, ctx.input);
  },
  onResponse: (ctx) => {
    console.log(`← ${ctx.procedure} ${ctx.duration}ms`, ctx.data);
  },
  onError: (ctx) => {
    console.error(`✕ ${ctx.procedure} attempt ${ctx.attempt}`, ctx.error);
  },
});
```

### 7.3 Retry with Exponential Backoff

```typescript
const rpc = createRpcClient({
  baseUrl: "/api",
  retry: {
    attempts: 3,
    delay: (n) => 1000 * 2 ** (n - 1),
  },
});
```

### 7.4 Timeout + Cancellation

```typescript
// Global 10s timeout
const rpc = createRpcClient({
  baseUrl: "/api",
  timeout: 10_000,
});

// Per-page cancellation (SvelteKit)
const controller = new AbortController();
const rpc = createRpcClient({
  baseUrl: "/api",
  signal: controller.signal,
});
onDestroy(() => controller.abort());
```

### 7.5 Custom Serialization with superjson

```typescript
import superjson from "superjson";

const rpc = createRpcClient({
  baseUrl: "/api",
  serialize: (input) => superjson.stringify(input),
  deserialize: (text) => superjson.parse(text),
});
```

### 7.6 Composing Multiple Concerns

```typescript
const rpc = createRpcClient({
  baseUrl: "/api",
  headers: { "X-App-Version": "1.2.0" },
  timeout: 5000,
  retry: { attempts: 2, delay: 500 },
  onRequest: (ctx) => {
    ctx.headers["Authorization"] = `Bearer ${getToken()}`;
    performance.mark(`rpc-${ctx.procedure}-start`);
  },
  onResponse: (ctx) => {
    performance.mark(`rpc-${ctx.procedure}-end`);
    performance.measure(ctx.procedure,
      `rpc-${ctx.procedure}-start`,
      `rpc-${ctx.procedure}-end`,
    );
  },
  onError: (ctx) => {
    if (!ctx.willRetry) reportToSentry(ctx.error);
  },
});
```

## 8. Codegen Changes (`client.rs`)

### 8.1 New Type Literals

Add string constants for the new interfaces (`RequestContext`, `ResponseContext`, `ErrorContext`, `RetryPolicy`) and update `CONFIG_INTERFACE` to include the new fields.

### 8.2 Updated `FETCH_HELPER`

The `rpcFetch` function gains the following:

```
rpcFetch(config, method, procedure, input?)
  │
  ├─ Build URL + headers
  ├─ Serialize input (config.serialize ?? JSON.stringify)
  ├─ Build RequestContext
  ├─ Call onRequest(ctx) — ctx.headers may be mutated
  ├─ Build AbortSignal (merge timeout + config.signal)
  ├─ Start timer
  ├─ Call fetch
  │   ├─ Success (2xx):
  │   │   ├─ Deserialize (config.deserialize ?? res.json)
  │   │   ├─ Build ResponseContext (with duration)
  │   │   ├─ Call onResponse(ctx)
  │   │   └─ Return data
  │   └─ Failure:
  │       ├─ Build ErrorContext (with attempt, willRetry)
  │       ├─ Call onError(ctx)
  │       ├─ If retryable && attempts left:
  │       │   ├─ Wait delay
  │       │   └─ Loop back to "Build RequestContext"
  │       └─ Throw RpcError
  └─ On AbortError / timeout:
      ├─ Build ErrorContext
      ├─ Call onError(ctx)
      └─ Throw RpcError(0, "Request timeout..." | "Request aborted")
```

### 8.3 No Changes to Overloads

The `RpcClient` interface (query/mutate overloads) and `createRpcClient` factory signature remain identical. All new behavior is internal to `rpcFetch`.

## 9. Files Modified

| File                                   | Action                                                            |
|----------------------------------------|-------------------------------------------------------------------|
| `crates/metaxy-cli/src/codegen/client.rs` | Update `CONFIG_INTERFACE`, `FETCH_HELPER`; add new type constants |
| `crates/metaxy-cli/tests/client.rs`       | Add tests for new types in generated output                       |
| `demo/src/lib/rpc-client.ts`           | Regenerated (auto)                                                |

## 10. Test Plan

### Unit tests (codegen)

| Test                                         | Description                                                                                                               |
|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| `generated_client_contains_request_context`  | Output includes `RequestContext` interface                                                                                |
| `generated_client_contains_response_context` | Output includes `ResponseContext` interface                                                                               |
| `generated_client_contains_error_context`    | Output includes `ErrorContext` interface                                                                                  |
| `generated_client_contains_retry_policy`     | Output includes `RetryPolicy` interface                                                                                   |
| `config_interface_has_new_fields`            | `RpcClientConfig` includes `onRequest`, `onResponse`, `onError`, `retry`, `timeout`, `serialize`, `deserialize`, `signal` |
| `fetch_helper_calls_on_request`              | `rpcFetch` body references `config.onRequest`                                                                             |
| `fetch_helper_calls_on_response`             | `rpcFetch` body references `config.onResponse`                                                                            |
| `fetch_helper_calls_on_error`                | `rpcFetch` body references `config.onError`                                                                               |
| `fetch_helper_handles_timeout`               | `rpcFetch` body references `AbortController` + `setTimeout`                                                               |
| `fetch_helper_handles_retry`                 | `rpcFetch` body contains retry loop logic                                                                                 |
| `fetch_helper_uses_custom_serialize`         | `rpcFetch` body references `config.serialize`                                                                             |
| `fetch_helper_uses_custom_deserialize`       | `rpcFetch` body references `config.deserialize`                                                                           |

### E2E tests (demo)

| Test                                        | Description                                            |
|---------------------------------------------|--------------------------------------------------------|
| `e2e_timeout_aborts_request`                | Client with `timeout: 1` fails with timeout error      |
| `e2e_retry_recovers_from_transient_failure` | Client retries and succeeds on flaky endpoint          |
| `e2e_on_request_injects_header`             | `onRequest` adds auth header, secret endpoint succeeds |
| `e2e_on_error_fires_on_failure`             | `onError` callback receives error context              |
| `e2e_signal_aborts_request`                 | Pre-aborted signal throws immediately                  |

## 11. Backward Compatibility

- **No breaking changes.** All new fields are optional.
- Existing `createRpcClient({ baseUrl: "/api" })` continues to work identically.
- The generated `rpcFetch` function short-circuits new features when their config fields are `undefined`, producing the same behavior as the current implementation.

## 12. Future Extensions

These are explicitly out of scope but the design accommodates them:

- **Per-request config overrides** — `rpc.query("hello", input, { timeout: 5000 })` via an optional third argument on query/mutate. Requires overload signature changes.
- **Middleware / interceptor chain** — A formal `use()` plugin API for stacking multiple concerns. The current hooks are a simpler starting point.
- **Batching** — Combine multiple calls into a single HTTP request. Orthogonal to this RFC.
- **Streaming / SSE** — Server-sent events for subscriptions. Requires new procedure kind (`#[rpc_subscription]`).

## 13. Alternatives Considered

### Wrapper / middleware pattern

Instead of extending `RpcClientConfig`, provide a `wrapClient(client, middleware)` helper. Rejected because:
- It loses type safety on the inner client.
- Users must learn a new abstraction.
- The generated code can't optimize (e.g. timeout signal merging).

### Separate `createAdvancedRpcClient`

Keep the simple client as-is and add a second factory. Rejected because:
- Two factories with overlapping APIs causes confusion.
- The "simple" client would never gain features users need.
- All new fields are optional, so the simple case stays simple.

### External dependency (ky, ofetch)

Use a battle-tested HTTP library internally. Rejected because:
- Adds a runtime dependency to generated code — breaks "zero-dependency" guarantee.
- Bundle size impact for features users may not need.
- The generated client must work in any JS runtime (Node, Deno, Bun, Cloudflare Workers) without compatibility shims.
