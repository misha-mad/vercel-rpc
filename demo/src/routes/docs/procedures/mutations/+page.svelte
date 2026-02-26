<script lang="ts">
	import { rpc } from '$lib/client';
	import { createMutation } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

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
	<h1 class="text-3xl font-bold">Mutations</h1>
	<p class="text-text-muted leading-relaxed">
		Use <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code> for
		write operations (create, update, delete). Unlike queries, mutations are triggered explicitly
		via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.mutate()</code> — they don't
		auto-refetch.
	</p>

	<!-- Code examples -->
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
		<CodeBlock html={data.highlighted['exampleSvelte']} />
	</div>

	<!-- Void input & Result -->
	<h2 class="text-xl font-semibold mt-8">Void Input &amp; Error Handling</h2>
	<p class="text-text-muted text-sm">
		Mutations can take no input (void) or return <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Result&lt;T, E&gt;</code>
		for typed error handling. Errors are propagated as <a href="/docs/error-handling" class="text-accent-ts hover:underline"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code></a>
		on the client side.
	</p>
	<CodeBlock html={data.highlighted['voidRust']} />

	<!-- Per-Call Options -->
	<h2 class="text-xl font-semibold mt-8">Per-Call Options</h2>
	<p class="text-text-muted text-sm">
		Every <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">mutate()</code> call
		accepts an optional trailing
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">CallOptions</code> object to override
		client-level defaults.
	</p>
	<CodeBlock html={data.highlighted['callOptionsType']} />
	<CodeBlock html={data.highlighted['callOptionsMutation']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>

	<!-- Echo: mutation with struct input/output -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Echo — Struct Mutation</h3>
		<p class="text-text-muted text-sm mb-4">
			Send a message, optionally uppercase it. Demonstrates <code
				class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createMutation</code
			> with struct input/output.
		</p>
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
			<div class="rounded-md bg-bg-code p-3 text-sm space-y-1">
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
			</div>
		{/if}
		{#if echo.isError}
			<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{echo.error?.message}</div>
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
	</div>
</div>
