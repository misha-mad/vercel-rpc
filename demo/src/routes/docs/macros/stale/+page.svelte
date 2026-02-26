<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>stale — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">stale</h1>
	<p class="text-text-muted leading-relaxed">
		Adds <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>stale-while-revalidate</code
		>
		to the <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cache-Control</code>
		header. The CDN serves stale content while fetching a fresh response in the background. Requires
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">cache</code> — compile error without
		it. Queries only.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">With Private Cache</h2>
	<CodeBlock html={data.highlighted['private']} />

	<h2 class="text-xl font-semibold">Generated Headers</h2>
	<CodeBlock html={data.highlighted['headers']} />

	<h2 class="text-xl font-semibold">Requires cache</h2>
	<p class="text-text-muted text-sm mb-2">
		Using <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">stale</code> without
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">cache</code> is a compile error.
	</p>
	<CodeBlock html={data.highlighted['error']} />
</div>
