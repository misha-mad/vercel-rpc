<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Field Naming — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Field Naming</h1>
	<p class="text-text-muted leading-relaxed">
		By default field names are emitted as-is. Set
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">camelCase</code> to convert
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">snake_case</code> fields to
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">camelCase</code> in the generated
		TypeScript.
	</p>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<h2 class="text-xl font-semibold">preserve (default)</h2>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['preserveTs']} />
	</div>

	<h2 class="text-xl font-semibold">camelCase</h2>
	<CodeBlock html={data.highlighted['camelTs']} />

	<p class="text-text-muted leading-relaxed text-sm">
		The transform also applies to struct variant fields in enums. Enum variant names and procedure
		names are not affected.
	</p>

	<p class="text-text-muted leading-relaxed text-sm">
		Priority: field-level
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(rename)</code> &gt;
		container
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">rename_all</code> &gt;
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">codegen.naming.fields</code>
		&gt; original name.
	</p>
</div>
