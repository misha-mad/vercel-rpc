<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Generics — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Generics">
		Generic structs and enums produce generic TypeScript interfaces. Type parameters are preserved
		in procedure signatures, so <Code>Paginated&lt;User&gt;</Code>
		in Rust becomes
		<Code>Paginated&lt;User&gt;</Code> in TypeScript.
	</PageHeader>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['genericsRust']} />
		<CodeBlock html={data.highlighted['genericsTs']} />
	</div>

	<SectionHeading>Multiple Type Parameters</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Structs with multiple type parameters are supported. All parameters are preserved in the
		generated TypeScript.
	</p>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['multiRust']} />
		<CodeBlock html={data.highlighted['multiTs']} />
	</div>
</div>
