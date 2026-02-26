<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Serialization — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Serialization</h1>
	<p class="text-text-muted leading-relaxed">
		Override <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serialize</code> and
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">deserialize</code> to use a custom
		serialization format instead of plain JSON.
	</p>

	<h2 class="text-xl font-semibold">Default</h2>
	<p class="text-text-muted text-sm mb-2">
		When not specified, the client uses <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.stringify</code
		>
		and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.parse</code>.
	</p>
	<CodeBlock html={data.highlighted['defaultSerialization']} />

	<h2 class="text-xl font-semibold">Custom (superjson)</h2>
	<p class="text-text-muted text-sm mb-2">
		Use a library like <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>superjson</code
		>
		to support <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Date</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">BigInt</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Map</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Set</code> and other types not natively
		supported by JSON.
	</p>
	<CodeBlock html={data.highlighted['customSerialization']} />

	<h2 class="text-xl font-semibold">Type Signature</h2>
	<CodeBlock html={data.highlighted['signature']} />
</div>
