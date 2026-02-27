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

<svelte:head>
	<title>Retry — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Retry</h1>
	<p class="text-text-muted leading-relaxed">
		Queries are retried automatically on network errors or retryable HTTP status codes. Mutations
		are <strong>never</strong> retried unless explicitly marked as
		<a href="/docs/macros/idempotent" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">idempotent</code></a
		>.
	</p>

	<h2 class="text-xl font-semibold">RetryPolicy</h2>
	<CodeBlock html={data.highlighted['retryConfig']} />

	<p class="text-text-muted leading-relaxed text-sm">
		A request is retried when a network error occurs or the response status is in
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">retryOn</code>, up to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">attempts</code> additional
		tries. On each retry the full
		<a href="/docs/client/hooks" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">onRequest</code></a
		> hook runs again, so dynamic headers (e.g. refreshed auth tokens) are re-evaluated.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['retryBasic']} />

	<h2 class="text-xl font-semibold">Exponential Backoff</h2>
	<CodeBlock html={data.highlighted['retryExponential']} />

	<h2 class="text-xl font-semibold">Custom Retry Logic</h2>
	<CodeBlock html={data.highlighted['retryCustom']} />

	<h2 class="text-xl font-semibold">Idempotent Mutations</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		By default, mutations are never retried — even with a retry policy configured. To opt a mutation
		into retry, mark it as
		<a href="/docs/macros/idempotent" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">idempotent</code></a
		>
		in the Rust macro. This signals that repeated calls produce the same result.
	</p>
	<CodeBlock html={data.highlighted['retryIdempotent']} />

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
</div>
