<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>timeout — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">timeout</h1>
	<p class="text-text-muted leading-relaxed">
		Server-side timeout via
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">tokio::time::timeout</code>.
		Returns HTTP 504 if exceeded. Also forwarded to the TypeScript client as the default timeout
		for that procedure. Works with both queries and mutations.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Duration Shorthand</h2>
	<CodeBlock html={data.highlighted['durations']} />

	<h2 class="text-xl font-semibold">Behavior</h2>
	<p class="text-text-muted text-sm mb-2">
		When the handler exceeds the timeout, the future is cancelled and the server returns 504.
		The TypeScript client receives an
		<a href="/docs/error-handling" class="text-accent-ts hover:underline"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code></a> with status 504.
	</p>
	<CodeBlock html={data.highlighted['behavior']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />
</div>
