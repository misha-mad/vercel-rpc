# Client Interactive Demos Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Add interactive demos to 7 client documentation pages (Headers, Timeout & Abort, Retry, Deduplication, Hooks, Serialization, Custom Fetch).

**Architecture:** Each demo adds a "Try it" section to the existing doc page. New Rust lambdas where needed (3 total), then UI updates to each `+page.svelte` and `+page.server.ts`. After creating lambdas, run `metaxy generate` from `demo/` to regenerate types/client.

**Tech Stack:** Rust (Vercel lambdas), SvelteKit (Svelte 5 runes), TypeScript, metaxy-cli codegen.

---

## Prerequisites

Before starting, create a new branch:
```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
git checkout -b feat/client-interactive-demos
```

## Task 1: Create `retry_demo.rs` lambda

**Files:**
- Create: `demo/api/retry_demo.rs`

**Step 1: Write the lambda**

```rust
use metaxy::rpc_query;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

#[derive(Deserialize, Serialize)]
pub struct RetryDemoInput {
    /// How many initial calls should return 500
    pub fail_count: u32,
    /// Pass true to reset the counter before this call
    pub reset: bool,
}

#[derive(Serialize)]
pub struct RetryDemoResponse {
    pub call_number: u32,
    pub total_calls: u32,
    pub message: String,
}

/// Returns 500 for the first `fail_count` calls, then 200.
/// Use `reset: true` to restart the counter.
#[rpc_query]
async fn retry_demo(input: RetryDemoInput) -> Result<RetryDemoResponse, String> {
    if input.reset {
        CALL_COUNT.store(0, Ordering::Relaxed);
    }
    let call_number = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    if call_number <= input.fail_count {
        return Err(format!(
            "Simulated failure (call {} of {} requested failures)",
            call_number, input.fail_count
        ));
    }

    Ok(RetryDemoResponse {
        call_number,
        total_calls: CALL_COUNT.load(Ordering::Relaxed),
        message: format!("Success on call {}", call_number),
    })
}
```

**Step 2: Commit**

```bash
git add demo/api/retry_demo.rs
git commit -m "feat(metaxy): add retry_demo lambda for retry/hooks demos"
```

---

## Task 2: Create `dedup_demo.rs` lambda

**Files:**
- Create: `demo/api/dedup_demo.rs`

**Step 1: Write the lambda**

```rust
use metaxy::rpc_query;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Serialize)]
pub struct DedupDemoResponse {
    pub request_number: u64,
    pub timestamp: String,
}

/// Slow endpoint (500ms) with a request counter.
/// Used to demonstrate request deduplication.
#[rpc_query]
async fn dedup_demo() -> DedupDemoResponse {
    let num = REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    DedupDemoResponse {
        request_number: num,
        timestamp: format!("{}ms", now),
    }
}
```

**Step 2: Commit**

```bash
git add demo/api/dedup_demo.rs
git commit -m "feat(metaxy): add dedup_demo lambda for deduplication demo"
```

---

## Task 3: Create `cookie_demo.rs` lambda

**Files:**
- Create: `demo/api/cookie_demo.rs`

**Step 1: Write the lambda**

```rust
use metaxy::{Headers, rpc_query};
use serde::Serialize;

#[derive(Serialize)]
pub struct CookieDemoResponse {
    pub authenticated: bool,
    pub message: String,
    pub cookie_value: Option<String>,
}

/// Checks for a `session` cookie in the request headers.
/// Returns OK if present, error details if missing.
#[rpc_query]
async fn cookie_demo(headers: Headers) -> CookieDemoResponse {
    let cookies = headers.get("cookie").and_then(|v| v.to_str().ok());
    let session = cookies
        .and_then(|c| {
            c.split(';')
                .map(|s| s.trim())
                .find(|s| s.starts_with("session="))
                .map(|s| s.trim_start_matches("session=").to_string())
        });

    match session {
        Some(val) => CookieDemoResponse {
            authenticated: true,
            message: "Authenticated via session cookie".into(),
            cookie_value: Some(val),
        },
        None => CookieDemoResponse {
            authenticated: false,
            message: "No session cookie found — client fetch does not forward server cookies".into(),
            cookie_value: None,
        },
    }
}
```

**Step 2: Commit**

```bash
git add demo/api/cookie_demo.rs
git commit -m "feat(metaxy): add cookie_demo lambda for custom fetch demo"
```

