<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Doc Comments — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Doc Comments</h1>
	<p class="text-text-muted leading-relaxed">
		Forward Rust
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">///</code> doc comments as JSDoc
		in the generated TypeScript — giving you editor tooltips and inline documentation on the TypeScript
		side.
	</p>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<h2 class="text-xl font-semibold">Example</h2>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
	</div>

	<p class="text-text-muted leading-relaxed text-sm">
		Doc comments are preserved on procedures, structs, and enums. Disabled by default (<code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">preserve_docs = false</code
		>).
	</p>
</div>
