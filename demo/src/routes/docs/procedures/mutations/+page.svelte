<script lang="ts">
	import { rpc } from '$lib/client';
	import { createMutation } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	// --- Echo (mutation with struct input/output) ---
	let echoMessage = $state('Hello from vercel-rpc!');
	let echoUppercase = $state(false);
	const echo = createMutation(rpc, 'echo');

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>Mutations — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Mutations</h1>
	<p class="text-text-muted leading-relaxed">
		Use <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code> for write operations (create, update, delete).
		Unlike queries, mutations are triggered explicitly via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.mutate()</code> — they don't auto-refetch.
	</p>

	<!-- Echo: mutation with struct input/output -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Echo — Struct Mutation</h3>
		<p class="text-text-muted text-sm mb-4">
			Send a message, optionally uppercase it. Demonstrates <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createMutation</code> with struct input/output.
		</p>
		<div class="flex flex-wrap items-center gap-3 mb-3">
			<input type="text" bind:value={echoMessage} placeholder="Enter message"
				class="flex-1 min-w-48 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts" />
			<label class="flex items-center gap-2 text-sm text-text-muted">
				<input type="checkbox" bind:checked={echoUppercase} class="rounded" />
				Uppercase
			</label>
			<button
				onclick={() => echo.mutate({ message: echoMessage, uppercase: echoUppercase })}
				disabled={echo.isLoading}
				class="rounded-md bg-accent-rust px-4 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50">
				{echo.isLoading ? 'Sending...' : 'Send'}
			</button>
		</div>
		{#if echo.data}
			<div class="rounded-md bg-bg-code p-3 text-sm space-y-1">
				<div><span class="text-text-muted">Original:</span> <span class="text-text-primary">{echo.data.original}</span></div>
				<div><span class="text-text-muted">Transformed:</span> <span class="text-green-400">{echo.data.transformed}</span></div>
				<div><span class="text-text-muted">Length:</span> <span class="text-text-primary">{echo.data.length}</span></div>
			</div>
		{/if}
		{#if echo.isError}
			<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{echo.error?.message}</div>
		{/if}
		<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('echo')}>
			{openCode['echo'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['echo']}
			<div class="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
				<div>
					<span class="text-xs text-accent-rust mb-1 block">Rust — api/echo.rs</span>
					<CodeBlock html={data.highlighted['echoRust']} />
				</div>
				<div>
					<span class="text-xs text-accent-ts mb-1 block">Svelte 5 Usage</span>
					<CodeBlock html={data.highlighted['echoTs']} />
				</div>
			</div>
		{/if}
	</div>
</div>
