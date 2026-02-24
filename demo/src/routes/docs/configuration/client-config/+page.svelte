<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>RpcClientConfig — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">RpcClientConfig</h1>
	<p class="text-text-muted leading-relaxed">
		The generated client accepts a config object with options for headers, retry, timeout, lifecycle hooks, and more.
	</p>

	<h2 class="text-2xl font-semibold">Basic</h2>
	<CodeBlock html={data.highlighted['basicConfig']} large />

	<h2 class="text-2xl font-semibold">Full options</h2>
	<CodeBlock html={data.highlighted['fullConfig']} large />

	<h2 class="text-2xl font-semibold">Lifecycle hook types</h2>
	<p class="text-text-muted text-sm">
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">onRequest</code> fires before the fetch — you can mutate <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ctx.headers</code>.
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">onResponse</code> fires after a successful response.
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">onError</code> fires on failure — check <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ctx.willRetry</code> to know if the client will retry.
	</p>
	<CodeBlock html={data.highlighted['hooksTypes']} large />
</div>
