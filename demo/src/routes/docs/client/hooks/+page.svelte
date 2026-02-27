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

<svelte:head>
	<title>Lifecycle Hooks — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Lifecycle Hooks</h1>
	<p class="text-text-muted leading-relaxed">
		Three hooks let you intercept the request lifecycle: before the fetch, after a successful
		response, and on failure.
	</p>

	<h2 class="text-xl font-semibold">Overview</h2>
	<CodeBlock html={data.highlighted['allHooks']} />

	<h2 class="text-2xl font-semibold mt-8">onRequest</h2>
	<p class="text-text-muted text-sm mb-2">
		Fires before the fetch. You can mutate <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ctx.headers</code
		>
		to add or override headers dynamically. Runs again on every retry attempt.
	</p>
	<CodeBlock html={data.highlighted['onRequest']} />
	<CodeBlock html={data.highlighted['requestCtx']} />

	<h2 class="text-2xl font-semibold mt-8">onResponse</h2>
	<p class="text-text-muted text-sm mb-2">
		Fires after a successful response. Use it for logging, metrics, or cache warming.
	</p>
	<CodeBlock html={data.highlighted['onResponse']} />
	<CodeBlock html={data.highlighted['responseCtx']} />

	<h2 class="text-2xl font-semibold mt-8">onError</h2>
	<p class="text-text-muted text-sm mb-2">
		Fires on failure. Check <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>ctx.willRetry</code
		>
		to know if the client will retry, or use
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ctx.attempt</code>
		to track retry progress.
	</p>
	<CodeBlock html={data.highlighted['onError']} />
	<CodeBlock html={data.highlighted['errorCtx']} />

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
</div>
