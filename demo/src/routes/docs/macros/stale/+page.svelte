<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let fetchLog: { serverTs: number; clientTs: number }[] = $state([]);

	function logFetch(serverTs: number) {
		fetchLog = [...fetchLog.slice(-4), { serverTs, clientTs: Math.floor(Date.now() / 1000) }];
	}

	const cachedTimeStale = createQuery(rpc, 'cached_time_stale', {
		onSuccess: (d) => logFetch(d.timestamp)
	});

	async function refetch() {
		await cachedTimeStale.refetch();
	}

	let openCode = $state(false);
</script>

<svelte:head>
	<title>stale — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">stale</h1>
	<p class="text-text-muted leading-relaxed">
		Adds <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>stale-while-revalidate</code
		>
		to the <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cache-Control</code>
		header. The CDN serves stale content while fetching a fresh response in the background. Requires
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">cache</code> — compilation error
		without it. Queries only.
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">With Private Cache</h2>
	<CodeBlock html={data.highlighted['private']} />

	<h2 class="text-xl font-semibold">Generated Headers</h2>
	<CodeBlock html={data.highlighted['headers']} />

	<h2 class="text-xl font-semibold">Requires cache</h2>
	<p class="text-text-muted text-sm mb-2">
		Using <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">stale</code> without
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">cache</code> is a compilation error.
	</p>
	<CodeBlock html={data.highlighted['error']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Stale-While-Revalidate — 10s + 30s</h3>
		<p class="text-text-muted text-xs mb-3 font-mono">
			Cache-Control: public, max-age=0, s-maxage=10, stale-while-revalidate=30
		</p>
		<p class="text-text-muted text-sm mb-3">
			Cached for 10s, then serves stale for up to 30s while revalidating in the background.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<button
				onclick={refetch}
				disabled={cachedTimeStale.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
				>Refetch</button
			>
		</div>
		{#if fetchLog.length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
				{#each fetchLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>server: <span class="text-accent-rust">{entry.serverTs}</span></span
						>
						<span class="text-text-muted"
							>client: <span class="text-text-primary">{entry.clientTs}</span></span
						>
						{#if i > 0 && entry.serverTs === fetchLog[i - 1].serverTs}
							<span class="text-green-400">cached</span>
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
				<CodeBlock html={data.highlighted['cachedTimeStaleRust']} />
			</div>
		{/if}
	</div>
</div>
