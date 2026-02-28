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

	let callLog: {
		coldStartAt: number;
		initMs: number;
		reqCount: number;
		now: number;
		sameInstance: boolean | null;
	}[] = $state([]);

	const initDemo = createQuery(rpc, 'init_demo', {
		onSuccess: (d) => {
			const prev = callLog.at(-1);
			callLog = [
				...callLog.slice(-4),
				{
					coldStartAt: d.cold_start_at,
					initMs: d.init_duration_ms,
					reqCount: d.request_count,
					now: d.now,
					sameInstance: prev ? prev.coldStartAt === d.cold_start_at : null
				}
			];
		}
	});

	async function refetch() {
		await initDemo.refetch();
	}
</script>

<svelte:head>
	<title>init — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="init" mono>
		Run a function once at cold start. Can be side-effects only (logger, dotenv) or return shared
		state (DB pool, HTTP client) injected as
		<Code>&T</Code> parameter. Works with both
		queries and mutations.
	</PageHeader>

	<SectionHeading>Side-Effects Only</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		When the init function returns nothing, it runs once for setup (logging, env vars, tracing).
	</p>
	<CodeBlock html={data.highlighted['sideEffect']} />

	<SectionHeading>Shared State</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		When the init function returns a value, it's stored and injected as
		<Code>&T</Code> into the handler. The init
		function runs once per cold start — the result is reused across requests.
	</p>
	<CodeBlock html={data.highlighted['sharedState']} />

	<SectionHeading>HTTP Client</SectionHeading>
	<CodeBlock html={data.highlighted['httpClient']} />

	<SectionHeading>Combining Attributes</SectionHeading>
	<CodeBlock html={data.highlighted['combined']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<DemoCard title="Cold Start &amp; Shared State">
		<p class="text-text-muted text-sm mb-3">
			The <Code>setup()</Code> function runs
			once at cold start, measuring its own duration. Subsequent requests reuse the same state — watch
			the request count increment while cold start time stays the same.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<Button onclick={refetch} disabled={initDemo.isLoading}>Refetch</Button>
		</div>
		{#if callLog.length > 0}
			<OutputBox mono>
				{#each callLog as entry, i (i)}
					<div class="flex gap-4">
						<span class="text-text-faint">#{i + 1}</span>
						<span class="text-text-muted"
							>init: <span class="text-accent-rust">{entry.initMs}ms</span></span
						>
						<span class="text-text-muted"
							>req: <span class="text-text-primary">{entry.reqCount}</span></span
						>
						{#if entry.sameInstance === true}
							<span class="text-green-400">same instance</span>
						{:else if entry.sameInstance === false}
							<span class="text-yellow-400">new cold start</span>
						{/if}
					</div>
				{/each}
			</OutputBox>
		{/if}
		<CollapsibleCode html={data.highlighted['initDemoRust']} />
	</DemoCard>
</div>
