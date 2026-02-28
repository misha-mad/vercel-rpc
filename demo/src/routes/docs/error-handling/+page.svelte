<script lang="ts">
	import { createRpcClient } from '$lib/rpc-client';
	import { RpcError } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	// --- Secret (protected endpoint with RpcClientConfig.headers) ---
	let secretResult = $state('');
	let secretError = $state('');
	let secretLoading = $state(false);

	async function callSecret(withToken: boolean) {
		secretLoading = true;
		secretResult = '';
		secretError = '';
		try {
			const client = createRpcClient(
				withToken
					? { baseUrl: '/api', headers: { Authorization: 'Bearer secret-token-123' } }
					: { baseUrl: '/api' }
			);
			secretResult = await client.query('secret');
		} catch (e) {
			if (e instanceof RpcError) {
				const data = e.data as { error?: { message?: string } } | undefined;
				secretError = data?.error?.message ?? e.message;
			} else {
				secretError = `${e}`;
			}
		} finally {
			secretLoading = false;
		}
	}

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>Error Handling — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Error Handling">
		When a Rust function returns <Code>Result&lt;T, E&gt;</Code>, errors are propagated as
		<Code>RpcError</Code> on the client side. You can also return custom HTTP status codes for authorization
		or validation errors.
	</PageHeader>
	<p class="text-text-muted leading-relaxed">
		The <Code>RpcError</Code> class provides <Code>.status</Code>,
		<Code>.message</Code>, and
		<Code>.data</Code> for structured error handling.
	</p>

	<h2 class="text-2xl font-semibold">RpcError API</h2>
	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm text-left">
			<thead class="bg-bg-code text-text-muted text-xs uppercase">
				<tr>
					<th class="px-4 py-3">Property</th>
					<th class="px-4 py-3">Type</th>
					<th class="px-4 py-3">Description</th>
				</tr>
			</thead>
			<tbody class="text-text-primary">
				<tr class="border-b border-border">
					<td class="px-4 py-2"><Code>.status</Code></td>
					<td class="px-4 py-2"><Code>number</Code></td>
					<td class="px-4 py-2 text-text-muted">HTTP status code (e.g. 401, 500)</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"><Code>.message</Code></td>
					<td class="px-4 py-2"><Code>string</Code></td>
					<td class="px-4 py-2 text-text-muted">Human-readable error message</td>
				</tr>
				<tr>
					<td class="px-4 py-2"><Code>.data</Code></td>
					<td class="px-4 py-2"><Code>unknown</Code></td>
					<td class="px-4 py-2 text-text-muted">Parsed JSON error body from server</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-2xl font-semibold">Global onError Hook</h2>
	<p class="text-text-muted text-sm mb-2">
		Catch all errors at the client level. The <Code>onError</Code>
		callback receives an
		<Code>ErrorContext</Code> with the procedure name, attempt number, and whether the client will retry.
	</p>
	<CodeBlock html={data.highlighted['onErrorCallback']} />

	<h2 class="text-2xl font-semibold">Reactive Error Handling</h2>
	<p class="text-text-muted text-sm mb-2">
		Framework wrappers expose <Code>isError</Code>
		and <Code>error</Code> state for conditional UI rendering.
	</p>
	<CodeBlock html={data.highlighted['frameworkError']} />

	<h2 class="text-2xl font-semibold">Mutation Errors</h2>
	<p class="text-text-muted text-sm mb-2">
		Use <Code>mutateAsync</Code> with try/catch for fine-grained control, or
		<Code>onError</Code>
		callback for fire-and-forget style.
	</p>
	<CodeBlock html={data.highlighted['mutationError']} />

	<h2 class="text-2xl font-semibold">Timeout &amp; Abort</h2>
	<p class="text-text-muted text-sm mb-2">
		Timeouts and manual aborts throw a <Code>DOMException</Code>
		with <Code>name: 'AbortError'</Code>, not <Code>RpcError</Code>.
	</p>
	<CodeBlock html={data.highlighted['timeoutError']} />

	<h2 class="text-2xl font-semibold">Secret — Protected Endpoint</h2>
	<p class="text-text-muted text-sm mb-4">
		This endpoint requires an <Code>Authorization</Code> header. Try calling it with and without a token
		to see error handling in action.
	</p>
	<DemoCard>
		<div class="flex flex-wrap gap-3 mb-3">
			<Button onclick={() => callSecret(true)} disabled={secretLoading}>Call with token</Button>
			<button
				onclick={() => callSecret(false)}
				disabled={secretLoading}
				class="rounded-md border border-red-500 px-4 py-1.5 text-sm font-medium text-red-400 transition-opacity hover:opacity-85 disabled:opacity-50"
			>
				Call without token
			</button>
		</div>
		{#if secretLoading}
			<OutputBox>Loading...</OutputBox>
		{/if}
		{#if secretResult}
			<OutputBox status="success">{secretResult}</OutputBox>
		{/if}
		{#if secretError}
			<OutputBox status="error">{secretError}</OutputBox>
		{/if}
		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => toggleCode('secret')}
		>
			{openCode['secret'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['secret']}
			<div class="mt-3 space-y-3">
				<a
					href="https://github.com/misha-mad/metaxy/blob/main/demo/api/secret.rs"
					target="_blank"
					class="text-xs text-text-faint hover:text-accent-rust transition-colors mb-1 block"
					>api/secret.rs</a
				>
				<CodeBlock html={data.highlighted['secretRust']} />
				<CodeBlock html={data.highlighted['secretTs']} />
			</div>
		{/if}
	</DemoCard>
</div>
