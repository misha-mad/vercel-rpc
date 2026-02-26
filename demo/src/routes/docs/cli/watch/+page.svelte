<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>metaxy watch — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">metaxy watch</h1>
	<p class="text-text-muted leading-relaxed">
		Same as <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">generate</code> but watches
		for <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.rs</code> file changes and
		regenerates automatically. Accepts all
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">generate</code> flags plus the
		watch-specific flags below.
	</p>

	<h2 class="text-xl font-semibold">Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Watch Flags</h2>
	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm">
			<thead>
				<tr class="text-left text-text-faint border-b border-border bg-bg-code">
					<th class="px-4 py-2 font-medium">Flag</th>
					<th class="px-4 py-2 font-medium">Default</th>
					<th class="px-4 py-2 font-medium">Description</th>
				</tr>
			</thead>
			<tbody class="text-text-muted">
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--debounce-ms</code></td>
					<td class="px-4 py-1.5">200</td>
					<td class="px-4 py-1.5">Milliseconds to wait after file change before regenerating</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs"><code>--clear-screen</code></td>
					<td class="px-4 py-1.5">false</td>
					<td class="px-4 py-1.5">Clear terminal before each regeneration</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-xl font-semibold">Examples</h2>
	<CodeBlock html={data.highlighted['examples']} />

	<p class="text-text-muted text-sm">
		All <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">generate</code> flags
		(input, output, codegen) work with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">watch</code>
		as well. See <a href="/docs/cli/generate" class="text-accent-ts hover:underline">metaxy generate</a>
		for the full reference.
	</p>
</div>
