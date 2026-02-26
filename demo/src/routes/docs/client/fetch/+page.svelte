<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Custom Fetch — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Custom Fetch</h1>
	<p class="text-text-muted leading-relaxed">
		Override the <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code> function
		used by the client. Useful for SSR (SvelteKit's platform fetch) and testing (mock fetch).
	</p>

	<h2 class="text-xl font-semibold">Default</h2>
	<p class="text-text-muted text-sm mb-2">
		When not specified, the client uses the global <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code>.
	</p>
	<CodeBlock html={data.highlighted['defaultFetch']} />

	<h2 class="text-xl font-semibold">SSR (SvelteKit)</h2>
	<p class="text-text-muted text-sm mb-2">
		In SvelteKit server loads, pass the platform <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code>
		to forward cookies and resolve relative URLs correctly.
	</p>
	<CodeBlock html={data.highlighted['ssrFetch']} />

	<h2 class="text-xl font-semibold">Testing</h2>
	<p class="text-text-muted text-sm mb-2">
		Inject a mock fetch for unit tests without hitting the network.
	</p>
	<CodeBlock html={data.highlighted['testFetch']} />
</div>
