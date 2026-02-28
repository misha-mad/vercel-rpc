<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import Code from '$lib/components/Code.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import Button from '$lib/components/Button.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	let name = $state('World');
	const hello = createQuery(rpc, 'hello', () => name);
</script>

<svelte:head>
	<title>Getting Started — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Getting Started">
		<strong class="text-text-primary">metaxy</strong> is an end-to-end typesafe RPC toolkit for building
		serverless APIs with Rust on Vercel. Write plain Rust functions, and get a fully typed TypeScript
		client — no manual sync required.
	</PageHeader>

	<SectionHeading level="large">Installation</SectionHeading>

	<p class="text-text-muted text-sm mb-2">Install the CLI</p>
	<CodeBlock html={data.highlighted['installCli']} />

	<p class="text-text-muted text-sm mb-2">Add the macro crate to your Rust project</p>
	<CodeBlock html={data.highlighted['installCrate']} />

	<SectionHeading level="large">Quick Start</SectionHeading>

	<p class="text-text-muted text-sm mb-2">Write a Rust lambda</p>
	<CodeBlock html={data.highlighted['writeLambda']} />

	<p class="text-text-muted text-sm mb-2">Generate TypeScript types and client</p>
	<CodeBlock html={data.highlighted['installGenerate']} />

	<p class="text-text-muted text-sm mb-2">Call your lambda</p>

	<ul class="list-disc list-inside text-text-muted text-sm mb-2">
		<li>Any TypeScript frontend</li>
	</ul>
	<CodeBlock html={data.highlighted['gettingStartedTs']} />

	<ul class="list-disc list-inside text-text-muted text-sm mb-2">
		<li>Or 1 of 4 frameworks</li>
	</ul>
	<CodeBlock html={data.highlighted['gettingStartedSvelte']} />

	<SectionHeading>Try it live</SectionHeading>

	<DemoCard>
		<p class="text-text-muted text-sm mb-4">
			<Code>createQuery(rpc, "hello", () => name)</Code> — auto-refetches as you type.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<input
				type="text"
				bind:value={name}
				placeholder="Enter your name"
				class="rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts"
			/>
			<Button onclick={() => hello.refetch()} disabled={hello.isLoading}>Refetch</Button>
		</div>
		{#if hello.isLoading && !hello.data}
			<OutputBox>Loading...</OutputBox>
		{:else if hello.data}
			<OutputBox status="success">{hello.data}</OutputBox>
		{/if}
		{#if hello.isError}
			<OutputBox status="error">{hello.error?.message}</OutputBox>
		{/if}
	</DemoCard>

	<SectionHeading>How it works</SectionHeading>

	<ol class="list-decimal list-inside space-y-2 text-text-muted">
		<li>
			Annotate Rust functions with <a
				href="/docs/procedures/queries"
				class="text-accent-ts hover:underline"
				><Code>#[rpc_query]</Code></a
			>
			or
			<a href="/docs/procedures/mutations" class="text-accent-ts hover:underline"
				><Code>#[rpc_mutation]</Code></a
			>
		</li>
		<li>
			The CLI scans your <Code>api/</Code>
			directory and parses Rust types via
			<Code>syn</Code>
		</li>
		<li>TypeScript types and a typed client are generated automatically</li>
		<li>Each Rust file deploys as a serverless lambda on Vercel</li>
	</ol>
</div>