---

## Task 4: Regenerate types and client

**Step 1: Run codegen**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy/demo
cargo run --manifest-path ../crates/metaxy-cli/Cargo.toml -- generate
```

**Step 2: Verify new types appeared**

Check that `demo/src/lib/rpc-types.ts` now includes `RetryDemoInput`, `RetryDemoResponse`, `DedupDemoResponse`, `CookieDemoResponse`.
Check that `demo/src/lib/rpc-client.ts` now includes `retry_demo`, `dedup_demo`, `cookie_demo` in the Procedures map.

**Step 3: Commit**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
git add demo/src/lib/rpc-types.ts demo/src/lib/rpc-client.ts
git commit -m "chore(metaxy): regenerate types and client after new demo lambdas"
```

---

## Task 5: Interactive demo — Headers page

**Files:**
- Modify: `demo/src/routes/docs/client/headers/+page.svelte`
- Modify: `demo/src/routes/docs/client/headers/+page.server.ts`

**Step 1: Add Rust code block to `+page.server.ts`**

Add a `secretRust` code block with the `secret.rs` source to the existing `codeBlocks` object:

```typescript
secretRust: {
    lang: 'rust',
    code: `#[rpc_query]
async fn secret(headers: Headers) -> Result<String, String> {
    let auth = headers.get("authorization")
        .and_then(|v| v.to_str().ok());
    if auth != Some("Bearer secret-token-123") {
        return Err("Unauthorized: invalid or missing token".into());
    }
    Ok("Top secret: the cake is a lie.".to_string())
}`
},
```

**Step 2: Add interactive "Try it" section to `+page.svelte`**

Add `import { rpc } from '$lib/client';` and `import { RpcError } from '$lib/rpc-client';` at the top. Add state variables and demo UI after the existing doc content:

```svelte
<script lang="ts">
	import { rpc } from '$lib/client';
	import { createRpcClient, RpcError } from '$lib/rpc-client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let result: string | undefined = $state();
	let error: string | undefined = $state();
	let loading = $state(false);
	let usedHeaders: Record<string, string> = $state({});

	async function fetchSecret(withToken: boolean) {
		loading = true;
		result = undefined;
		error = undefined;
		const headers = withToken ? { Authorization: 'Bearer secret-token-123' } : {};
		usedHeaders = headers;
		try {
			result = await rpc.query('secret', undefined, { headers });
		} catch (e) {
			error = e instanceof RpcError ? `${e.status}: ${e.message}` : String(e);
		} finally {
			loading = false;
		}
	}

	let openCode = $state(false);
</script>
```

