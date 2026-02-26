<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Lifecycle Hooks — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Lifecycle Hooks</h1>
	<p class="text-text-muted leading-relaxed">
		Three hooks let you intercept the request lifecycle: before the fetch, after a successful
		response, and on failure.
	</p>

	<h2 class="text-xl font-semibold">Overview</h2>
	<CodeBlock html={data.highlighted['allHooks']} />

	<h2 class="text-2xl font-semibold mt-8">onRequest</h2>
	<p class="text-text-muted text-sm mb-2">
		Fires before the fetch. You can mutate <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ctx.headers</code
		>
		to add or override headers dynamically. Runs again on every retry attempt.
	</p>
	<CodeBlock html={data.highlighted['onRequest']} />
	<CodeBlock html={data.highlighted['requestCtx']} />

	<h2 class="text-2xl font-semibold mt-8">onResponse</h2>
	<p class="text-text-muted text-sm mb-2">
		Fires after a successful response. Use it for logging, metrics, or cache warming.
	</p>
	<CodeBlock html={data.highlighted['onResponse']} />
	<CodeBlock html={data.highlighted['responseCtx']} />

	<h2 class="text-2xl font-semibold mt-8">onError</h2>
	<p class="text-text-muted text-sm mb-2">
		Fires on failure. Check <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>ctx.willRetry</code
		>
		to know if the client will retry, or use
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ctx.attempt</code>
		to track retry progress.
	</p>
	<CodeBlock html={data.highlighted['onError']} />
	<CodeBlock html={data.highlighted['errorCtx']} />
</div>
