<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Serde Support — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-10">
	<PageHeader title="Serde Support">
		metaxy respects serde attributes on your Rust types and translates them into the corresponding
		TypeScript output. This ensures the generated types match exactly what your API serializes.
	</PageHeader>

	<!-- rename_all -->
	<section class="space-y-4">
		<SectionHeading level="large">rename_all</SectionHeading>
		<p class="text-text-muted text-sm">
			Apply a naming convention to all fields or variants. Supported: <Code>camelCase</Code>, <Code
				>snake_case</Code
			>,
			<Code>PascalCase</Code>,
			<Code>SCREAMING_SNAKE_CASE</Code>,
			<Code>kebab-case</Code>,
			<Code>lowercase</Code>,
			<Code>UPPERCASE</Code>.
		</p>
		<div class="space-y-3">
			<CodeBlock html={data.highlighted['renameAllRust']} />
			<CodeBlock html={data.highlighted['renameAllTs']} />
		</div>
	</section>

	<!-- rename -->
	<section class="space-y-4">
		<SectionHeading level="large">rename</SectionHeading>
		<p class="text-text-muted text-sm">
			Override individual field or variant names. Takes priority over <Code>rename_all</Code>.
		</p>
		<div class="space-y-3">
			<CodeBlock html={data.highlighted['renameFieldRust']} />
			<CodeBlock html={data.highlighted['renameFieldTs']} />
		</div>
	</section>

	<!-- flatten -->
	<section class="space-y-4">
		<SectionHeading level="large">flatten</SectionHeading>
		<p class="text-text-muted text-sm">
			Merge a nested struct's fields into the parent. Produces a TypeScript intersection type.
		</p>
		<div class="space-y-3">
			<CodeBlock html={data.highlighted['flattenRust']} />
			<CodeBlock html={data.highlighted['flattenTs']} />
		</div>
	</section>

	<!-- Enum tagging -->
	<section class="space-y-6">
		<SectionHeading level="large">Enum Tagging</SectionHeading>
		<p class="text-text-muted text-sm">
			All four serde enum representations are supported. The default is externally tagged.
		</p>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">External (default)</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumExternalRust']} />
				<CodeBlock html={data.highlighted['enumExternalTs']} />
			</div>
		</div>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">
				Internal <Code>tag = "..."</Code>
			</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumInternalRust']} />
				<CodeBlock html={data.highlighted['enumInternalTs']} />
			</div>
		</div>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">
				Adjacent <Code>tag + content</Code>
			</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumAdjacentRust']} />
				<CodeBlock html={data.highlighted['enumAdjacentTs']} />
			</div>
		</div>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">Untagged</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumUntaggedRust']} />
				<CodeBlock html={data.highlighted['enumUntaggedTs']} />
			</div>
		</div>
	</section>
</div>
