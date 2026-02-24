<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Branded Newtypes — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Branded Newtypes</h1>
	<p class="text-text-muted leading-relaxed">
		Single-field tuple structs (newtypes) become type aliases by default. Enable <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">branded_newtypes</code> for nominal type safety — preventing accidental mixing of structurally identical types like <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">UserId</code> and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">OrderId</code>.
	</p>

	<div>
		<span class="text-xs text-accent-rust mb-1 block">Rust</span>
		<CodeBlock html={data.highlighted['newtypeRust']} />
	</div>

	<div class="grid grid-cols-1 md:grid-cols-2 gap-3">
		<div>
			<span class="text-xs text-text-muted mb-1 block">Default output</span>
			<CodeBlock html={data.highlighted['newtypeDefaultTs']} />
		</div>
		<div>
			<span class="text-xs text-accent-ts mb-1 block">Branded output</span>
			<CodeBlock html={data.highlighted['newtypeBrandedTs']} />
		</div>
	</div>

	<div>
		<span class="text-xs text-text-muted mb-1 block">Enable via config or CLI</span>
		<CodeBlock html={data.highlighted['configToml']} />
	</div>
</div>
