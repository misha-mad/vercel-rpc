<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>SolidJS — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="SolidJS">
		Fine-grained reactive primitives using <Code>createSignal</Code>, <Code>createEffect</Code>,
		<Code>createMemo</Code>. Input is a getter <Code>() => value</Code>. Returns signal accessors —
		call them to read:
		<Code>data()</Code>.
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
