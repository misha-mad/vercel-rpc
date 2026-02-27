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

<svelte:head>
	<title>Timeout &amp; Abort — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Timeout & Abort</h1>
	<p class="text-text-muted leading-relaxed">
		Set a global timeout for all requests, pass an <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">AbortSignal</code
		>
		for manual cancellation, or combine both. Timeouts and aborts throw a
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">DOMException</code> with
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">name: 'AbortError'</code>, not
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code>.
	</p>

	<h2 class="text-xl font-semibold">Client Timeout</h2>
	<CodeBlock html={data.highlighted['clientTimeout']} />

	<h2 class="text-xl font-semibold">Per-Call Timeout</h2>
	<CodeBlock html={data.highlighted['perCallTimeout']} />

	<h2 class="text-xl font-semibold">AbortSignal</h2>
	<p class="text-text-muted text-sm mb-2">
		A client-level signal aborts all in-flight requests when fired.
	</p>
	<CodeBlock html={data.highlighted['abortSignal']} />

	<h2 class="text-xl font-semibold">Per-Call Signal</h2>
	<CodeBlock html={data.highlighted['perCallSignal']} />

	<h2 class="text-xl font-semibold">Combined Signals</h2>
	<p class="text-text-muted text-sm mb-2">
		When both client and per-call signals are provided, they are combined — the request aborts when
		either signal fires.
	</p>
	<CodeBlock html={data.highlighted['combinedSignals']} />

	<h2 class="text-xl font-semibold">Error Handling</h2>
	<p class="text-text-muted text-sm mb-2">
		Timeouts and manual aborts throw a <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">DOMException</code
		>, not an <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code>.
	</p>
	<CodeBlock html={data.highlighted['errorHandling']} />

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
</div>