"Try it" section HTML (add before closing `</div>`):

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    The <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">secret</code> endpoint requires
    a Bearer token. Try with and without it.
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6">
    <div class="flex items-center gap-2 mb-4">
        <button
            onclick={() => fetchSecret(false)}
            disabled={loading}
            class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >Without token</button
        >
        <button
            onclick={() => fetchSecret(true)}
            disabled={loading}
            class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >With token</button
        >
        {#if loading}
            <span class="text-sm text-text-muted">Loading...</span>
        {/if}
    </div>

    {#if result || error}
        <div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1">
            <div class="text-text-faint">
                Headers: {JSON.stringify(usedHeaders)}
            </div>
            {#if result}
                <div class="text-green-400">200 OK → {JSON.stringify(result)}</div>
            {:else if error}
                <div class="text-red-400">{error}</div>
            {/if}
        </div>
    {/if}

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['secretRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/headers/
git commit -m "feat(metaxy): add interactive headers demo with auth token"
```

---

## Task 6: Interactive demo — Timeout & Abort page

**Files:**
- Modify: `demo/src/routes/docs/client/timeout/+page.svelte`
- Modify: `demo/src/routes/docs/client/timeout/+page.server.ts`

**Step 1: Add Rust code block to `+page.server.ts`**

```typescript
timeoutDemoRust: {
    lang: 'rust',
    code: `#[rpc_query(timeout = "3s")]
async fn timeout_demo(input: TimeoutDemoInput) -> TimeoutDemoResponse {
    let start = Instant::now();
    tokio::time::sleep(Duration::from_millis(input.sleep_ms)).await;
    TimeoutDemoResponse {
        requested_ms: input.sleep_ms,
        actual_ms: start.elapsed().as_millis() as u64,
        timeout_ms: 3000,
    }
}`
},
```

**Step 2: Replace `+page.svelte` with interactive version**

The demo shows two scenarios:
1. **Client-side timeout**: calls `timeout_demo` with `sleep_ms=1000` but client timeout=500ms → AbortError
2. **Manual abort**: starts a slow call, user clicks Abort

```svelte
<script lang="ts">
	import { createRpcClient } from '$lib/rpc-client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	type LogEntry = { label: string; status: 'ok' | 'abort' | 'error'; detail: string };
	let callLog: LogEntry[] = $state([]);
	let loading = $state(false);
	let abortController: AbortController | undefined = $state();

	async function runClientTimeout(serverDelayMs: number, clientTimeoutMs: number) {
		loading = true;
		const client = createRpcClient({ baseUrl: '/api', timeout: clientTimeoutMs });
		try {
			const res = await client.query('timeout_demo', { sleep_ms: serverDelayMs });
			callLog = [...callLog.slice(-4), {
				label: `delay=${serverDelayMs}ms, timeout=${clientTimeoutMs}ms`,
				status: 'ok',
				detail: `200 OK — actual ${res.actual_ms}ms`
			}];
		} catch (e) {
			const isAbort = e instanceof DOMException && e.name === 'AbortError';
			callLog = [...callLog.slice(-4), {
				label: `delay=${serverDelayMs}ms, timeout=${clientTimeoutMs}ms`,
				status: isAbort ? 'abort' : 'error',
				detail: isAbort ? 'AbortError: client timeout exceeded' : String(e)
			}];
		} finally {
			loading = false;
		}
	}

	async function runManualAbort() {
		loading = true;
		abortController = new AbortController();
		const client = createRpcClient({ baseUrl: '/api' });
		try {
			const res = await client.query('timeout_demo', { sleep_ms: 10000 }, {
				signal: abortController.signal
			});
			callLog = [...callLog.slice(-4), {
				label: 'manual abort (10s delay)',
				status: 'ok',
				detail: `200 OK — actual ${res.actual_ms}ms`
			}];
		} catch (e) {
			const isAbort = e instanceof DOMException && e.name === 'AbortError';
			callLog = [...callLog.slice(-4), {
				label: 'manual abort (10s delay)',
				status: isAbort ? 'abort' : 'error',
				detail: isAbort ? 'AbortError: manually cancelled' : String(e)
			}];
		} finally {
			loading = false;
			abortController = undefined;
		}
	}

	let openCode = $state(false);
</script>
```

"Try it" HTML section (add after existing doc content, before closing `</div>`):

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    The server sleeps for the requested duration. Client-side timeout fires an
    <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">AbortError</code> before the server responds.
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6">
    <h3 class="text-lg font-semibold mb-1">Client Timeout</h3>
    <p class="text-text-muted text-xs mb-3">
        Server delay 1s, client timeout 2s (OK) vs server delay 1s, client timeout 300ms (abort).
    </p>
    <div class="flex items-center gap-2 mb-4 flex-wrap">
        <button
            onclick={() => runClientTimeout(1000, 2000)}
            disabled={loading}
            class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >1s delay, 2s timeout</button
        >
        <button
            onclick={() => runClientTimeout(1000, 300)}
            disabled={loading}
            class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >1s delay, 300ms timeout</button
        >
    </div>

    <h3 class="text-lg font-semibold mb-1">Manual Abort</h3>
    <p class="text-text-muted text-xs mb-3">Start a 10s request, then cancel it with AbortController.</p>
    <div class="flex items-center gap-2 mb-4">
        {#if abortController}
            <button
                onclick={() => abortController?.abort()}
                class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85"
                >Abort now</button
            >
            <span class="text-sm text-text-muted">Request in flight...</span>
        {:else}
            <button
                onclick={runManualAbort}
                disabled={loading}
                class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
                >Start 10s request</button
            >
        {/if}
    </div>

    {#if callLog.length > 0}
        <div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
            {#each callLog as entry, i (i)}
                <div class="flex gap-4">
                    <span class="text-text-faint">#{i + 1}</span>
                    <span class="text-text-muted">{entry.label}</span>
                    {#if entry.status === 'ok'}
                        <span class="text-green-400">{entry.detail}</span>
                    {:else if entry.status === 'abort'}
                        <span class="text-red-400">{entry.detail}</span>
                    {:else}
                        <span class="text-yellow-400">{entry.detail}</span>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['timeoutDemoRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/timeout/
git commit -m "feat(metaxy): add interactive client timeout & abort demo"
```

---

## Task 7: Interactive demo — Retry page

**Files:**
- Modify: `demo/src/routes/docs/client/retry/+page.svelte`
- Modify: `demo/src/routes/docs/client/retry/+page.server.ts`

**Step 1: Add Rust code block to `+page.server.ts`**

```typescript
retryDemoRust: {
    lang: 'rust',
    code: `static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Returns 500 for the first \`fail_count\` calls, then 200.
#[rpc_query]
async fn retry_demo(input: RetryDemoInput) -> Result<RetryDemoResponse, String> {
    if input.reset { CALL_COUNT.store(0, Ordering::Relaxed); }
    let call_number = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if call_number <= input.fail_count {
        return Err(format!("Simulated failure (call {})", call_number));
    }
    Ok(RetryDemoResponse {
        call_number,
        total_calls: CALL_COUNT.load(Ordering::Relaxed),
        message: format!("Success on call {}", call_number),
    })
}`
},
```

**Step 2: Update `+page.svelte` with interactive demo**

Add imports, state, and the demo function. Create a temporary client with retry + hooks to capture the log:

```svelte
<script lang="ts">
	import { createRpcClient, RpcError } from '$lib/rpc-client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	type LogEntry = { attempt: number; status: 'pending' | 'fail' | 'success'; detail: string; ts: number };
	let retryLog: LogEntry[] = $state([]);
	let loading = $state(false);
	let failCount = $state(2);

	async function runRetry() {
		loading = true;
		retryLog = [];
		const startTs = Date.now();
		const log: LogEntry[] = [];

		const client = createRpcClient({
			baseUrl: '/api',
			retry: { attempts: 3, delay: 500 },
			onRequest: (ctx) => {
				log.push({
					attempt: log.filter(e => e.status !== 'pending').length,
					status: 'pending',
					detail: `onRequest → attempt ${log.filter(e => e.status !== 'pending').length}`,
					ts: Date.now() - startTs
				});
				retryLog = [...log];
			},
			onError: (ctx) => {
				log.push({
					attempt: ctx.attempt,
					status: 'fail',
					detail: `onError → ${ctx.error instanceof RpcError ? ctx.error.status : 'err'} ${ctx.willRetry ? '(will retry)' : '(final)'}`,
					ts: Date.now() - startTs
				});
				retryLog = [...log];
			},
			onResponse: (ctx) => {
				log.push({
					attempt: log.filter(e => e.status === 'fail').length,
					status: 'success',
					detail: `onResponse → 200 OK (${ctx.duration}ms)`,
					ts: Date.now() - startTs
				});
				retryLog = [...log];
			}
		});

		try {
			await client.query('retry_demo', { fail_count: failCount, reset: true });
		} catch {
			// final error already logged via onError
		} finally {
			loading = false;
		}
	}

	let openCode = $state(false);
</script>
```

"Try it" HTML (after existing doc content):

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    The server fails the first N calls, then succeeds. The client retries up to 3 times with 500ms delay.
    Lifecycle hooks log every step.
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6">
    <div class="flex items-center gap-3 mb-4">
        <label class="text-sm text-text-muted">
            Fail first
            <select bind:value={failCount} class="ml-1 rounded bg-bg-code px-2 py-1 text-xs font-mono text-text-primary">
                <option value={1}>1</option>
                <option value={2}>2</option>
                <option value={3}>3</option>
                <option value={4}>4 (all retries fail)</option>
            </select>
            requests
        </label>
        <button
            onclick={runRetry}
            disabled={loading}
            class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >Fetch with retry</button
        >
    </div>

    {#if retryLog.length > 0}
        <div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
            {#each retryLog as entry, i (i)}
                <div class="flex gap-4">
                    <span class="text-text-faint w-12 text-right">{entry.ts}ms</span>
                    {#if entry.status === 'pending'}
                        <span class="text-blue-400">{entry.detail}</span>
                    {:else if entry.status === 'fail'}
                        <span class="text-red-400">{entry.detail}</span>
                    {:else}
                        <span class="text-green-400">{entry.detail}</span>
                    {/if}
                </div>
            {/each}
        </div>
    {/if}

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['retryDemoRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/retry/
git commit -m "feat(metaxy): add interactive retry demo with attempt log"
```

---

## Task 8: Interactive demo — Deduplication page

**Files:**
- Modify: `demo/src/routes/docs/client/deduplication/+page.svelte`
- Modify: `demo/src/routes/docs/client/deduplication/+page.server.ts`

**Step 1: Add Rust code block to `+page.server.ts`**

```typescript
dedupDemoRust: {
    lang: 'rust',
    code: `static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

/// Slow endpoint (500ms) with a request counter.
#[rpc_query]
async fn dedup_demo() -> DedupDemoResponse {
    let num = REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    tokio::time::sleep(Duration::from_millis(500)).await;
    DedupDemoResponse { request_number: num, timestamp: format!("{}ms", now) }
}`
},
```

**Step 2: Update `+page.svelte` with interactive demo**

```svelte
<script lang="ts">
	import { rpc } from '$lib/client';
	import { createRpcClient } from '$lib/rpc-client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	type DedupResult = { mode: string; calls: number; serverRequests: string; results: string[] };
	let dedupResult: DedupResult | undefined = $state();
	let loading = $state(false);

	async function fireCalls(dedupe: boolean) {
		loading = true;
		dedupResult = undefined;
		const client = dedupe
			? rpc
			: createRpcClient({ baseUrl: '/api', dedupe: false });

		const N = 5;
		const promises = Array.from({ length: N }, () => client.query('dedup_demo'));
		const results = await Promise.all(promises);

		const requestNumbers = results.map(r => r.request_number);
		const unique = new Set(requestNumbers);

		dedupResult = {
			mode: dedupe ? 'dedupe: true (default)' : 'dedupe: false',
			calls: N,
			serverRequests: `${unique.size} unique server request(s)`,
			results: results.map((r, i) => `call ${i + 1} → request #${r.request_number}`)
		};
		loading = false;
	}

	let openCode = $state(false);
</script>
```

"Try it" HTML:

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    Fire 5 identical queries concurrently. With dedup enabled, only 1 HTTP request is made.
    The server sleeps 500ms and increments a counter on each real request.
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6">
    <div class="flex items-center gap-2 mb-4 flex-wrap">
        <button
            onclick={() => fireCalls(true)}
            disabled={loading}
            class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >5 calls (dedupe on)</button
        >
        <button
            onclick={() => fireCalls(false)}
            disabled={loading}
            class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >5 calls (dedupe off)</button
        >
        {#if loading}
            <span class="text-sm text-text-muted">Loading...</span>
        {/if}
    </div>

    {#if dedupResult}
        <div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1">
            <div class="text-text-faint">{dedupResult.mode}</div>
            <div class="text-text-muted">{dedupResult.calls} calls → {dedupResult.serverRequests}</div>
            {#each dedupResult.results as line, i (i)}
                <div class="text-accent-ts">{line}</div>
            {/each}
        </div>
    {/if}

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['dedupDemoRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/deduplication/
git commit -m "feat(metaxy): add interactive deduplication demo"
```

---

## Task 9: Interactive demo — Hooks page

**Files:**
- Modify: `demo/src/routes/docs/client/hooks/+page.svelte`
- Modify: `demo/src/routes/docs/client/hooks/+page.server.ts`

**Step 1: Add Rust code block to `+page.server.ts`**

Reuse the same `retryDemoRust` block from Task 7 (copy it here, or reference — implementor should copy the same block).

```typescript
retryDemoRust: {
    lang: 'rust',
    code: `static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Returns 500 for the first \`fail_count\` calls, then 200.
#[rpc_query]
async fn retry_demo(input: RetryDemoInput) -> Result<RetryDemoResponse, String> {
    if input.reset { CALL_COUNT.store(0, Ordering::Relaxed); }
    let call_number = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if call_number <= input.fail_count {
        return Err(format!("Simulated failure (call {})", call_number));
    }
    Ok(RetryDemoResponse { call_number, total_calls: CALL_COUNT.load(Ordering::Relaxed), message: format!("Success on call {}", call_number) })
}`
},
```

**Step 2: Update `+page.svelte` with lifecycle log demo**

```svelte
<script lang="ts">
	import { createRpcClient, RpcError } from '$lib/rpc-client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	type HookEntry = { hook: 'onRequest' | 'onResponse' | 'onError'; detail: string; ts: number };
	let hookLog: HookEntry[] = $state([]);
	let loading = $state(false);

	async function runHooksDemo() {
		loading = true;
		hookLog = [];
		const startTs = Date.now();
		const log: HookEntry[] = [];

		const client = createRpcClient({
			baseUrl: '/api',
			retry: { attempts: 2, delay: 300 },
			onRequest: (ctx) => {
				log.push({ hook: 'onRequest', detail: `procedure="${ctx.procedure}" url="${ctx.url}"`, ts: Date.now() - startTs });
				hookLog = [...log];
			},
			onResponse: (ctx) => {
				log.push({ hook: 'onResponse', detail: `status=200 duration=${ctx.duration}ms data=${JSON.stringify(ctx.data).slice(0, 80)}`, ts: Date.now() - startTs });
				hookLog = [...log];
			},
			onError: (ctx) => {
				log.push({ hook: 'onError', detail: `attempt=${ctx.attempt} willRetry=${ctx.willRetry} error="${ctx.error instanceof RpcError ? ctx.error.message : ctx.error}"`, ts: Date.now() - startTs });
				hookLog = [...log];
			}
		});

		try {
			await client.query('retry_demo', { fail_count: 1, reset: true });
		} catch {
			// error logged via onError
		} finally {
			loading = false;
		}
	}

	let openCode = $state(false);
