<script lang="ts">
	import { rpc } from '$lib/client';
	import { RpcError } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let loading = $state(false);

	let callLog: {
		sleepMs: number;
		actualMs: number | null;
		status: 'ok' | 'timeout' | 'error';
		error?: string;
	}[] = $state([]);

	async function run(ms: number) {
		loading = true;
		try {
			const result = await rpc.query('timeout_demo', { sleep_ms: ms });
			callLog = [
				...callLog.slice(-4),
				{ sleepMs: ms, actualMs: result.actual_ms, status: 'ok' }
			];
		} catch (e) {
			if (e instanceof RpcError && e.status === 504) {
				callLog = [
					...callLog.slice(-4),
					{ sleepMs: ms, actualMs: null, status: 'timeout', error: '504 Gateway Timeout' }
				];
			} else {
				callLog = [
					...callLog.slice(-4),
					{
						sleepMs: ms,
						actualMs: null,
						status: 'error',
						error: e instanceof Error ? e.message : String(e)
					}
				];
			}
		} finally {
			loading = false;
		}
	}

	let openCode = $state(false);
</script>

<svelte:head>
	<title>timeout — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">timeout</h1>
	<p class="text-text-muted leading-relaxed">
		Server-side timeout via
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">tokio::time::timeout</code>.
		Returns HTTP 504 if exceeded. Also forwarded to the TypeScript client as the default timeout for
		that procedure. Works with both queries and mutations.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Duration Shorthand</h2>
	<CodeBlock html={data.highlighted['durations']} />

	<h2 class="text-xl font-semibold">Behavior</h2>
	<p class="text-text-muted text-sm mb-2">
		When the handler exceeds the timeout, the future is cancelled and the server returns 504. The
		TypeScript client receives an
		<a href="/docs/error-handling" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code></a
		> with status 504.
	</p>
	<CodeBlock html={data.highlighted['behavior']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Server-Side Timeout</h3>
		<p class="text-text-muted text-sm mb-3">
			This handler has a <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
				>timeout = "3s"</code
			> — it sleeps for the requested duration. Try a short sleep to see a successful response, then try
			exceeding 3 seconds to trigger a 504.
		</p>
		<div class="flex items-center gap-2 mb-3 flex-wrap">
			<button
				onclick={() => run(500)}
				disabled={loading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>500ms</button
			>
			<button
				onclick={() => run(2000)}
				disabled={loading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>2s</button
			>
			<button
				onclick={() => run(4000)}
				disabled={loading}
				class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>4s (timeout!)</button
			>
		</div>
		{#if loading}
			<div class="text-xs text-text-faint mb-2">waiting for response...</div>
		{/if}
		{#if callLog.length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
				{#each callLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>sleep: <span class="text-text-primary">{entry.sleepMs}ms</span></span
						>
						{#if entry.status === 'ok'}
							<span class="text-text-muted"
								>actual: <span class="text-accent-rust">{entry.actualMs}ms</span></span
							>
							<span class="text-green-400">200 OK</span>
						{:else if entry.status === 'timeout'}
							<span class="text-red-400">{entry.error}</span>
						{:else}
							<span class="text-yellow-400">{entry.error}</span>
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
</div>
