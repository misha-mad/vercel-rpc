<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import Code from '$lib/components/Code.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

	let { data } = $props();

	// --- Math (reactive query with struct input, Result<T, E>) ---
	let mathA = $state(10);
	let mathB = $state(3);
	let mathOp = $state<'Add' | 'Subtract' | 'Multiply' | 'Divide'>('Add');
	const math = createQuery(rpc, 'math', () => ({ a: mathA, b: mathB, op: mathOp }));
</script>

<svelte:head>
	<title>Queries — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Queries">
		Use <Code>#[rpc_query]</Code> for
		read-only operations. Wrap them with
		<Code>createQuery</Code> in Svelte for
		reactive, auto-refetching data.
	</PageHeader>

	<!-- Code examples -->
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
		<CodeBlock html={data.highlighted['exampleSvelte']} />
	</div>

	<!-- Per-Call Options -->
	<SectionHeading>Per-Call Options</SectionHeading>
	<p class="text-text-muted text-sm">
		Every <Code>query()</Code> call
		accepts an optional trailing
		<Code>CallOptions</Code> object to override
		client-level defaults.
	</p>
	<CodeBlock html={data.highlighted['callOptionsType']} />
	<CodeBlock html={data.highlighted['callOptionsUsage']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>

	<!-- Math: Struct input, Result<T, E> -->
	<DemoCard title="Math — Struct Input, Result<T, E>">
		<p class="text-text-muted text-sm mb-4">
			Demonstrates struct input and <Code>Result&lt;T, E&gt;</Code> error handling.
		</p>
		<div class="flex flex-wrap items-center gap-3 mb-3">
			<input
				type="number"
				bind:value={mathA}
				class="w-20 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts"
			/>
			<select
				bind:value={mathOp}
				class="rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts"
			>
				<option>Add</option><option>Subtract</option><option>Multiply</option><option>Divide</option
				>
			</select>
			<input
				type="number"
				bind:value={mathB}
				class="w-20 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts"
			/>
		</div>
		{#if math.data !== undefined}
			<OutputBox>
				<div>
					<span class="text-text-muted">Expression:</span>
					<span class="text-text-primary">{math.data.expression}</span>
				</div>
				<div>
					<span class="text-text-muted">Result:</span>
					<span class="text-green-400">{math.data.result}</span>
				</div>
			</OutputBox>
		{/if}
		{#if math.isError}
			<OutputBox status="error">{math.error?.message}</OutputBox>
		{/if}
	</DemoCard>
</div>
