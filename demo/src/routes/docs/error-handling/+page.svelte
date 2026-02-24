<script lang="ts">
	import { createRpcClient } from '$lib/rpc-client';
	import { RpcError } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	// --- Secret (protected endpoint with RpcClientConfig.headers) ---
	let secretResult = $state('');
	let secretError = $state('');
	let secretLoading = $state(false);

	async function callSecret(withToken: boolean) {
		secretLoading = true;
		secretResult = '';
		secretError = '';
		try {
			const client = createRpcClient(
				withToken
					? { baseUrl: '/api', headers: { Authorization: 'Bearer secret-token-123' } }
					: { baseUrl: '/api' }
			);
			secretResult = await client.query('secret');
		} catch (e) {
			if (e instanceof RpcError) {
				const data = e.data as { error?: { message?: string } } | undefined;
				secretError = data?.error?.message ?? e.message;
			} else {
				secretError = `${e}`;
			}
		} finally {
			secretLoading = false;
		}
	}

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>Error Handling — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Error Handling</h1>
	<p class="text-text-muted leading-relaxed">
		When a Rust function returns <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Result&lt;T, E&gt;</code>, errors are propagated as <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code> on the client side. You can also return custom HTTP status codes for authorization or validation errors.
	</p>
	<p class="text-text-muted leading-relaxed">
		The <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code> class provides <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.status</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.message</code>, and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.data</code> for structured error handling.
	</p>

	<h2 class="text-2xl font-semibold">RpcError API</h2>
	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm text-left">
			<thead class="bg-bg-code text-text-muted text-xs uppercase">
				<tr>
					<th class="px-4 py-3">Property</th>
					<th class="px-4 py-3">Type</th>
					<th class="px-4 py-3">Description</th>
				</tr>
			</thead>
			<tbody class="text-text-primary">
				<tr class="border-b border-border">
					<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.status</code></td>
					<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code></td>
					<td class="px-4 py-2 text-text-muted">HTTP status code (e.g. 401, 500)</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.message</code></td>
					<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">string</code></td>
					<td class="px-4 py-2 text-text-muted">Human-readable error message</td>
				</tr>
				<tr>
					<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.data</code></td>
					<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">unknown</code></td>
					<td class="px-4 py-2 text-text-muted">Parsed JSON error body from server</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-2xl font-semibold">Secret — Protected Endpoint</h2>
	<p class="text-text-muted text-sm mb-4">
		This endpoint requires an <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Authorization</code> header. Try calling it with and without a token to see error handling in action.
	</p>
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<div class="flex flex-wrap gap-3 mb-3">
			<button
				onclick={() => callSecret(true)}
				disabled={secretLoading}
				class="rounded-md bg-accent-ts px-4 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50">
				Call with token
			</button>
			<button
				onclick={() => callSecret(false)}
				disabled={secretLoading}
				class="rounded-md border border-red-500 px-4 py-1.5 text-sm font-medium text-red-400 transition-opacity hover:opacity-85 disabled:opacity-50">
				Call without token
			</button>
		</div>
		{#if secretLoading}
			<div class="rounded-md bg-bg-code p-3 text-sm text-text-muted">Loading...</div>
		{/if}
		{#if secretResult}
			<div class="rounded-md bg-bg-code p-3 text-sm text-green-400">{secretResult}</div>
		{/if}
		{#if secretError}
			<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{secretError}</div>
		{/if}
		<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('secret')}>
			{openCode['secret'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['secret']}
			<div class="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
				<div>
					<span class="text-xs text-accent-rust mb-1 block">Rust — api/secret.rs</span>
					<CodeBlock html={data.highlighted['secretRust']} />
				</div>
				<div>
					<span class="text-xs text-accent-ts mb-1 block">Error Handling</span>
					<CodeBlock html={data.highlighted['secretTs']} />
				</div>
			</div>
		{/if}
	</div>
</div>
