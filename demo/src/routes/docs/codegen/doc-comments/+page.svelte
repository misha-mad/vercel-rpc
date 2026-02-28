<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Doc Comments — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Doc Comments">
		Forward Rust
		<Code>///</Code> doc comments as JSDoc in the generated TypeScript — giving you editor tooltips and
		inline documentation on the TypeScript side.
	</PageHeader>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<SectionHeading>Example</SectionHeading>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
	</div>

	<p class="text-text-muted leading-relaxed text-sm">
		Doc comments are preserved on procedures, structs, struct fields, enums, and enum variants.
		Disabled by default (<Code>preserve_docs = false</Code>).
	</p>
</div>
