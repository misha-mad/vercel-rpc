<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	let fetchLog: { serverTs: number; clientTs: number; cached: boolean }[] = $state([]);

	function logFetch(serverTs: number) {
		const prev = fetchLog.at(-1);
		fetchLog = [
			...fetchLog.slice(-4),
			{ serverTs, clientTs: Math.floor(Date.now() / 1000), cached: prev?.serverTs === serverTs }
		];
	}

	const cachedTimeStale = createQuery(rpc, 'cached_time_stale', {
		onSuccess: (d) => logFetch(d.timestamp)
	});

	async function refetch() {
		await cachedTimeStale.refetch();
	}
</script>

<svelte:head>
	<title>stale — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="stale" mono>
		Adds <Code>stale-while-revalidate</Code>
		to the <Code>Cache-Control</Code>
		header. The CDN serves stale content while fetching a fresh response in the background. Requires
		<Code>cache</Code> — compilation error
		without it. Queries only.
	</PageHeader>

	<SectionHeading>Basic Usage</SectionHeading>
	<CodeBlock html={data.highlighted['basic']} />

	<SectionHeading>With Private Cache</SectionHeading>
	<CodeBlock html={data.highlighted['private']} />

	<SectionHeading>Generated Headers</SectionHeading>
	<CodeBlock html={data.highlighted['headers']} />

	<SectionHeading>Requires cache</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Using <Code>stale</Code> without
		<Code>cache</Code> is a compilation error.
	</p>
	<CodeBlock html={data.highlighted['error']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<DemoCard title="Stale-While-Revalidate — 10s + 30s">
		<p class="text-text-muted text-xs mb-3 font-mono">
			Cache-Control: public, max-age=0, s-maxage=10, stale-while-revalidate=30
		</p>
		<p class="text-text-muted text-sm mb-3">
			Cached for 10s, then serves stale for up to 30s while revalidating in the background.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<Button onclick={refetch} disabled={cachedTimeStale.isLoading}>Refetch</Button>
		</div>
		{#if fetchLog.length > 0}
			<OutputBox mono>
				{#each fetchLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>server: <span class="text-accent-rust">{entry.serverTs}</span></span
						>
						<span class="text-text-muted"
							>client: <span class="text-text-primary">{entry.clientTs}</span></span
						>
						{#if entry.cached}
							<span class="text-green-400">cached</span>
						{/if}
					</div>
				{/each}
			</OutputBox>
		{/if}
		<CollapsibleCode html={data.highlighted['cachedTimeStaleRust']} />
	</DemoCard>
</div>
