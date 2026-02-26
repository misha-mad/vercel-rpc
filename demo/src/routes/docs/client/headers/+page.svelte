<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
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
</div>
