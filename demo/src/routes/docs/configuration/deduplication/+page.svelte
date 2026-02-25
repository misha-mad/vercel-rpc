<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Request Deduplication — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Request Deduplication</h1>
	<p class="text-text-muted leading-relaxed">
		Identical in-flight queries are automatically deduplicated — only one HTTP request is made and
		all callers share the same promise.
	</p>

	<h2 class="text-xl font-semibold">How It Works</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		When multiple callers issue the same query with the same input concurrently, only one HTTP
		request is made. Requests are matched by procedure name + serialized input. Subsequent callers
		receive the same in-flight promise.
	</p>
	<CodeBlock html={data.highlighted['dedupExample']} />

	<h2 class="text-xl font-semibold">Disabling Deduplication</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		Dedup is controlled at two levels — client config and per-call. Per-call takes precedence.
	</p>
	<CodeBlock html={data.highlighted['dedupDisableGlobal']} />
	<CodeBlock html={data.highlighted['dedupDisablePerCall']} />

	<p class="text-text-muted leading-relaxed text-sm">
		Mutations are never deduplicated. Each per-caller
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">AbortSignal</code> is wrapped independently
		— aborting one caller does not affect others sharing the same in-flight promise.
	</p>
</div>
