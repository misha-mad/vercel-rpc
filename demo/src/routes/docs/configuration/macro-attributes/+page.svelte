<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Macro Attributes — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-10">
	<h1 class="text-3xl font-bold">Macro Attributes</h1>
	<p class="text-text-muted leading-relaxed">
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> and
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code> accept optional
		attributes that control caching, initialization, timeouts, and retry behavior.
	</p>

	<!-- cache -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">cache</h2>
		<p class="text-text-muted text-sm">
			Add <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cache-Control</code>
			headers to successful responses. Duration shorthand:
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">30s</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">5m</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">1h</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">1d</code>. Queries only —
			mutations cannot be cached.
		</p>
		<CodeBlock html={data.highlighted['cacheRust']} />
	</section>

	<!-- init -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">init</h2>
		<p class="text-text-muted text-sm">
			Run a function once at cold start. Can be side-effects only (logger, dotenv) or return shared
			state (DB pool, HTTP client) injected as <code
				class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&amp;T</code
			> parameter.
		</p>
		<CodeBlock html={data.highlighted['initRust']} />
	</section>

	<!-- timeout -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">timeout</h2>
		<p class="text-text-muted text-sm">
			Server-side timeout via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
				>tokio::time::timeout</code
			>. Returns 504 if exceeded. Also forwarded to the TypeScript client as the default timeout for
			that procedure.
		</p>
		<CodeBlock html={data.highlighted['timeoutRust']} />
	</section>

	<!-- idempotent -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">idempotent</h2>
		<p class="text-text-muted text-sm">
			Mutations only. Marks the mutation as safe to retry on network errors. By default, mutations
			are <strong class="text-text-primary">never</strong> retried. Queries (GET) are always retryable.
		</p>
		<CodeBlock html={data.highlighted['idempotentRust']} />
	</section>

	<!-- Summary table -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">Availability</h2>
		<div class="overflow-x-auto rounded-lg border border-border">
			<table class="w-full text-sm text-left">
				<thead class="bg-bg-code text-text-muted text-xs uppercase">
					<tr>
						<th class="px-4 py-3">Attribute</th>
						<th class="px-4 py-3">#[rpc_query]</th>
						<th class="px-4 py-3">#[rpc_mutation]</th>
					</tr>
				</thead>
				<tbody class="text-text-primary">
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs">cache</td>
						<td class="px-4 py-2 text-accent-ts">yes</td>
						<td class="px-4 py-2 text-red-400">no (compile error)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs">init</td>
						<td class="px-4 py-2 text-accent-ts">yes</td>
						<td class="px-4 py-2 text-accent-ts">yes</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs">timeout</td>
						<td class="px-4 py-2 text-accent-ts">yes</td>
						<td class="px-4 py-2 text-accent-ts">yes</td>
					</tr>
					<tr>
						<td class="px-4 py-2 font-mono text-xs">idempotent</td>
						<td class="px-4 py-2 text-red-400">no (compile error)</td>
						<td class="px-4 py-2 text-accent-ts">yes</td>
					</tr>
				</tbody>
			</table>
		</div>
	</section>
</div>
