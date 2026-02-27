<script lang="ts">
	import { rpc } from '$lib/client';
	import { RpcError } from '$lib/rpc-client';
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
		const headers: Record<string, string> = withToken ? { Authorization: 'Bearer secret-token-123' } : {};
		usedHeaders = headers;
		try {
			result = await rpc.query('secret', { headers });
		} catch (e) {
			error = e instanceof RpcError ? `${e.status}: ${e.message}` : String(e);
		} finally {
			loading = false;
		}
	}

	let openCode = $state(false);
</script>

<svelte:head>
	<title>Headers — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Headers</h1>
	<p class="text-text-muted leading-relaxed">
		Set default headers for all requests, or provide an async function that resolves headers before
		each call. Per-call headers can override client-level defaults.
	</p>

	<h2 class="text-xl font-semibold">Static Headers</h2>
	<CodeBlock html={data.highlighted['staticHeaders']} />

	<h2 class="text-xl font-semibold">Async Headers</h2>
	<p class="text-text-muted text-sm mb-2">
		Pass an async function to resolve headers dynamically — useful for refreshing auth tokens.
	</p>
	<CodeBlock html={data.highlighted['asyncHeaders']} />

	<h2 class="text-xl font-semibold">Per-Call Headers</h2>
	<p class="text-text-muted text-sm mb-2">
		Override headers for a single call via <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">CallOptions</code
		>.
	</p>
	<CodeBlock html={data.highlighted['perCallHeaders']} />

	<h2 class="text-xl font-semibold">Merge Order</h2>
	<p class="text-text-muted text-sm mb-2">
		Per-call headers are merged on top of client headers. Same-key per-call headers win.
	</p>
	<CodeBlock html={data.highlighted['mergeOrder']} />

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
</div>
