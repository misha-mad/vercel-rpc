<script lang="ts">
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
</script>

<svelte:head>
	<title>Streaming — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Streaming">
		<Code>#[rpc_stream]</Code> is a new procedure type alongside queries and mutations that enables HTTP
		streaming responses. It's built on top of Axum's streaming primitives and Vercel's streaming support.
	</PageHeader>

	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<span
			class="inline-block rounded-full bg-accent-rust/20 text-accent-rust text-xs font-medium px-3 py-1 mb-4"
			>Coming Soon</span
		>

		<h3 class="text-lg font-semibold mb-3">How it works</h3>
		<p class="text-text-muted text-sm mb-4">
			The handler receives typed input (deserialized as usual) plus a <Code>StreamSender</Code>
			for emitting chunks. The generated TypeScript client gets a
			<Code>stream()</Code>
			method returning an
			<Code>AsyncIterable</Code>.
		</p>

		<h3 class="text-lg font-semibold mb-3">Supported formats</h3>
		<ul class="list-disc list-inside space-y-1 text-text-muted text-sm">
			<li>
				<strong class="text-text-primary">Raw chunked</strong> (<Code>text/plain</Code>) — general
				purpose
			</li>
			<li>
				<strong class="text-text-primary">SSE</strong> (<Code>text/event-stream</Code>) — real-time
				events, LLM token streaming
			</li>
			<li>
				<strong class="text-text-primary">JSON Lines</strong> (<Code>application/x-ndjson</Code>) —
				structured streaming data
			</li>
		</ul>
	</div>
</div>
