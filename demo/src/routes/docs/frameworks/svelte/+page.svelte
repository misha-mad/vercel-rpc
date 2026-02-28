<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Svelte 5 — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Svelte 5">
		Rune-based reactive wrappers using <Code>$state</Code>
		and <Code>$effect</Code>. Input is a
		getter <Code>() => Input</Code> for
		reactive dependency tracking. Generated as
		<Code>.svelte.ts</Code> (required for rune
		syntax).
	</PageHeader>

	<SectionHeading level="large">Setup</SectionHeading>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['setupToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['setupCli']} />
	</div>

	<SectionHeading level="large">createQuery</SectionHeading>
	<CodeBlock html={data.highlighted['query']} large />

	<SectionHeading level="large">createMutation</SectionHeading>
	<CodeBlock html={data.highlighted['mutation']} large />
</div>
