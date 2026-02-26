<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>init — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">init</h1>
	<p class="text-text-muted leading-relaxed">
		Run a function once at cold start. Can be side-effects only (logger, dotenv) or return shared
		state (DB pool, HTTP client) injected as
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&T</code> parameter.
		Works with both queries and mutations.
	</p>

	<h2 class="text-xl font-semibold">Side-Effects Only</h2>
	<p class="text-text-muted text-sm mb-2">
		When the init function returns nothing, it runs once for setup (logging, env vars, tracing).
	</p>
	<CodeBlock html={data.highlighted['sideEffect']} />

	<h2 class="text-xl font-semibold">Shared State</h2>
	<p class="text-text-muted text-sm mb-2">
		When the init function returns a value, it's stored and injected as
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&T</code> into the handler.
		The init function runs once per cold start — the result is reused across requests.
	</p>
	<CodeBlock html={data.highlighted['sharedState']} />

	<h2 class="text-xl font-semibold">HTTP Client</h2>
	<CodeBlock html={data.highlighted['httpClient']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />
</div>
