<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Type Overrides — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Type Overrides</h1>
	<p class="text-text-muted leading-relaxed">
		Map external crate types (or any Rust type) to custom TypeScript types via
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>[codegen.type_overrides]</code
		>.
	</p>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<h2 class="text-xl font-semibold">Example</h2>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
	</div>

	<h2 class="text-xl font-semibold">Matching Rules</h2>
	<p class="text-text-muted leading-relaxed text-sm">
		Override keys are matched against type names by their last path segment. For example, key
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">"chrono::DateTime"</code>
		matches both the fully-qualified
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">chrono::DateTime&lt;Utc&gt;</code>
		and the imported
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">DateTime&lt;Utc&gt;</code>.
		If you use fully-qualified paths in your Rust source, exact full-path matching takes priority.
	</p>
	<p class="text-text-muted leading-relaxed text-sm">
		Overrides are applied before code generation — every occurrence of the matched type
		(including inside
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Vec&lt;T&gt;</code>,
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Option&lt;T&gt;</code>,
		etc.) is replaced with the specified TypeScript type, and generic parameters are stripped.
	</p>
</div>
