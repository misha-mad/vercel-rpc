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

	function hasLoss(val: number | bigint, exact: string): boolean {
		return String(val) !== exact;
	}

	let openCode = $state(false);
</script>

<svelte:head>
	<title>BigInt Mapping — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">BigInt Mapping</h1>
	<p class="text-text-muted leading-relaxed">
		By default all integer types map to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code>. Use
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint_types</code> to map
		large integer types to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint</code> instead.
	</p>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<h2 class="text-xl font-semibold">Before / After</h2>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['defaultTs']} />
		<CodeBlock html={data.highlighted['bigintTs']} />
	</div>

	<h2 class="text-xl font-semibold">Custom deserializer</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		The generated client accepts a
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">deserialize</code> option. Plug
		in a BigInt-aware JSON parser so large numbers arrive as native
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">BigInt</code> at runtime:
	</p>
	<CodeBlock html={data.highlighted['losslessClient']} />

	<h2 class="text-xl font-semibold">Why BigInt?</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		JavaScript <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code> is a
		64-bit float (IEEE 754), which can only safely represent integers up to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">2<sup>53</sup> − 1</code>
		(<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Number.MAX_SAFE_INTEGER</code>
		= 9,007,199,254,740,991). Rust's
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i64</code>/<code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u64</code
		>
		can exceed this range, causing silent precision loss. Using
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint</code> avoids this entirely.
	</p>

	<p class="text-text-muted leading-relaxed text-sm">
		Commonly mapped types:
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i64</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u64</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i128</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u128</code>. Smaller types like
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i32</code>
		and
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u32</code> are safe as
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code> and rarely need mapping.
	</p>

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<p class="text-text-muted text-sm">
		Both clients call the same endpoint. The default client uses
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.parse</code> which
		silently loses precision on large integers. The lossless client plugs
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">lossless-json</code> into the
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">deserialize</code> option —
		large numbers arrive as native
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">BigInt</code>.
	</p>

	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-3">JSON.parse vs lossless-json</h3>
		<div class="flex items-center gap-3 mb-4">
			<button
				onclick={fetchDemo}
				disabled={loading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Fetch u64 values</button
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
							<th class="px-1.5 py-2 text-left">Label</th>
							<th class="px-1.5 py-2 text-left">Server (exact)</th>
							<th class="px-1.5 py-2 text-left">
								<code>JSON.parse</code>
							</th>
							<th class="px-1.5 py-2 text-left"></th>
							<th class="px-1.5 py-2 text-left">
								<code>lossless-json</code>
							</th>
							<th class="px-1.5 py-2 text-left"></th>
						</tr>
					</thead>
					<tbody>
						{#each defaultResult.values as row, i (i)}
							{@const lost = hasLoss(row.as_number, row.exact)}
							{@const losslessVal = losslessResult[i]?.as_number}
							{@const losslessStr = String(losslessVal)}
							{@const losslessLost = losslessStr !== row.exact}
							<tr class="border-t border-border/50">
								<td class="px-1.5 py-2 text-text-muted">{row.label}</td>
								<td class="px-1.5 py-2 text-accent-rust">{row.exact}</td>
								<td class="px-1.5 py-2 text-accent-ts">{row.as_number}</td>
								<td class="px-1.5 py-2"
									>{#if lost}<span class="text-red-400">lost</span>{:else}<span
											class="text-green-400">ok</span
										>{/if}</td
								>
								<td class="px-1.5 py-2 text-accent-ts">{losslessStr}</td>
								<td class="px-1.5 py-2"
									>{#if losslessLost}<span class="text-red-400">lost</span>{:else}<span
											class="text-green-400">ok</span
										>{/if}</td
								>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			<div class="mt-3 space-y-1">
				<p class="text-text-faint text-xs">
					<code class="bg-bg-code px-1 py-0.5 rounded">typeof</code> of
					<code class="bg-bg-code px-1 py-0.5 rounded">as_number</code>: JSON.parse &rarr;
					<code class="bg-bg-code px-1 py-0.5 rounded"
						>{typeof defaultResult.values[0]?.as_number}</code
					>, lossless-json &rarr;
					<code class="bg-bg-code px-1 py-0.5 rounded">{typeof losslessResult[0]?.as_number}</code>
					/
					<code class="bg-bg-code px-1 py-0.5 rounded"
						>{typeof losslessResult[losslessResult.length - 1]?.as_number}</code
					>
				</p>
				<p class="text-text-faint text-xs">
					Plug any BigInt-aware parser into the client's
					<code class="bg-bg-code px-1 py-0.5 rounded">deserialize</code> option and
					<code class="bg-bg-code px-1 py-0.5 rounded">bigint_types</code> fields carry exact values at
					runtime.
				</p>
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
