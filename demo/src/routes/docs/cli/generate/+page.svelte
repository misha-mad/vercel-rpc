<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>metaxy generate — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="metaxy generate" mono>
		Generate TypeScript types and a typed client from Rust source files. One-shot execution.
	</PageHeader>

	<SectionHeading>Usage</SectionHeading>
	<CodeBlock html={data.highlighted['basic']} />

	<!-- Input flags -->
	<SectionHeading>Input Flags</SectionHeading>
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
	<CodeBlock html={data.highlighted['inputExamples']} />

	<!-- Output flags -->
	<SectionHeading>Output Flags</SectionHeading>
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
					<td class="px-4 py-1.5 font-mono text-xs"><code>-o, --output</code></td>
					<td class="px-4 py-1.5">src/lib/rpc-types.ts</td>
					<td class="px-4 py-1.5">Types file path</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>-c, --client-output</code></td>
					<td class="px-4 py-1.5">src/lib/rpc-client.ts</td>
					<td class="px-4 py-1.5">Client file path</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--svelte-output</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5">Svelte 5 reactive wrapper (opt-in)</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--react-output</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5">React hooks (opt-in)</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--vue-output</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5">Vue 3 composables (opt-in)</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--solid-output</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5">SolidJS primitives (opt-in)</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--types-import</code></td>
					<td class="px-4 py-1.5">./rpc-types</td>
					<td class="px-4 py-1.5">Import path for types in client</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs"><code>--extension</code></td>
					<td class="px-4 py-1.5">""</td>
					<td class="px-4 py-1.5"
						>Import extension, e.g. <code class="text-xs font-mono">".js"</code> for ESM</td
					>
				</tr>
			</tbody>
		</table>
	</div>
	<CodeBlock html={data.highlighted['outputExamples']} />

	<!-- Codegen flags -->
	<SectionHeading>Codegen Flags</SectionHeading>
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
					<td class="px-4 py-1.5 font-mono text-xs"><code>--preserve-docs</code></td>
					<td class="px-4 py-1.5">false</td>
					<td class="px-4 py-1.5"
						>Forward Rust <code class="text-xs font-mono">///</code> as JSDoc</td
					>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--branded-newtypes</code></td>
					<td class="px-4 py-1.5">false</td>
					<td class="px-4 py-1.5">Emit branded types for newtypes</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--fields</code></td>
					<td class="px-4 py-1.5">preserve</td>
					<td class="px-4 py-1.5"
						><code class="text-xs font-mono">preserve</code> or
						<code class="text-xs font-mono">camelCase</code></td
					>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs"><code>--type-override</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5">Map Rust type to TS (repeatable)</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs"><code>--bigint-type</code></td>
					<td class="px-4 py-1.5">—</td>
					<td class="px-4 py-1.5"
						>Map integer type to <code class="text-xs font-mono">bigint</code> (repeatable)</td
					>
				</tr>
			</tbody>
		</table>
	</div>
	<CodeBlock html={data.highlighted['codegenExamples']} />
</div>
