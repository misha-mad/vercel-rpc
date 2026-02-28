<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Field Naming — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Field Naming">
		By default field names are emitted as-is. Set
		<Code>camelCase</Code> to convert
		<Code>snake_case</Code> fields to
		<Code>camelCase</Code> in the generated TypeScript.
	</PageHeader>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<SectionHeading>preserve (default)</SectionHeading>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['preserveTs']} />
	</div>

	<SectionHeading>camelCase</SectionHeading>
	<CodeBlock html={data.highlighted['camelTs']} />

	<p class="text-text-muted leading-relaxed text-sm">
		The transform also applies to struct variant fields in enums. Enum variant names and procedure
		names are not affected.
	</p>

	<p class="text-text-muted leading-relaxed text-sm">
		Priority: field-level
		<a href="/docs/codegen/serde" class="text-accent-ts hover:underline"
			><Code>serde(rename)</Code></a
		>
		&gt; container
		<a href="/docs/codegen/serde" class="text-accent-ts hover:underline"><Code>rename_all</Code></a>
		&gt;
		<Code>codegen.naming.fields</Code>
		&gt; original name.
	</p>
</div>
