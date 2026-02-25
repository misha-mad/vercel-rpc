<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>BigInt Mapping — vercel-rpc</title>
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
</div>
