<script lang="ts">
	import { rpc } from '$lib/client';
	import { createMutation } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	// --- Echo (mutation with struct input/output) ---
	let echoMessage = $state('Hello from metaxy!');
	let echoUppercase = $state(false);
	const echo = createMutation(rpc, 'echo');

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>Mutations — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Mutations">
		Use <Code>#[rpc_mutation]</Code> for write operations (create, update, delete). Unlike queries, mutations
		are triggered explicitly via <Code>.mutate()</Code> — they don't auto-refetch.
	</PageHeader>

	<!-- Code examples -->
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
		<CodeBlock html={data.highlighted['exampleSvelte']} />
	</div>

	<!-- Void input & Result -->
	<SectionHeading>Void Input &amp; Error Handling</SectionHeading>
	<p class="text-text-muted text-sm">
		Mutations can take no input (void) or return <Code>Result&lt;T, E&gt;</Code>
		for typed error handling. Errors are propagated as
		<a href="/docs/error-handling" class="text-accent-ts hover:underline"><Code>RpcError</Code></a>
		on the client side.
	</p>
	<CodeBlock html={data.highlighted['voidRust']} />

	<!-- Per-Call Options -->
	<SectionHeading>Per-Call Options</SectionHeading>
	<p class="text-text-muted text-sm">
		Every <Code>mutate()</Code> call accepts an optional trailing
		<Code>CallOptions</Code> object to override client-level defaults.
	</p>
	<CodeBlock html={data.highlighted['callOptionsType']} />
	<CodeBlock html={data.highlighted['callOptionsMutation']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>

	<!-- Echo: mutation with struct input/output -->
	<DemoCard
		title="Echo — Struct Mutation"
		subtitle="Send a message, optionally uppercase it. Demonstrates createMutation with struct input/output."
	>
		<div class="flex flex-wrap items-center gap-3 mb-3">
			<input
				type="text"
				bind:value={echoMessage}
				placeholder="Enter message"
				class="flex-1 min-w-48 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts"
			/>
			<label class="flex items-center gap-2 text-sm text-text-muted">
				<input type="checkbox" bind:checked={echoUppercase} class="rounded" />
				Uppercase
			</label>
			<button
				onclick={() => echo.mutate({ message: echoMessage, uppercase: echoUppercase })}
				disabled={echo.isLoading}
				class="rounded-md bg-accent-rust px-4 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
			>
				{echo.isLoading ? 'Sending...' : 'Send'}
			</button>
		</div>
		{#if echo.data}
			<OutputBox>
				<div>
					<span class="text-text-muted">Original:</span>
					<span class="text-text-primary">{echo.data.original}</span>
				</div>
				<div>
					<span class="text-text-muted">Transformed:</span>
					<span class="text-green-400">{echo.data.transformed}</span>
				</div>
				<div>
					<span class="text-text-muted">Length:</span>
					<span class="text-text-primary">{echo.data.length}</span>
				</div>
			</OutputBox>
		{/if}
		{#if echo.isError}
			<OutputBox status="error">{echo.error?.message}</OutputBox>
		{/if}
		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => toggleCode('echo')}
		>
			{openCode['echo'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['echo']}
			<div class="mt-3 space-y-3">
				<a
					href="https://github.com/misha-mad/metaxy/blob/main/demo/api/echo.rs"
					target="_blank"
					class="text-xs text-text-faint hover:text-accent-rust transition-colors mb-1 block"
					>api/echo.rs</a
				>
				<CodeBlock html={data.highlighted['echoRust']} />
				<CodeBlock html={data.highlighted['echoTs']} />
			</div>
		{/if}
	</DemoCard>
</div>
