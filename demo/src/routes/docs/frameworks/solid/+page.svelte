<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>SolidJS — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">SolidJS</h1>
	<p class="text-text-muted leading-relaxed">
		Fine-grained reactive primitives using <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createSignal</code
		>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createEffect</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createMemo</code>. Input is a
		getter <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">() => value</code>.
		Returns signal accessors — call them to read:
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">data()</code>.
	</p>

	<h2 class="text-2xl font-semibold">Setup</h2>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['setupToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['setupCli']} />
	</div>

	<h2 class="text-2xl font-semibold">createQuery</h2>
	<CodeBlock html={data.highlighted['query']} large />

	<h2 class="text-2xl font-semibold">createMutation</h2>
	<CodeBlock html={data.highlighted['mutation']} large />
</div>