</script>
```

"Try it" HTML:

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    Calls an endpoint that fails once, then succeeds. All three hooks fire and log their context.
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6">
    <div class="flex items-center gap-2 mb-4">
        <button
            onclick={runHooksDemo}
            disabled={loading}
            class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >Run lifecycle demo</button
        >
        {#if loading}
            <span class="text-sm text-text-muted">Loading...</span>
        {/if}
    </div>

    {#if hookLog.length > 0}
        <div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
            {#each hookLog as entry, i (i)}
                <div class="flex gap-4">
                    <span class="text-text-faint w-12 text-right">{entry.ts}ms</span>
                    {#if entry.hook === 'onRequest'}
                        <span class="text-blue-400">onRequest</span>
                    {:else if entry.hook === 'onResponse'}
                        <span class="text-green-400">onResponse</span>
                    {:else}
                        <span class="text-red-400">onError</span>
                    {/if}
                    <span class="text-text-muted">{entry.detail}</span>
                </div>
            {/each}
        </div>
    {/if}

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['retryDemoRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/hooks/
git commit -m "feat(metaxy): add interactive lifecycle hooks demo"
```

---

## Task 10: Interactive demo — Serialization page

**Files:**
- Modify: `demo/src/routes/docs/client/serialization/+page.svelte`
- Modify: `demo/src/routes/docs/client/serialization/+page.server.ts`

