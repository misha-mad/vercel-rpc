<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Branded Newtypes — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Branded Newtypes">
		Single-field tuple structs (newtypes) become type aliases by default. Enable <Code
			>branded_newtypes</Code
		>
		for nominal type safety — preventing accidental mixing of structurally identical types like
		<Code>UserId</Code>
		and <Code>OrderId</Code>.
	</PageHeader>

	<CodeBlock html={data.highlighted['newtypeRust']} />

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['newtypeDefaultTs']} />
		<CodeBlock html={data.highlighted['newtypeBrandedTs']} />
	</div>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>
</div>
