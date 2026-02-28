<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>metaxy watch — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="metaxy watch" mono>
		Same as <Code>generate</Code> but
		watches for <Code>.rs</Code> file
		changes and regenerates automatically. Accepts all
		<Code>generate</Code> flags plus the watch-specific
		flags below.
	</PageHeader>

	<SectionHeading>Usage</SectionHeading>
	<CodeBlock html={data.highlighted['basic']} />

	<SectionHeading>Watch Flags</SectionHeading>
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

	<SectionHeading>Examples</SectionHeading>
	<CodeBlock html={data.highlighted['examples']} />

	<p class="text-text-muted text-sm">
		All <Code>generate</Code> flags
		(input, output, codegen) work with
		<Code>watch</Code>
		as well. See
		<a href="/docs/cli/generate" class="text-accent-ts hover:underline">metaxy generate</a>
		for the full reference.
	</p>
</div>
