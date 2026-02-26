<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>metaxy scan — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">metaxy scan</h1>
	<p class="text-text-muted leading-relaxed">
		Parse Rust files and print discovered procedures, structs, and enums. Useful for debugging
		and verifying what the CLI sees before generating code.
	</p>

	<h2 class="text-xl font-semibold">Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Flags</h2>
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
					<td class="px-4 py-1.5 font-mono text-xs"><code>-d, --dir</code></td>
					<td class="px-4 py-1.5">api</td>
					<td class="px-4 py-1.5">Rust source directory</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--include</code></td>
					<td class="px-4 py-1.5">**/*.rs</td>
					<td class="px-4 py-1.5">Glob patterns to include (repeatable)</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs"><code>--exclude</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5">Glob patterns to exclude (repeatable)</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-xl font-semibold">Examples</h2>
	<CodeBlock html={data.highlighted['custom']} />

	<h2 class="text-xl font-semibold">Sample Output</h2>
	<CodeBlock html={data.highlighted['output']} />
</div>
