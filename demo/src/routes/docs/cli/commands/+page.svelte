<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>CLI — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">CLI</h1>
	<p class="text-text-muted leading-relaxed">
		The <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">metaxy</code> CLI scans
		Rust source files, parses types via
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code>, and generates
		TypeScript types and a typed client.
	</p>

	<h2 class="text-xl font-semibold">Install</h2>
	<CodeBlock html={data.highlighted['install']} />

	<h2 class="text-xl font-semibold">Commands</h2>
	<CodeBlock html={data.highlighted['usage']} />

	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm">
			<thead>
				<tr class="text-left text-text-faint border-b border-border bg-bg-code">
					<th class="px-4 py-2 font-medium">Command</th>
					<th class="px-4 py-2 font-medium">Description</th>
				</tr>
			</thead>
			<tbody class="text-text-muted">
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5"
						><a href="/docs/cli/generate" class="text-accent-ts hover:underline font-mono text-xs"
							>generate</a
						></td
					>
					<td class="px-4 py-1.5">Generate TypeScript types and client (one-shot)</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5"
						><a href="/docs/cli/scan" class="text-accent-ts hover:underline font-mono text-xs"
							>scan</a
						></td
					>
					<td class="px-4 py-1.5">Print discovered procedures and types (debugging)</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5"
						><a href="/docs/cli/watch" class="text-accent-ts hover:underline font-mono text-xs"
							>watch</a
						></td
					>
					<td class="px-4 py-1.5">Watch for changes and regenerate automatically</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-xl font-semibold">Global Flags</h2>
	<p class="text-text-muted text-sm mb-2">Available on all commands.</p>
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
					<td class="px-4 py-1.5 font-mono text-xs"><code>--config</code></td>
					<td class="px-4 py-1.5">auto</td>
					<td class="px-4 py-1.5">Explicit config file path</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs"><code>--no-config</code></td>
					<td class="px-4 py-1.5">false</td>
					<td class="px-4 py-1.5">Disable config file loading</td>
				</tr>
			</tbody>
		</table>
	</div>
	<CodeBlock html={data.highlighted['configFlags']} />
</div>
