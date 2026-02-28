# Client Interactive Demos — Design

## Goal

Add interactive demos to 7 client documentation pages (all except Config).
Each demo lets users click a button and see the feature in action — live API calls, logs, error states.

## Demos

### 1. Headers — auth token

- **Lambda:** existing `secret.rs` (Bearer token check)
- **UI:** two buttons — "Without token" (→ 401 error) and "With token" (→ secret data). Show request headers and response.

### 2. Timeout & Abort

- **Lambda:** existing `timeout_demo.rs` (server timeout 3s, accepts `sleep_ms`)
- **UI:** numeric input "Server delay (ms)" (default 1000). "Fetch with 2s client timeout" button — delay > 2000 → AbortError. "Abort manually" button with AbortController. Event log with timestamps.

### 3. Retry — attempt log

- **Lambda:** NEW `retry_demo.rs` — accepts `fail_count: u32`, returns 500 for first N calls (AtomicU32 counter), then 200. Returns `{ attempt: u32, total_calls: u32, message: String }`.
- **UI:** select "Fail first N requests" (1-3). "Fetch with retry" button. Log: "Attempt 1 → 500 (retrying...) → Attempt 2 → 200 OK". Uses `onError`/`onResponse` hooks to build log.

### 4. Deduplication — N calls → 1 fetch

- **Lambda:** NEW `dedup_demo.rs` — sleep 500ms, returns `{ request_count: u64, timestamp: String }` using AtomicU64 counter.
- **UI:** "Fire 5 identical queries" button → shows "5 calls, 1 HTTP request" (counter +1). "Fire 5 with dedupe: false" → counter +5. Display server request_count before/after.

### 5. Hooks — lifecycle log

- **Lambda:** reuses `retry_demo.rs` (fail_count=1)
- **UI:** "Fetch (will fail once)" button. Real-time lifecycle log: `onRequest {procedure, attempt: 0}` → `onError {status: 500, willRetry: true}` → `onRequest {attempt: 1}` → `onResponse {status: 200, duration}`. Each step timestamped and color-coded.

### 6. Serialization — JSON vs lossless-json

- **Lambda:** reuses existing `bigint_demo.rs`
- **UI:** two clients (default JSON.parse vs lossless-json). Table comparing values, `typeof`, precision loss. Same pattern as bigint page but framed as "serialization config" demo.

### 7. Custom Fetch — SSR cookies vs client

- **Lambda:** NEW `cookie_demo.rs` — reads a specific cookie from Headers, returns OK if present, 401 if not.
- **UI:** page has `+page.server.ts` that calls via `event.fetch` (forwards cookies → OK) and client-side call (no cookies → 401). Shows both results side by side. "Set cookie" button to demonstrate client fetch after cookie is set.

## New Rust Lambdas (3)

1. `retry_demo.rs` — configurable failure count with atomic counter
2. `dedup_demo.rs` — slow endpoint with request counter
3. `cookie_demo.rs` — cookie-based auth check

## Patterns

- Follow existing interactive demo pattern (type-mappings, bigint pages)
- `$state()` reactivity, `rpc.query()` calls, result tables, collapsible Rust source
- Code snippets pre-highlighted via `+page.server.ts`
- Work in new git branch, no push
