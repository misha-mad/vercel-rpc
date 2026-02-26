<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Branded Newtypes — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Branded Newtypes</h1>
	<p class="text-text-muted leading-relaxed">
		Single-field tuple structs (newtypes) become type aliases by default. Enable <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">branded_newtypes</code
		>
		for nominal type safety — preventing accidental mixing of structurally identical types like
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">UserId</code>
		and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">OrderId</code>.
	</p>

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
