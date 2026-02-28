<script lang="ts">
	import { rpc } from '$lib/client';
	import { RpcError } from '$lib/rpc-client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	let result: string | undefined = $state();
	let error: string | undefined = $state();
	let loading = $state(false);
	let usedHeaders: Record<string, string> = $state({});

	async function fetchSecret(withToken: boolean) {
		loading = true;
		result = undefined;
		error = undefined;
		const headers: Record<string, string> = withToken
			? { Authorization: 'Bearer secret-token-123' }
			: {};
		usedHeaders = headers;
		try {
			result = await rpc.query('secret', { headers });
		} catch (e) {
			error = e instanceof RpcError ? `${e.status}: ${e.message}` : String(e);
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Headers — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Headers">
		Set default headers for all requests, or provide an async function that resolves headers before
		each call. Per-call headers can override client-level defaults.
	</PageHeader>

	<SectionHeading>Static Headers</SectionHeading>
	<CodeBlock html={data.highlighted['staticHeaders']} />

	<SectionHeading>Async Headers</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Pass an async function to resolve headers dynamically — useful for refreshing auth tokens.
	</p>
	<CodeBlock html={data.highlighted['asyncHeaders']} />

	<SectionHeading>Per-Call Headers</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Override headers for a single call via <Code>CallOptions</Code>.
	</p>
	<CodeBlock html={data.highlighted['perCallHeaders']} />

	<SectionHeading>Merge Order</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Per-call headers are merged on top of client headers. Same-key per-call headers win.
	</p>
	<CodeBlock html={data.highlighted['mergeOrder']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<p class="text-text-muted text-sm">
		The <Code>secret</Code> endpoint requires a Bearer token. Try with and without it.
	</p>

	<DemoCard>
		<div class="flex items-center gap-2 mb-4">
			<button
				onclick={() => fetchSecret(false)}
				disabled={loading}
				class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Without token</button
			>
			<Button onclick={() => fetchSecret(true)} disabled={loading}>With token</Button>
			{#if loading}
				<span class="text-sm text-text-muted">Loading...</span>
			{/if}
		</div>

		{#if result || error}
			<OutputBox mono>
				<div class="text-text-faint">
					Headers: {JSON.stringify(usedHeaders)}
				</div>
				{#if result}
					<div class="text-green-400">200 OK → {JSON.stringify(result)}</div>
				{:else if error}
					<div class="text-red-400">{error}</div>
				{/if}
			</OutputBox>
		{/if}

		<CollapsibleCode html={data.highlighted['secretRust']} />
	</DemoCard>
</div>
