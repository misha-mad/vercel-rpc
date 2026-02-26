<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>idempotent — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">idempotent</h1>
	<p class="text-text-muted leading-relaxed">
		Mutations only. Marks the mutation as safe to retry on network errors. By default, mutations are <strong
			class="text-text-primary">never</strong
		> retried — even with a retry policy configured. Queries (GET) are always retryable.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Good Candidates</h2>
	<p class="text-text-muted text-sm mb-2">
		Mark mutations as idempotent when repeated calls produce the same result: upserts, overwrites,
		deletes.
	</p>
	<CodeBlock html={data.highlighted['examples']} />

	<h2 class="text-xl font-semibold">Retry Behavior</h2>
	<p class="text-text-muted text-sm mb-2">
		When marked idempotent, the mutation follows the same <a
			href="/docs/client/retry"
			class="text-accent-ts hover:underline">retry policy</a
		> as queries. Without it, the mutation is never retried regardless of the retry configuration.
	</p>
	<CodeBlock html={data.highlighted['retryBehavior']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />
</div>
