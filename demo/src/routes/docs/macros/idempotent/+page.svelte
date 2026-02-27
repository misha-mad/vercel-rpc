<script lang="ts">
	import { rpc } from '$lib/client';
	import { createMutation } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

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

	let openCode = $state(false);
</script>

<svelte:head>
	<title>idempotent — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">idempotent</h1>
	<p class="text-text-muted leading-relaxed">
		Mutations only. Marks the mutation as safe to retry on network errors. By default, mutations are <strong
			class="text-text-primary">never</strong
		> retried — even with a retry policy configured. Queries (GET) are always retryable.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Good Candidates</h2>
	<p class="text-text-muted text-sm mb-2">
		Mark mutations as idempotent when repeated calls produce the same result: upserts, overwrites,
		deletes.
	</p>
	<CodeBlock html={data.highlighted['examples']} />

	<h2 class="text-xl font-semibold">Retry Behavior</h2>
	<p class="text-text-muted text-sm mb-2">
		When marked idempotent, the mutation follows the same <a
			href="/docs/client/retry"
			class="text-accent-ts hover:underline">retry policy</a
		> as queries. Without it, the mutation is never retried regardless of the retry configuration.
	</p>
	<CodeBlock html={data.highlighted['retryBehavior']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Idempotent Upsert</h3>
		<p class="text-text-muted text-sm mb-3">
			This mutation stores a value on the server (upsert). Click the same button twice — the result
			is identical, proving it's safe to retry. The call counter increments, but the stored value
			stays the same.
		</p>
		<div class="flex items-center gap-2 mb-3 flex-wrap">
			<button
				onclick={() => upsert(42)}
				disabled={demo.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Set 42</button
			>
			<button
				onclick={() => upsert(99)}
				disabled={demo.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Set 99</button
			>
			<button
				onclick={() => upsert(0)}
				disabled={demo.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Set 0</button
			>
		</div>
		{#if callLog.length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
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
			</div>
		{/if}
		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => (openCode = !openCode)}
		>
			{openCode ? '▾ Hide' : '▸ Show'} Rust
		</button>
		{#if openCode}
			<div class="mt-3">
				<CodeBlock html={data.highlighted['idempotentDemoRust']} />
			</div>
		{/if}
	</div>
</div>
