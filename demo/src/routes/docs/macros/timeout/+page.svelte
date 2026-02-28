<script lang="ts">
	import { rpc } from '$lib/client';
	import { RpcError } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	let loading = $state(false);

	let callLog: {
		sleepMs: number;
		actualMs: number | null;
		status: 'ok' | 'timeout' | 'error';
		error?: string;
	}[] = $state([]);

	async function run(ms: number) {
		loading = true;
		try {
			const result = await rpc.query('timeout_demo', { sleep_ms: ms });
			callLog = [...callLog.slice(-4), { sleepMs: ms, actualMs: result.actual_ms, status: 'ok' }];
		} catch (e: unknown) {
			if (e instanceof RpcError && e.status === 504) {
				callLog = [
					...callLog.slice(-4),
					{ sleepMs: ms, actualMs: null, status: 'timeout', error: '504 Gateway Timeout' }
				];
			} else {
				callLog = [
					...callLog.slice(-4),
					{
						sleepMs: ms,
						actualMs: null,
						status: 'error',
						error: e instanceof Error ? e.message : String(e)
					}
				];
			}
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>timeout — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="timeout" mono>
		Server-side timeout via
		<Code>tokio::time::timeout</Code>. Returns HTTP 504 if exceeded. Also forwarded to the
		TypeScript client as the default timeout for that procedure. Works with both queries and
		mutations.
	</PageHeader>

	<SectionHeading>Basic Usage</SectionHeading>
	<CodeBlock html={data.highlighted['basic']} />

	<SectionHeading>Duration Shorthand</SectionHeading>
	<CodeBlock html={data.highlighted['durations']} />

	<SectionHeading>Behavior</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		When the handler exceeds the timeout, the future is cancelled and the server returns 504. The
		TypeScript client receives an
		<a href="/docs/error-handling" class="text-accent-ts hover:underline"><Code>RpcError</Code></a> with
		status 504.
	</p>
	<CodeBlock html={data.highlighted['behavior']} />

	<SectionHeading>Combining Attributes</SectionHeading>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<DemoCard title="Server-Side Timeout">
		<p class="text-text-muted text-sm mb-3">
			This handler has a <Code>timeout = "3s"</Code> — it sleeps for the requested duration. Try a short
			sleep to see a successful response, then try exceeding 3 seconds to trigger a 504.
		</p>
		<div class="flex items-center gap-2 mb-3 flex-wrap">
			<Button onclick={() => run(500)} disabled={loading}>500ms</Button>
			<Button onclick={() => run(2000)} disabled={loading}>2s</Button>
			<button
				onclick={() => run(4000)}
				disabled={loading}
				class="rounded-md bg-red-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>4s (timeout!)</button
			>
		</div>
		{#if loading}
			<div class="text-xs text-text-faint mb-2">waiting for response...</div>
		{/if}
		{#if callLog.length > 0}
			<OutputBox mono>
				{#each callLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>sleep: <span class="text-text-primary">{entry.sleepMs}ms</span></span
						>
						{#if entry.status === 'ok'}
							<span class="text-text-muted"
								>actual: <span class="text-accent-rust">{entry.actualMs}ms</span></span
							>
							<span class="text-green-400">200 OK</span>
						{:else if entry.status === 'timeout'}
							<span class="text-red-400">{entry.error}</span>
						{:else}
							<span class="text-yellow-400">{entry.error}</span>
						{/if}
					</div>
				{/each}
			</OutputBox>
		{/if}
		<CollapsibleCode html={data.highlighted['timeoutDemoRust']} />
	</DemoCard>
</div>
