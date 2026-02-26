<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>cache — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">cache</h1>
	<p class="text-text-muted leading-relaxed">
		Add <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cache-Control</code> headers
		to successful responses. Queries only — mutations cannot be cached (compile error).
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Duration Shorthand</h2>
	<CodeBlock html={data.highlighted['durations']} />
	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm">
			<thead>
				<tr class="text-left text-text-faint border-b border-border bg-bg-code">
					<th class="px-4 py-2 font-medium">Suffix</th>
					<th class="px-4 py-2 font-medium">Unit</th>
					<th class="px-4 py-2 font-medium">Example</th>
				</tr>
			</thead>
			<tbody class="text-text-muted">
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs">s</td>
					<td class="px-4 py-1.5">seconds</td>
					<td class="px-4 py-1.5 font-mono text-xs">30s → max-age=30</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs">m</td>
					<td class="px-4 py-1.5">minutes</td>
					<td class="px-4 py-1.5 font-mono text-xs">5m → max-age=300</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs">h</td>
					<td class="px-4 py-1.5">hours</td>
					<td class="px-4 py-1.5 font-mono text-xs">1h → max-age=3600</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs">d</td>
					<td class="px-4 py-1.5">days</td>
					<td class="px-4 py-1.5 font-mono text-xs">1d → max-age=86400</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-xl font-semibold">Public vs Private</h2>
	<p class="text-text-muted text-sm mb-2">
		By default caches are public (shared on CDN). Prefix with
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">private,</code> for user-specific
		data that should only be cached in the browser.
	</p>
	<CodeBlock html={data.highlighted['private']} />

	<h2 class="text-xl font-semibold">With stale-while-revalidate</h2>
	<p class="text-text-muted text-sm mb-2">
		Combine with <a href="/docs/macros/stale" class="text-accent-ts hover:underline">stale</a>
		to serve stale content while fetching fresh data in the background.
	</p>
	<CodeBlock html={data.highlighted['withStale']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />
</div>
