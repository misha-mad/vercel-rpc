<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Per-Call Options — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Per-Call Options</h1>
	<p class="text-text-muted leading-relaxed">
		Every <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">query()</code> and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">mutate()</code> call accepts an optional trailing <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">CallOptions</code> object to override client-level defaults on a per-request basis.
	</p>

	<h2 class="text-2xl font-semibold">CallOptions</h2>
	<CodeBlock html={data.highlighted['callOptionsType']} large />

	<h2 class="text-2xl font-semibold">Usage</h2>
	<CodeBlock html={data.highlighted['usage']} large />

	<h2 class="text-2xl font-semibold">Timeout resolution</h2>
	<p class="text-text-muted text-sm">
		Per-call timeout takes highest priority, followed by the macro-level <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">timeout</code> attribute, then client config.
	</p>
	<CodeBlock html={data.highlighted['resolution']} large />
</div>
