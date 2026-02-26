<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Retry — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Retry</h1>
	<p class="text-text-muted leading-relaxed">
		Queries are retried automatically on network errors or retryable HTTP status codes. Mutations
		are <strong>never</strong> retried unless explicitly marked as
		<a href="/docs/macros/idempotent" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">idempotent</code></a
		>.
	</p>

	<h2 class="text-xl font-semibold">RetryPolicy</h2>
	<CodeBlock html={data.highlighted['retryConfig']} />

	<p class="text-text-muted leading-relaxed text-sm">
		A request is retried when a network error occurs or the response status is in
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">retryOn</code>, up to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">attempts</code> additional
		tries. On each retry the full
		<a href="/docs/client/hooks" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">onRequest</code></a
		> hook runs again, so dynamic headers (e.g. refreshed auth tokens) are re-evaluated.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['retryBasic']} />

	<h2 class="text-xl font-semibold">Exponential Backoff</h2>
	<CodeBlock html={data.highlighted['retryExponential']} />

	<h2 class="text-xl font-semibold">Custom Retry Logic</h2>
	<CodeBlock html={data.highlighted['retryCustom']} />

	<h2 class="text-xl font-semibold">Idempotent Mutations</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		By default, mutations are never retried — even with a retry policy configured. To opt a mutation
		into retry, mark it as
		<a href="/docs/macros/idempotent" class="text-accent-ts hover:underline"
			><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">idempotent</code></a
		>
		in the Rust macro. This signals that repeated calls produce the same result.
	</p>
	<CodeBlock html={data.highlighted['retryIdempotent']} />
</div>
