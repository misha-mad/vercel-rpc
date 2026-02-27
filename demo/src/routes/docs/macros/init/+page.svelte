<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	const initDemo = createQuery(rpc, 'init_demo');

	let callLog: { coldStartAt: number; initMs: number; reqCount: number; now: number }[] = $state(
		[]
	);

	function logCall(d: typeof initDemo.data) {
		if (d) {
			callLog = [
				...callLog.slice(-4),
				{
					coldStartAt: d.cold_start_at,
					initMs: d.init_duration_ms,
					reqCount: d.request_count,
					now: d.now
				}
			];
		}
	}

	async function refetch() {
		await initDemo.refetch();
		logCall(initDemo.data);
	}

	$effect(() => {
		if (initDemo.data) logCall(initDemo.data);
	});

	let openCode = $state(false);
</script>

<svelte:head>
	<title>init — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">init</h1>
	<p class="text-text-muted leading-relaxed">
		Run a function once at cold start. Can be side-effects only (logger, dotenv) or return shared
		state (DB pool, HTTP client) injected as
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&T</code> parameter. Works with both
		queries and mutations.
	</p>

	<h2 class="text-xl font-semibold">Side-Effects Only</h2>
	<p class="text-text-muted text-sm mb-2">
		When the init function returns nothing, it runs once for setup (logging, env vars, tracing).
	</p>
	<CodeBlock html={data.highlighted['sideEffect']} />

	<h2 class="text-xl font-semibold">Shared State</h2>
	<p class="text-text-muted text-sm mb-2">
		When the init function returns a value, it's stored and injected as
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&T</code> into the handler. The init
		function runs once per cold start — the result is reused across requests.
	</p>
	<CodeBlock html={data.highlighted['sharedState']} />

	<h2 class="text-xl font-semibold">HTTP Client</h2>
	<CodeBlock html={data.highlighted['httpClient']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Cold Start &amp; Shared State</h3>
		<p class="text-text-muted text-sm mb-3">
			The <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">setup()</code> function runs
			once at cold start, measuring its own duration. Subsequent requests reuse the same state — watch
			the request count increment while cold start time stays the same.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<button
				onclick={refetch}
				disabled={initDemo.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Refetch</button
			>
		</div>
		{#if callLog.length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
				{#each callLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>init: <span class="text-accent-rust">{entry.initMs}ms</span></span
						>
						<span class="text-text-muted"
							>req: <span class="text-text-primary">{entry.reqCount}</span></span
						>
						{#if i > 0 && entry.coldStartAt === callLog[i - 1].coldStartAt}
							<span class="text-green-400">same instance</span>
						{:else if i > 0}
							<span class="text-yellow-400">new cold start</span>
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
				<CodeBlock html={data.highlighted['initDemoRust']} />
			</div>
		{/if}
	</div>
</div>
