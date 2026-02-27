<script lang="ts">
	import { rpc } from '$lib/client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let result: import('$lib/rpc-types').BigIntDemoResponse | undefined = $state();
	let loading = $state(false);

	async function fetchDemo() {
		loading = true;
		try {
			result = await rpc.query('bigint_demo');
		} finally {
			loading = false;
		}
	}

	function hasLoss(num: number, str: string): boolean {
		return String(num) !== str;
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
		This demo does <strong>not</strong> use
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint_types</code> — so
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u64</code> maps to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code>. The server
		returns each value as both a <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u64</code>
		field and a string field. Compare them to see where precision is lost.
	</p>

	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-3">Precision Loss Demo</h3>
		<div class="flex items-center gap-3 mb-4">
			<button
				onclick={fetchDemo}
				disabled={loading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
			>Fetch u64 values</button>
			{#if loading}
				<span class="text-sm text-text-muted">Loading...</span>
			{/if}
		</div>

		{#if result}
			<div class="overflow-x-auto rounded-md border border-border">
				<table class="w-full text-xs font-mono">
					<thead class="bg-bg-code text-text-faint">
						<tr>
							<th class="px-3 py-2 text-left">Label</th>
							<th class="px-3 py-2 text-left">Server (exact)</th>
							<th class="px-3 py-2 text-left">JS number</th>
							<th class="px-3 py-2 text-left"></th>
						</tr>
					</thead>
					<tbody>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">small (42)</td>
							<td class="px-3 py-2 text-accent-rust">{result.small_str}</td>
							<td class="px-3 py-2 text-accent-ts">{result.small}</td>
							<td class="px-3 py-2 text-green-400">ok</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">MAX_SAFE_INTEGER</td>
							<td class="px-3 py-2 text-accent-rust">{result.max_safe_str}</td>
							<td class="px-3 py-2 text-accent-ts">{result.max_safe}</td>
							<td class="px-3 py-2">{#if hasLoss(result.max_safe, result.max_safe_str)}<span class="text-red-400">precision lost!</span>{:else}<span class="text-green-400">ok</span>{/if}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">MAX_SAFE + 2</td>
							<td class="px-3 py-2 text-accent-rust">{result.above_safe_str}</td>
							<td class="px-3 py-2 text-accent-ts">{result.above_safe}</td>
							<td class="px-3 py-2">{#if hasLoss(result.above_safe, result.above_safe_str)}<span class="text-red-400">precision lost!</span>{:else}<span class="text-green-400">ok</span>{/if}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">u64::MAX</td>
							<td class="px-3 py-2 text-accent-rust">{result.u64_max_str}</td>
							<td class="px-3 py-2 text-accent-ts">{result.u64_max}</td>
							<td class="px-3 py-2">{#if hasLoss(result.u64_max, result.u64_max_str)}<span class="text-red-400">precision lost!</span>{:else}<span class="text-green-400">ok</span>{/if}</td>
						</tr>
					</tbody>
				</table>
			</div>
			<p class="text-text-faint text-xs mt-2">
				With <code class="bg-bg-code px-1 py-0.5 rounded">bigint_types = ["u64"]</code> in your config, the "JS number" column would use <code class="bg-bg-code px-1 py-0.5 rounded">bigint</code> and match exactly.
			</p>
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