**Step 1: Add Rust code block to `+page.server.ts`**

```typescript
bigintDemoRust: {
    lang: 'rust',
    code: `#[derive(Serialize)]
pub struct BigIntDemoValue {
    pub label: String,
    pub exact: String,
    pub as_number: u64,
}

#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let cases: &[(&str, u64)] = &[
        ("small (42)", 42),
        ("MAX_SAFE_INTEGER", 9_007_199_254_740_991),
        ("MAX_SAFE + 2", 9_007_199_254_740_993),
        ("u64::MAX", u64::MAX),
    ];
    // ...
}`
},
```

**Step 2: Update `+page.svelte` with interactive demo**

Reuses `bigint_demo` endpoint. Shows JSON.parse vs lossless-json side by side (similar to bigint page but framed as serialization).

```svelte
<script lang="ts">
	import { rpc } from '$lib/client';
	import { type BigIntDemoResponse, type BigIntDemoValue, createRpcClient } from '$lib/rpc-client';
	import { parse, parseNumberAndBigInt } from 'lossless-json';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	const losslessRpc = createRpcClient({
		baseUrl: '/api',
		deserialize: (text) => parse(text, undefined, parseNumberAndBigInt)
	});

	let defaultResult: BigIntDemoResponse | undefined = $state();
	let losslessResult: BigIntDemoValue[] | undefined = $state();
	let loading = $state(false);

	async function fetchDemo() {
		loading = true;
		try {
			const [def, lossless] = await Promise.all([
				rpc.query('bigint_demo'),
				losslessRpc.query('bigint_demo')
			]);
			defaultResult = def;
			losslessResult = lossless.values;
		} finally {
			loading = false;
		}
	}

	let openCode = $state(false);
</script>
```

