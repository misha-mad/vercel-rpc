<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>React — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">React</h1>
	<p class="text-text-muted leading-relaxed">
		Hook-based wrappers using <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">useState</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">useEffect</code>. Input is a plain value (not a getter) — dependencies tracked via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">JSON.stringify</code>.
	</p>

	<h2 class="text-2xl font-semibold">Setup</h2>
	<CodeBlock html={data.highlighted['setup']} />

	<h2 class="text-2xl font-semibold">useQuery</h2>
	<CodeBlock html={data.highlighted['query']} large />

	<h2 class="text-2xl font-semibold">useMutation</h2>
	<CodeBlock html={data.highlighted['mutation']} large />
</div>
