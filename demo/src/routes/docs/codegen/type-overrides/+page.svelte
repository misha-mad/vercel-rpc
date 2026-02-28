<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Type Overrides — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Type Overrides">
		Map external crate types (or any Rust type) to custom TypeScript types via
		<Code>[codegen.type_overrides]</Code>.
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

	<SectionHeading>Matching Rules</SectionHeading>
	<p class="text-text-muted leading-relaxed text-sm">
		Override keys are matched against type names by their last path segment. For example, key
		<Code>"chrono::DateTime"</Code>
		matches both the fully-qualified
		<Code>chrono::DateTime&lt;Utc&gt;</Code>
		and the imported
		<Code>DateTime&lt;Utc&gt;</Code>. If you use fully-qualified paths in your Rust source, exact
		full-path matching takes priority.
	</p>
	<p class="text-text-muted leading-relaxed text-sm">
		Overrides are applied before code generation — every occurrence of the matched type (including
		inside
		<Code>Vec&lt;T&gt;</Code>,
		<Code>Option&lt;T&gt;</Code>, etc.) is replaced with the specified TypeScript type, and generic
		parameters are stripped.
	</p>
</div>