"Try it" HTML:

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    Same endpoint, two clients. The default uses <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.parse</code>,
    the custom one uses <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">lossless-json</code>
    via the <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">deserialize</code> option.
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6">
    <div class="flex items-center gap-3 mb-4">
        <button
            onclick={fetchDemo}
            disabled={loading}
            class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
            >Fetch with both clients</button
        >
        {#if loading}
            <span class="text-sm text-text-muted">Loading...</span>
        {/if}
    </div>

    {#if defaultResult && losslessResult}
        <div class="overflow-x-auto rounded-md border border-border">
            <table class="w-full text-xs font-mono">
                <thead class="bg-bg-code text-text-faint">
                    <tr>
                        <th class="px-3 py-2 text-left">Label</th>
                        <th class="px-3 py-2 text-left">Server (exact)</th>
                        <th class="px-3 py-2 text-left">JSON.parse</th>
                        <th class="px-3 py-2 text-left">typeof</th>
                        <th class="px-3 py-2 text-left">lossless-json</th>
                        <th class="px-3 py-2 text-left">typeof</th>
                    </tr>
                </thead>
                <tbody>
                    {#each defaultResult.values as row, i (i)}
                        {@const losslessVal = losslessResult[i]?.as_number}
                        {@const defaultLost = String(row.as_number) !== row.exact}
                        {@const losslessLost = String(losslessVal) !== row.exact}
                        <tr class="border-t border-border/50">
                            <td class="px-3 py-2 text-text-muted">{row.label}</td>
                            <td class="px-3 py-2 text-accent-rust">{row.exact}</td>
                            <td class="px-3 py-2" class:text-red-400={defaultLost} class:text-green-400={!defaultLost}>{row.as_number}</td>
                            <td class="px-3 py-2 text-text-faint">{typeof row.as_number}</td>
                            <td class="px-3 py-2" class:text-red-400={losslessLost} class:text-green-400={!losslessLost}>{String(losslessVal)}</td>
                            <td class="px-3 py-2 text-text-faint">{typeof losslessVal}</td>
                        </tr>
                    {/each}
                </tbody>
            </table>
        </div>
    {/if}

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['bigintDemoRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/serialization/
git commit -m "feat(metaxy): add interactive serialization demo with lossless-json"
```

---

## Task 11: Interactive demo — Custom Fetch page

**Files:**
- Modify: `demo/src/routes/docs/client/fetch/+page.svelte`
- Modify: `demo/src/routes/docs/client/fetch/+page.server.ts`
- Create: `demo/src/routes/docs/client/fetch/+page.server.ts` (modify existing — add server load that calls cookie_demo via event.fetch)

**Step 1: Update `+page.server.ts` to do SSR fetch**

Replace the existing server load to also call the cookie_demo endpoint via `event.fetch` (which forwards cookies):

```typescript
import { highlightCode } from '$lib/highlight.server';
import { createRpcClient } from '$lib/rpc-client';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' | 'rust' }> = {
	defaultFetch: {
		lang: 'typescript',
		code: `// Default: uses globalThis.fetch
const rpc = createRpcClient({ baseUrl: '/api' });`
	},
	ssrFetch: {
		lang: 'typescript',
		code: `// SvelteKit server load — forward cookies & resolve relative URLs
export const load: PageServerLoad = async ({ fetch }) => {
  const rpc = createRpcClient({ baseUrl: '/api', fetch });
  const data = await rpc.query('cookie_demo');
  return { ssrResult: data };
};`
	},
	testFetch: {
		lang: 'typescript',
		code: `// Mock fetch for unit tests
const mockFetch = async (url: string) =>
  new Response(JSON.stringify({ ok: true }), { status: 200 });

const rpc = createRpcClient({ baseUrl: '/api', fetch: mockFetch });`
	},
	cookieDemoRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn cookie_demo(headers: Headers) -> CookieDemoResponse {
    let session = headers.get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').find(|s| s.trim().starts_with("session=")))
        .map(|s| s.trim().trim_start_matches("session=").to_string());

    match session {
        Some(val) => CookieDemoResponse {
            authenticated: true,
            message: "Authenticated via session cookie".into(),
            cookie_value: Some(val),
        },
        None => CookieDemoResponse {
            authenticated: false,
            message: "No session cookie — client fetch doesn't forward server cookies".into(),
            cookie_value: None,
        },
    }
}`
	}
};

export const load: PageServerLoad = async ({ fetch }) => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(entries.map(([, { code, lang }]) => highlightCode(code, lang)));
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});

	// SSR call with platform fetch (forwards cookies)
	let ssrResult: unknown = null;
	try {
		const ssrRpc = createRpcClient({ baseUrl: '/api', fetch });
		ssrResult = await ssrRpc.query('cookie_demo');
	} catch (e) {
		ssrResult = { authenticated: false, message: `SSR error: ${e}`, cookie_value: null };
	}

	return { highlighted, ssrResult };
};
```

**Step 2: Update `+page.svelte`**

```svelte
<script lang="ts">
	import { rpc } from '$lib/client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let clientResult: { authenticated: boolean; message: string; cookie_value: string | null } | undefined = $state();
	let loading = $state(false);

	async function fetchFromClient() {
		loading = true;
		try {
			clientResult = await rpc.query('cookie_demo');
		} catch (e) {
			clientResult = { authenticated: false, message: String(e), cookie_value: null };
		} finally {
			loading = false;
		}
	}

	function setCookie() {
		document.cookie = 'session=demo-session-xyz; path=/; max-age=60';
		fetchFromClient();
	}

	function clearCookie() {
		document.cookie = 'session=; path=/; max-age=0';
		clientResult = undefined;
	}

	let openCode = $state(false);
</script>
```

Keep existing doc sections, then add "Try it":

```svelte
<!-- Try it -->
<h2 class="text-2xl font-bold mt-12">Try it</h2>
<p class="text-text-muted text-sm">
    The server checks for a <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">session</code> cookie.
    SSR <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">event.fetch</code> forwards it automatically;
    browser <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code> does not (on Vercel, cross-origin).
</p>

<div class="rounded-lg border border-border bg-bg-soft p-6 space-y-4">
    <div>
        <h3 class="text-lg font-semibold mb-1">SSR Result</h3>
        <p class="text-text-muted text-xs mb-2">Called during server load with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">event.fetch</code></p>
        <div class="rounded-md bg-bg-code p-3 text-xs font-mono">
            {#if data.ssrResult}
                {@const ssr = data.ssrResult as { authenticated: boolean; message: string }}
                <span class={ssr.authenticated ? 'text-green-400' : 'text-red-400'}>
                    {ssr.authenticated ? '✓' : '✗'} {ssr.message}
                </span>
            {/if}
        </div>
    </div>

    <div>
        <h3 class="text-lg font-semibold mb-1">Client Result</h3>
        <p class="text-text-muted text-xs mb-2">Called from browser with default <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">globalThis.fetch</code></p>
        <div class="flex items-center gap-2 mb-2 flex-wrap">
            <button
                onclick={fetchFromClient}
                disabled={loading}
                class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
                >Fetch from client</button
            >
            <button
                onclick={setCookie}
                class="rounded-md bg-green-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85"
                >Set cookie & fetch</button
            >
            <button
                onclick={clearCookie}
                class="rounded-md bg-bg-code px-3 py-1.5 text-sm font-medium text-text-muted transition-opacity hover:opacity-85"
                >Clear cookie</button
            >
        </div>
        {#if clientResult}
            <div class="rounded-md bg-bg-code p-3 text-xs font-mono">
                <span class={clientResult.authenticated ? 'text-green-400' : 'text-red-400'}>
                    {clientResult.authenticated ? '✓' : '✗'} {clientResult.message}
                </span>
            </div>
        {/if}
    </div>

    <button
        class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
        onclick={() => (openCode = !openCode)}
    >
        {openCode ? '▾ Hide' : '▸ Show'} Rust
    </button>
    {#if openCode}
        <div class="mt-3">
            <CodeBlock html={data.highlighted['cookieDemoRust']} />
        </div>
    {/if}
</div>
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/client/fetch/
git commit -m "feat(metaxy): add interactive custom fetch demo with SSR vs client cookies"
```

---

## Task 12: Verify everything builds

**Step 1: Run SvelteKit dev build check**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy/demo
npm run check
```

Expected: no type errors.

**Step 2: Fix any issues found**

Adjust imports, types, or code as needed.

**Step 3: Final commit if fixes were needed**

```bash
git add -u
git commit -m "fix(metaxy): fix build issues in client interactive demos"
```
