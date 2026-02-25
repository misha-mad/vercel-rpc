<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Generics — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Generics</h1>
	<p class="text-text-muted leading-relaxed">
		Generic structs and enums produce generic TypeScript interfaces. Type parameters are preserved
		in procedure signatures, so <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>Paginated&lt;User&gt;</code
		>
		in Rust becomes
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Paginated&lt;User&gt;</code> in TypeScript.
	</p>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['genericsRust']} />
		<CodeBlock html={data.highlighted['genericsTs']} />
	</div>
</div>
