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

<svelte:head>
	<title>Serialization — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Serialization</h1>
	<p class="text-text-muted leading-relaxed">
		Override <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serialize</code> and
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">deserialize</code> to use a custom
		serialization format instead of plain JSON.
	</p>

	<h2 class="text-xl font-semibold">Default</h2>
	<p class="text-text-muted text-sm mb-2">
		When not specified, the client uses <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.stringify</code
		>
		and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.parse</code>.
	</p>
	<CodeBlock html={data.highlighted['defaultSerialization']} />

	<h2 class="text-xl font-semibold">Custom (superjson)</h2>
	<p class="text-text-muted text-sm mb-2">
		Use a library like <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>superjson</code
		>
		to support <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Date</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">BigInt</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Map</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Set</code> and other types not natively
		supported by JSON.
	</p>
	<CodeBlock html={data.highlighted['customSerialization']} />

	<h2 class="text-xl font-semibold">Type Signature</h2>
	<CodeBlock html={data.highlighted['signature']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<p class="text-text-muted text-sm">
		Same endpoint, two clients. The default uses <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.parse</code
		>, the custom one uses
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">lossless-json</code>
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
								<td
									class="px-3 py-2"
									class:text-red-400={defaultLost}
									class:text-green-400={!defaultLost}>{row.as_number}</td
								>
								<td class="px-3 py-2 text-text-faint">{typeof row.as_number}</td>
								<td
									class="px-3 py-2"
									class:text-red-400={losslessLost}
									class:text-green-400={!losslessLost}>{String(losslessVal)}</td
								>
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
</div>
