<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let name = $state('World');
	const hello = createQuery(rpc, 'hello', () => name);
</script>

<svelte:head>
	<title>Getting Started — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Getting Started</h1>

	<p class="text-text-muted leading-relaxed">
		<strong class="text-text-primary">metaxy</strong> is an end-to-end typesafe RPC toolkit for building
		serverless APIs with Rust on Vercel. Write plain Rust functions, and get a fully typed TypeScript
		client — no manual sync required.
	</p>

	<h2 class="text-2xl font-semibold">Installation</h2>

	<p class="text-text-muted text-sm mb-2">Install the CLI</p>
	<CodeBlock html={data.highlighted['installCli']} />

	<p class="text-text-muted text-sm mb-2">Add the macro crate to your Rust project</p>
	<CodeBlock html={data.highlighted['installCrate']} />

	<h2 class="text-2xl font-semibold">Quick Start</h2>

	<p class="text-text-muted text-sm mb-2">Write a Rust lambda</p>
	<CodeBlock html={data.highlighted['writeLambda']} />

	<p class="text-text-muted text-sm mb-2">Generate TypeScript types and client</p>
	<CodeBlock html={data.highlighted['installGenerate']} />

	<p class="text-text-muted text-sm mb-2">Call your lambda</p>

	<ul class="list-disc list-inside text-text-muted text-sm mb-2">
		<li>Your any TS frontend</li>
	</ul>
	<CodeBlock html={data.highlighted['gettingStartedTs']} />

	<ul class="list-disc list-inside text-text-muted text-sm mb-2">
		<li>Or 1 of 4 frameworks</li>
	</ul>
	<CodeBlock html={data.highlighted['gettingStartedSvelte']} />

	<h3 class="text-xl font-semibold mt-8">Try it live</h3>

	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<p class="text-text-muted text-sm mb-4">
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
				>createQuery(rpc, "hello", () => name)</code
			> — auto-refetches as you type.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<input
				type="text"
				bind:value={name}
				placeholder="Enter your name"
				class="rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts"
			/>
			<button
				onclick={() => hello.refetch()}
				disabled={hello.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Refetch</button
			>
		</div>
		{#if hello.isLoading && !hello.data}
			<div class="rounded-md bg-bg-code p-3 text-sm text-text-muted">Loading...</div>
		{:else if hello.data}
			<div class="rounded-md bg-bg-code p-3 text-sm text-green-400">{hello.data}</div>
		{/if}
		{#if hello.isError}
			<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{hello.error?.message}</div>
		{/if}
	</div>

	<h3 class="text-xl font-semibold mt-8">How it works</h3>

	<ol class="list-decimal list-inside space-y-2 text-text-muted">
		<li>
			Annotate Rust functions with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
				>#[rpc_query]</code
			>
			or <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code>
		</li>
		<li>
			The CLI scans your <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">api/</code
			>
			directory and parses Rust types via
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code>
		</li>
		<li>TypeScript types and a typed client are generated automatically</li>
		<li>Each Rust file deploys as a serverless lambda on Vercel</li>
	</ol>
</div>
