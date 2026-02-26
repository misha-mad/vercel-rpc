<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Timeout &amp; Abort — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Timeout & Abort</h1>
	<p class="text-text-muted leading-relaxed">
		Set a global timeout for all requests, pass an <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">AbortSignal</code
		>
		for manual cancellation, or combine both. Timeouts and aborts throw a
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">DOMException</code> with
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">name: 'AbortError'</code>, not
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code>.
	</p>

	<h2 class="text-xl font-semibold">Client Timeout</h2>
	<CodeBlock html={data.highlighted['clientTimeout']} />

	<h2 class="text-xl font-semibold">Per-Call Timeout</h2>
	<CodeBlock html={data.highlighted['perCallTimeout']} />

	<h2 class="text-xl font-semibold">AbortSignal</h2>
	<p class="text-text-muted text-sm mb-2">
		A client-level signal aborts all in-flight requests when fired.
	</p>
	<CodeBlock html={data.highlighted['abortSignal']} />

	<h2 class="text-xl font-semibold">Per-Call Signal</h2>
	<CodeBlock html={data.highlighted['perCallSignal']} />

	<h2 class="text-xl font-semibold">Combined Signals</h2>
	<p class="text-text-muted text-sm mb-2">
		When both client and per-call signals are provided, they are combined — the request aborts when
		either signal fires.
	</p>
	<CodeBlock html={data.highlighted['combinedSignals']} />

	<h2 class="text-xl font-semibold">Error Handling</h2>
	<p class="text-text-muted text-sm mb-2">
		Timeouts and manual aborts throw a <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">DOMException</code
		>, not an <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code>.
	</p>
	<CodeBlock html={data.highlighted['errorHandling']} />
</div>
