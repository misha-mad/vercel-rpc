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

	function hasLoss(val: number | bigint, exact: string): boolean {
		return String(val) !== exact;
	}
</script>

<svelte:head>
	<title>BigInt Mapping — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="BigInt Mapping">
		By default all integer types map to
		<Code>number</Code>. Use
		<Code>bigint_types</Code> to map
		large integer types to
		<Code>bigint</Code> instead.
	</PageHeader>

	<div class="space-y-3">
		<CodeBlock html={data.highlighted['configToml']} />
		<p class="text-text-faint text-xs">or via CLI</p>
		<CodeBlock html={data.highlighted['configCli']} />
	</div>

	<SectionHeading>Before / After</SectionHeading>
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['defaultTs']} />
		<CodeBlock html={data.highlighted['bigintTs']} />
	</div>

	<SectionHeading>Custom deserializer</SectionHeading>
	<p class="text-text-muted leading-relaxed text-sm">
		The generated client accepts a
		<Code>deserialize</Code> option. Plug
		in a BigInt-aware JSON parser so large numbers arrive as native
		<Code>BigInt</Code> at runtime:
	</p>
	<CodeBlock html={data.highlighted['losslessClient']} />

	<SectionHeading>Why BigInt?</SectionHeading>
	<p class="text-text-muted leading-relaxed text-sm">
		JavaScript <Code>number</Code> is a
		64-bit float (IEEE 754), which can only safely represent integers up to
		<Code>2<sup>53</sup> − 1</Code>
		(<Code>Number.MAX_SAFE_INTEGER</Code>
		= 9,007,199,254,740,991). Rust's
		<Code>i64</Code>/<Code>u64</Code>
		can exceed this range, causing silent precision loss. Using
		<Code>bigint</Code> avoids this entirely.
	</p>

	<p class="text-text-muted leading-relaxed text-sm">
		Commonly mapped types:
		<Code>i64</Code>,
		<Code>u64</Code>,
		<Code>i128</Code>,
		<Code>u128</Code>. Smaller types like
		<Code>i32</Code>
		and
		<Code>u32</Code> are safe as
		<Code>number</Code> and rarely need mapping.
	</p>

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<p class="text-text-muted text-sm">
		Both clients call the same endpoint. The default client uses
		<Code>JSON.parse</Code> which
		silently loses precision on large integers. The lossless client plugs
		<Code>lossless-json</Code> into the
		<Code>deserialize</Code> option —
		large numbers arrive as native
		<Code>BigInt</Code>.
	</p>

	<DemoCard title="JSON.parse vs lossless-json">
		<div class="flex items-center gap-3 mb-4">
			<Button onclick={fetchDemo} disabled={loading}>Fetch u64 values</Button>
			{#if loading}
				<span class="text-sm text-text-muted">Loading...</span>
			{/if}
		</div>

		{#if defaultResult && losslessResult}
			<div class="overflow-x-auto rounded-md border border-border">
				<table class="w-full text-xs font-mono">
					<thead class="bg-bg-code text-text-faint">
						<tr>
							<th class="px-1.5 py-2 text-left">Label</th>
							<th class="px-1.5 py-2 text-left">Server (exact)</th>
							<th class="px-1.5 py-2 text-left">
								<code>JSON.parse</code>
							</th>
							<th class="px-1.5 py-2 text-left"></th>
							<th class="px-1.5 py-2 text-left">
								<code>lossless-json</code>
							</th>
							<th class="px-1.5 py-2 text-left"></th>
						</tr>
					</thead>
					<tbody>
						{#each defaultResult.values as row, i (i)}
							{@const lost = hasLoss(row.as_number, row.exact)}
							{@const losslessVal = losslessResult[i]?.as_number}
							{@const losslessStr = String(losslessVal)}
							{@const losslessLost = losslessStr !== row.exact}
							<tr class="border-t border-border/50">
								<td class="px-1.5 py-2 text-text-muted">{row.label}</td>
								<td class="px-1.5 py-2 text-accent-rust">{row.exact}</td>
								<td class="px-1.5 py-2 text-accent-ts">{row.as_number}</td>
								<td class="px-1.5 py-2"
									>{#if lost}<span class="text-red-400">lost</span>{:else}<span
											class="text-green-400">ok</span
										>{/if}</td
								>
								<td class="px-1.5 py-2 text-accent-ts">{losslessStr}</td>
								<td class="px-1.5 py-2"
									>{#if losslessLost}<span class="text-red-400">lost</span>{:else}<span
											class="text-green-400">ok</span
										>{/if}</td
								>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			<div class="mt-3 space-y-1">
				<p class="text-text-faint text-xs">
					<code class="bg-bg-code px-1 py-0.5 rounded">typeof</code> of
					<code class="bg-bg-code px-1 py-0.5 rounded">as_number</code>: JSON.parse &rarr;
					<code class="bg-bg-code px-1 py-0.5 rounded"
						>{typeof defaultResult.values[0]?.as_number}</code
					>, lossless-json &rarr;
					<code class="bg-bg-code px-1 py-0.5 rounded">{typeof losslessResult[0]?.as_number}</code>
					/
					<code class="bg-bg-code px-1 py-0.5 rounded"
						>{typeof losslessResult[losslessResult.length - 1]?.as_number}</code
					>
				</p>
				<p class="text-text-faint text-xs">
					Plug any BigInt-aware parser into the client's
					<code class="bg-bg-code px-1 py-0.5 rounded">deserialize</code> option and
					<code class="bg-bg-code px-1 py-0.5 rounded">bigint_types</code> fields carry exact values at
					runtime.
				</p>
			</div>
		{/if}

		<CollapsibleCode html={data.highlighted['bigintDemoRust']} />
	</DemoCard>
</div>
