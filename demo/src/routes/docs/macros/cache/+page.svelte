<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	// --- Cached time demos ---
	const cachedTime = createQuery(rpc, 'cached_time');
	const cachedTimeStale = createQuery(rpc, 'cached_time_stale');
	const cachedTimePrivate = createQuery(rpc, 'cached_time_private');

	let fetchLog: Record<string, { serverTs: number; clientTs: number }[]> = $state({
		public: [],
		stale: [],
		private: []
	});

	function logFetch(key: 'public' | 'stale' | 'private', serverTs: number | undefined) {
		if (serverTs !== undefined) {
			fetchLog[key] = [...fetchLog[key].slice(-4), { serverTs, clientTs: Math.floor(Date.now() / 1000) }];
		}
	}

	async function refetchPublic() {
		await cachedTime.refetch();
		logFetch('public', cachedTime.data?.timestamp);
	}

	async function refetchStale() {
		await cachedTimeStale.refetch();
		logFetch('stale', cachedTimeStale.data?.timestamp);
	}

	async function refetchPrivate() {
		await cachedTimePrivate.refetch();
		logFetch('private', cachedTimePrivate.data?.timestamp);
	}

	// Log initial fetches
	$effect(() => {
		if (cachedTime.data) logFetch('public', cachedTime.data.timestamp);
	});
	$effect(() => {
		if (cachedTimeStale.data) logFetch('stale', cachedTimeStale.data.timestamp);
	});
	$effect(() => {
		if (cachedTimePrivate.data) logFetch('private', cachedTimePrivate.data.timestamp);
	});

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>cache — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold font-mono">cache</h1>
	<p class="text-text-muted leading-relaxed">
		Add <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cache-Control</code> headers
		to successful responses. Queries only — mutations cannot be cached (compilation error).
	</p>

	<h2 class="text-xl font-semibold">Basic Usage</h2>
	<CodeBlock html={data.highlighted['basic']} />

	<h2 class="text-xl font-semibold">Duration Shorthand</h2>
	<CodeBlock html={data.highlighted['durations']} />
	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm">
			<thead>
				<tr class="text-left text-text-faint border-b border-border bg-bg-code">
					<th class="px-4 py-2 font-medium">Suffix</th>
					<th class="px-4 py-2 font-medium">Unit</th>
					<th class="px-4 py-2 font-medium">Example</th>
				</tr>
			</thead>
			<tbody class="text-text-muted">
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs">s</td>
					<td class="px-4 py-1.5">seconds</td>
					<td class="px-4 py-1.5 font-mono text-xs">30s → max-age=30</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs">m</td>
					<td class="px-4 py-1.5">minutes</td>
					<td class="px-4 py-1.5 font-mono text-xs">5m → max-age=300</td>
				</tr>
				<tr class="border-b border-border/50">
					<td class="px-4 py-1.5 font-mono text-xs">h</td>
					<td class="px-4 py-1.5">hours</td>
					<td class="px-4 py-1.5 font-mono text-xs">1h → max-age=3600</td>
				</tr>
				<tr>
					<td class="px-4 py-1.5 font-mono text-xs">d</td>
					<td class="px-4 py-1.5">days</td>
					<td class="px-4 py-1.5 font-mono text-xs">1d → max-age=86400</td>
				</tr>
			</tbody>
		</table>
	</div>

	<h2 class="text-xl font-semibold">Public vs Private</h2>
	<p class="text-text-muted text-sm mb-2">
		By default caches are public (shared on CDN). Prefix with
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">private,</code> for user-specific
		data that should only be cached in the browser.
	</p>
	<CodeBlock html={data.highlighted['private']} />

	<h2 class="text-xl font-semibold">With stale-while-revalidate</h2>
	<p class="text-text-muted text-sm mb-2">
		Combine with <a href="/docs/macros/stale" class="text-accent-ts hover:underline">stale</a>
		to serve stale content while fetching fresh data in the background.
	</p>
	<CodeBlock html={data.highlighted['withStale']} />

	<h2 class="text-xl font-semibold">Combining Attributes</h2>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<p class="text-text-muted text-sm">
		Each demo calls a real lambda that returns the server timestamp. When cached, the server time
		stays the same across refetches while the client time updates.
	</p>

	<!-- Public cache: 30s -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Public — CDN Cache 30s</h3>
		<p class="text-text-muted text-xs mb-3 font-mono">
			Cache-Control: public, max-age=0, s-maxage=30
		</p>
		<div class="flex items-center gap-3 mb-3">
			<button
				onclick={refetchPublic}
				disabled={cachedTime.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
			>Refetch</button>
			{#if cachedTime.isLoading && !cachedTime.data}
				<span class="text-sm text-text-muted">Loading...</span>
			{/if}
		</div>
		{#if fetchLog['public'].length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
				{#each fetchLog['public'] as entry, i}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted">server: <span class="text-accent-rust">{entry.serverTs}</span></span>
						<span class="text-text-muted">client: <span class="text-text-primary">{entry.clientTs}</span></span>
						{#if i > 0 && entry.serverTs === fetchLog['public'][i - 1].serverTs}
							<span class="text-green-400">cached</span>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => toggleCode('cachedTime')}
		>
			{openCode['cachedTime'] ? '▾ Hide' : '▸ Show'} Rust
		</button>
		{#if openCode['cachedTime']}
			<div class="mt-3">
				<CodeBlock html={data.highlighted['cachedTimeRust']} />
			</div>
		{/if}
	</div>

	<!-- Stale: 10s cache + 30s stale -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Stale-While-Revalidate — 10s + 30s</h3>
		<p class="text-text-muted text-xs mb-3 font-mono">
			Cache-Control: public, max-age=0, s-maxage=10, stale-while-revalidate=30
		</p>
		<div class="flex items-center gap-3 mb-3">
			<button
				onclick={refetchStale}
				disabled={cachedTimeStale.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
			>Refetch</button>
		</div>
		{#if fetchLog['stale'].length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
				{#each fetchLog['stale'] as entry, i}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted">server: <span class="text-accent-rust">{entry.serverTs}</span></span>
						<span class="text-text-muted">client: <span class="text-text-primary">{entry.clientTs}</span></span>
						{#if i > 0 && entry.serverTs === fetchLog['stale'][i - 1].serverTs}
							<span class="text-green-400">cached</span>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => toggleCode('cachedTimeStale')}
		>
			{openCode['cachedTimeStale'] ? '▾ Hide' : '▸ Show'} Rust
		</button>
		{#if openCode['cachedTimeStale']}
			<div class="mt-3">
				<CodeBlock html={data.highlighted['cachedTimeStaleRust']} />
			</div>
		{/if}
	</div>

	<!-- Private: browser-only 1m -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-1">Private — Browser Cache 1m</h3>
		<p class="text-text-muted text-xs mb-3 font-mono">
			Cache-Control: private, max-age=60
		</p>
		<div class="flex items-center gap-3 mb-3">
			<button
				onclick={refetchPrivate}
				disabled={cachedTimePrivate.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
			>Refetch</button>
		</div>
		{#if fetchLog['private'].length > 0}
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono space-y-1 overflow-x-auto">
				{#each fetchLog['private'] as entry, i}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted">server: <span class="text-accent-rust">{entry.serverTs}</span></span>
						<span class="text-text-muted">client: <span class="text-text-primary">{entry.clientTs}</span></span>
						{#if i > 0 && entry.serverTs === fetchLog['private'][i - 1].serverTs}
							<span class="text-green-400">cached</span>
						{/if}
					</div>
				{/each}
			</div>
		{/if}
		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => toggleCode('cachedTimePrivate')}
		>
			{openCode['cachedTimePrivate'] ? '▾ Hide' : '▸ Show'} Rust
		</button>
		{#if openCode['cachedTimePrivate']}
			<div class="mt-3">
				<CodeBlock html={data.highlighted['cachedTimePrivateRust']} />
			</div>
		{/if}
	</div>

	<p class="text-text-faint text-xs italic">
		Caching is active on the Vercel deployment. In local dev, headers are set but there is no CDN layer.
	</p>
</div>
