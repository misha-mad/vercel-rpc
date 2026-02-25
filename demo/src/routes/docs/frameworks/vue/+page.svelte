<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Vue 3 — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Vue 3</h1>
	<p class="text-text-muted leading-relaxed">
		Composition API composables using <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">ref</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">watch</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">computed</code>. Input is a getter <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">() => value</code>. Returns <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Ref&lt;T&gt;</code> properties.
	</p>

	<h2 class="text-2xl font-semibold">Setup</h2>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['setupToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['setupCli']} />
	</div>

	<h2 class="text-2xl font-semibold">useQuery</h2>
	<CodeBlock html={data.highlighted['query']} large />

	<h2 class="text-2xl font-semibold">useMutation</h2>
	<CodeBlock html={data.highlighted['mutation']} large />
</div>
