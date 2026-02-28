<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Config — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Config">
		The generated client accepts a config object with options for headers, retry, timeout, lifecycle
		hooks, and more.
	</PageHeader>

	<SectionHeading level="large">Basic</SectionHeading>
	<CodeBlock html={data.highlighted['basicConfig']} large />

	<SectionHeading level="large">Full options</SectionHeading>
	<CodeBlock html={data.highlighted['fullConfig']} large />

	<SectionHeading level="large">Lifecycle hook types</SectionHeading>
	<p class="text-text-muted text-sm">
		<Code>onRequest</Code> fires before
		the fetch — you can mutate
		<Code>ctx.headers</Code>.
		<Code>onResponse</Code> fires after a
		successful response.
		<Code>onError</Code> fires on failure
		— check <Code>ctx.willRetry</Code> to know
		if the client will retry.
	</p>
	<CodeBlock html={data.highlighted['hooksTypes']} large />
</div>
