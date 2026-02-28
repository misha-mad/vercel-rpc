<script lang="ts">
	import { rpc } from '$lib/client';
	import { createMutation } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	const demo = createMutation(rpc, 'idempotent_demo');

	let callLog: { value: number; previous: number; current: number; totalCalls: number }[] = $state(
		[]
	);

	async function upsert(value: number) {
		try {
			const result = await demo.mutateAsync({ value });
			callLog = [
				...callLog.slice(-4),
				{
					value,
					previous: result.previous,
					current: result.current,
					totalCalls: result.total_calls
				}
			];
		} catch {
			// ignore
		}
	}
</script>

<svelte:head>
	<title>idempotent — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="idempotent" mono>
		Mutations only. Marks the mutation as safe to retry on network errors. By default, mutations are <strong
			class="text-text-primary">never</strong
		> retried — even with a retry policy configured. Queries (GET) are always retryable.
	</PageHeader>

	<SectionHeading>Basic Usage</SectionHeading>
	<CodeBlock html={data.highlighted['basic']} />

	<SectionHeading>Good Candidates</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Mark mutations as idempotent when repeated calls produce the same result: upserts, overwrites,
		deletes.
	</p>
	<CodeBlock html={data.highlighted['examples']} />

	<SectionHeading>Retry Behavior</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		When marked idempotent, the mutation follows the same <a
			href="/docs/client/retry"
			class="text-accent-ts hover:underline">retry policy</a
		> as queries. Without it, the mutation is never retried regardless of the retry configuration.
	</p>
	<CodeBlock html={data.highlighted['retryBehavior']} />

	<SectionHeading>Combining Attributes</SectionHeading>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<DemoCard title="Idempotent Upsert">
		<p class="text-text-muted text-sm mb-3">
			This mutation stores a value on the server (upsert). Click the same button twice — the result
			is identical, proving it's safe to retry. The call counter increments, but the stored value
			stays the same.
		</p>
		<div class="flex items-center gap-2 mb-3 flex-wrap">
			<Button onclick={() => upsert(42)} disabled={demo.isLoading}>Set 42</Button>
			<Button onclick={() => upsert(99)} disabled={demo.isLoading}>Set 99</Button>
			<Button onclick={() => upsert(0)} disabled={demo.isLoading}>Set 0</Button>
		</div>
		{#if callLog.length > 0}
			<OutputBox mono>
				{#each callLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>{entry.previous} → <span class="text-accent-rust">{entry.current}</span></span
						>
						<span class="text-text-muted"
							>calls: <span class="text-text-primary">{entry.totalCalls}</span></span
						>
						{#if entry.previous === entry.current}
							<span class="text-green-400">same value (idempotent)</span>
						{:else}
							<span class="text-yellow-400">value changed</span>
						{/if}
					</div>
				{/each}
			</OutputBox>
		{/if}
		<CollapsibleCode html={data.highlighted['idempotentDemoRust']} />
	</DemoCard>
</div>
