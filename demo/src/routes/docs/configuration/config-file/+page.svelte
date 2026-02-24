<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>rpc.config.toml — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">rpc.config.toml</h1>
	<p class="text-text-muted leading-relaxed">
		Place an <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">rpc.config.toml</code> in your project root. The CLI auto-discovers it by walking up from the current directory. Use <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">--config &lt;path&gt;</code> to override or <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">--no-config</code> to disable.
	</p>

	<p class="text-text-muted text-sm">
		Resolution order (highest wins): CLI flag > config file > built-in default.
	</p>

	<CodeBlock html={data.highlighted['fullConfig']} large />

	<div class="space-y-4">
		<h2 class="text-2xl font-semibold">Sections</h2>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold">[input]</h3>
			<p class="text-text-muted text-sm">Where to find Rust source files. <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">dir</code> is the root directory, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">include</code>/<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">exclude</code> are glob patterns.</p>
		</div>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold">[output]</h3>
			<p class="text-text-muted text-sm">Generated file paths. <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">types</code> and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">client</code> are always generated. Framework wrappers (<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">svelte</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">react</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">vue</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">solid</code>) are opt-in.</p>
		</div>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold">[codegen]</h3>
			<p class="text-text-muted text-sm">Code generation options: <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">preserve_docs</code> forwards Rust doc comments as JSDoc, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">branded_newtypes</code> enables nominal types, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint_types</code> maps large integers to <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">type_overrides</code> maps external crate types.</p>
		</div>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold">[watch]</h3>
			<p class="text-text-muted text-sm"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">debounce_ms</code> controls how long to wait after a file change before regenerating. <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">clear_screen</code> clears the terminal before each regeneration.</p>
		</div>
	</div>
</div>
