<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Getting Started — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Getting Started</h1>

	<p class="text-text-muted leading-relaxed">
		<strong class="text-text-primary">vercel-rpc</strong> is an end-to-end typesafe RPC toolkit for building
		serverless APIs with Rust on Vercel. Write plain Rust functions, and get a fully typed TypeScript
		client — no manual sync required.
	</p>

	<h2 class="text-2xl font-semibold">Installation</h2>

	<p class="text-text-muted text-sm mb-2">1. Install the CLI</p>
	<CodeBlock html={data.highlighted['installCli']} />

	<p class="text-text-muted text-sm mb-2">2. Add the macro crate to your Rust project</p>
	<CodeBlock html={data.highlighted['installCrate']} />

	<p class="text-text-muted text-sm mb-2">3. Generate TypeScript types and client</p>
	<CodeBlock html={data.highlighted['installGenerate']} />

	<h3 class="text-xl font-semibold mt-8">How it works</h3>

	<ol class="list-decimal list-inside space-y-2 text-text-muted">
		<li>Annotate Rust functions with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> or <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code></li>
		<li>The CLI scans your <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">api/</code> directory and parses Rust types via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code></li>
		<li>TypeScript types and a typed client are generated automatically</li>
		<li>Each Rust file deploys as a serverless lambda on Vercel</li>
	</ol>

	<h3 class="text-xl font-semibold mt-8">Quick example</h3>

	<a href="https://github.com/misha-mad/vercel-rpc/blob/main/demo/api/hello.rs" target="_blank" class="text-xs text-text-faint hover:text-accent-rust transition-colors mb-1 block">api/hello.rs</a>
	<CodeBlock html={data.highlighted['gettingStartedRust']} large />

	<CodeBlock html={data.highlighted['gettingStartedTs']} large />

	<CodeBlock html={data.highlighted['gettingStartedSvelte']} large />
</div>
