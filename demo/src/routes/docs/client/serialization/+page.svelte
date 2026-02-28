<script lang="ts">
	import { rpc } from '$lib/client';
	import { type BigIntDemoResponse, type BigIntDemoValue, createRpcClient } from '$lib/rpc-client';
	import { parse, parseNumberAndBigInt } from 'lossless-json';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';

	let { data } = $props();

	const losslessRpc = createRpcClient({
		baseUrl: '/api',
		deserialize: (text) => parse(text, undefined, parseNumberAndBigInt)
	});

	let defaultResult: BigIntDemoResponse | undefined = $state();
	let losslessResult: BigIntDemoValue[] | undefined = $state();
	let loading = $state(false);

	async function fetchDemo() {
		loading = true;
		try {
			const [def, lossless] = await Promise.all([
				rpc.query('bigint_demo'),
				losslessRpc.query('bigint_demo')
			]);
			defaultResult = def;
			losslessResult = lossless.values;
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Serialization — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Serialization">
		Override <Code>serialize</Code> and
		<Code>deserialize</Code> to use a custom serialization format instead of plain JSON.
	</PageHeader>

	<SectionHeading>Default</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		When not specified, the client uses <Code>JSON.stringify</Code>
		and <Code>JSON.parse</Code>.
	</p>
	<CodeBlock html={data.highlighted['defaultSerialization']} />

	<SectionHeading>Custom (superjson)</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Use a library like <Code>superjson</Code>
		to support <Code>Date</Code>,
		<Code>BigInt</Code>,
		<Code>Map</Code>,
		<Code>Set</Code> and other types not natively supported by JSON.
	</p>
	<CodeBlock html={data.highlighted['customSerialization']} />

	<SectionHeading>Type Signature</SectionHeading>
	<CodeBlock html={data.highlighted['signature']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<p class="text-text-muted text-sm">
		Same endpoint, two clients. The default uses <Code>JSON.parse</Code>, the custom one uses
		<Code>lossless-json</Code>
		via the <Code>deserialize</Code> option.
	</p>

	<DemoCard>
		<div class="flex items-center gap-3 mb-4">
			<Button onclick={fetchDemo} disabled={loading}>Fetch with both clients</Button>
			{#if loading}
				<span class="text-sm text-text-muted">Loading...</span>
			{/if}
		</div>

		{#if defaultResult && losslessResult}
			<div class="overflow-x-auto rounded-md border border-border">
				<table class="w-full text-xs font-mono">
					<thead class="bg-bg-code text-text-faint">
						<tr>
							<th class="px-3 py-2 text-left">Label</th>
							<th class="px-3 py-2 text-left">Server (exact)</th>
							<th class="px-3 py-2 text-left">JSON.parse</th>
							<th class="px-3 py-2 text-left">typeof</th>
							<th class="px-3 py-2 text-left">lossless-json</th>
							<th class="px-3 py-2 text-left">typeof</th>
						</tr>
					</thead>
					<tbody>
						{#each defaultResult.values as row, i (i)}
							{@const losslessVal = losslessResult[i]?.as_number}
							{@const defaultLost = String(row.as_number) !== row.exact}
							{@const losslessLost = String(losslessVal) !== row.exact}
							<tr class="border-t border-border/50">
								<td class="px-3 py-2 text-text-muted">{row.label}</td>
								<td class="px-3 py-2 text-accent-rust">{row.exact}</td>
								<td
									class="px-3 py-2"
									class:text-red-400={defaultLost}
									class:text-green-400={!defaultLost}>{row.as_number}</td
								>
								<td class="px-3 py-2 text-text-faint">{typeof row.as_number}</td>
								<td
									class="px-3 py-2"
									class:text-red-400={losslessLost}
									class:text-green-400={!losslessLost}>{String(losslessVal)}</td
								>
								<td class="px-3 py-2 text-text-faint">{typeof losslessVal}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		{/if}

		<CollapsibleCode html={data.highlighted['bigintDemoRust']} />
	</DemoCard>
</div>
