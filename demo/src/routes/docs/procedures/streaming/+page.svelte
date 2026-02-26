<svelte:head>
	<title>Streaming — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Streaming</h1>
	<p class="text-text-muted leading-relaxed">
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_stream]</code> is a new procedure
		type alongside queries and mutations that enables HTTP streaming responses. It's built on top of Axum's
		streaming primitives and Vercel's streaming support.
	</p>

	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<span
			class="inline-block rounded-full bg-accent-rust/20 text-accent-rust text-xs font-medium px-3 py-1 mb-4"
			>Coming Soon</span
		>

		<h3 class="text-lg font-semibold mb-3">How it works</h3>
		<p class="text-text-muted text-sm mb-4">
			The handler receives typed input (deserialized as usual) plus a <code
				class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">StreamSender</code
			>
			for emitting chunks. The generated TypeScript client gets a
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">stream()</code>
			method returning an
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">AsyncIterable</code>.
		</p>

		<h3 class="text-lg font-semibold mb-3">Supported formats</h3>
		<ul class="list-disc list-inside space-y-1 text-text-muted text-sm">
			<li>
				<strong class="text-text-primary">Raw chunked</strong> (<code
					class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">text/plain</code
				>) — general purpose
			</li>
			<li>
				<strong class="text-text-primary">SSE</strong> (<code
					class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">text/event-stream</code
				>) — real-time events, LLM token streaming
			</li>
			<li>
				<strong class="text-text-primary">JSON Lines</strong> (<code
					class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">application/x-ndjson</code
				>) — structured streaming data
			</li>
		</ul>
	</div>
</div>
