<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>React — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="React">
		Hook-based wrappers using <Code>useState</Code>
		/ <Code>useEffect</Code>. Input is a
		plain value (not a getter) — dependencies tracked via
		<Code>JSON.stringify</Code>.
	</PageHeader>

	<SectionHeading level="large">Setup</SectionHeading>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['setupToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['setupCli']} />
	</div>

	<SectionHeading level="large">useQuery</SectionHeading>
	<CodeBlock html={data.highlighted['query']} large />

	<SectionHeading level="large">useMutation</SectionHeading>
	<CodeBlock html={data.highlighted['mutation']} large />
</div>
