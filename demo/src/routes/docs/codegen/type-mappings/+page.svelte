<script lang="ts">
	import { rpc } from '$lib/client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import Code from '$lib/components/Code.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import Button from '$lib/components/Button.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';

	let { data } = $props();

	let result: import('$lib/rpc-types').TypeShowcase | undefined = $state();
	let loading = $state(false);

	async function fetchTypes() {
		loading = true;
		try {
			result = await rpc.query('types');
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Type Mappings — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Type Mappings">
		Every Rust type is automatically mapped to its TypeScript equivalent during code generation.
		Here's the complete reference.
	</PageHeader>

	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm text-left">
			<thead class="bg-bg-code text-text-muted text-xs uppercase">
				<tr>
					<th class="px-4 py-3">Rust</th>
					<th class="px-4 py-3">TypeScript</th>
					<th class="px-4 py-3">Notes</th>
				</tr>
			</thead>
			<tbody class="text-text-primary">
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>String</Code>,
						<Code>&amp;str</Code>,
						<Code>char</Code></td
					>
					<td class="px-4 py-2"
						><Code>string</Code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>i8</Code>–<Code>i128</Code>, <Code>u8</Code>–<Code>u128</Code>, <Code>f32</Code>,
						<Code>f64</Code></td
					>
					<td class="px-4 py-2"
						><Code>number</Code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>or <Code>bigint</Code> via
						<a href="/docs/codegen/bigint" class="text-accent-ts hover:underline"
							><Code>bigint_types</Code></a
						></td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>bool</Code></td
					>
					<td class="px-4 py-2"
						><Code>boolean</Code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>()</Code></td
					>
					<td class="px-4 py-2"
						><Code>void</Code></td
					>
					<td class="px-4 py-2 text-text-muted">no-input procedures</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>Vec&lt;T&gt;</Code>,
						<Code>HashSet&lt;T&gt;</Code>,
						<Code>BTreeSet&lt;T&gt;</Code></td
					>
					<td class="px-4 py-2"
						><Code>T[]</Code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>also <Code>Array&lt;T&gt;</Code> in generic positions</td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>Option&lt;T&gt;</Code></td
					>
					<td class="px-4 py-2"
						><Code>T | null</Code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>with <Code>serde(default)</Code>:
						<Code>field?: T | null</Code></td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>HashMap&lt;K, V&gt;</Code>,
						<Code>BTreeMap&lt;K, V&gt;</Code></td
					>
					<td class="px-4 py-2"
						><Code>Record&lt;K, V&gt;</Code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>Box&lt;T&gt;</Code>,
						<Code>Arc&lt;T&gt;</Code>,
						<Code>Rc&lt;T&gt;</Code>,
						<Code>Cow&lt;T&gt;</Code></td
					>
					<td class="px-4 py-2"
						><Code>T</Code></td
					>
					<td class="px-4 py-2 text-text-muted">transparent unwrap</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>(A, B, C)</Code></td
					>
					<td class="px-4 py-2"
						><Code>[A, B, C]</Code></td
					>
					<td class="px-4 py-2 text-text-muted">tuples</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>[T; N]</Code></td
					>
					<td class="px-4 py-2"
						><Code>T[]</Code></td
					>
					<td class="px-4 py-2 text-text-muted">fixed-size arrays</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>Result&lt;T, E&gt;</Code></td
					>
					<td class="px-4 py-2"
						><Code>T</Code></td
					>
					<td class="px-4 py-2 text-text-muted"
						><Code>Ok(T)</Code>
						unwrapped;
						<Code>Err(E)</Code>
						serialized as JSON and thrown as
						<a href="/docs/error-handling" class="text-accent-ts hover:underline"
							><Code>RpcError</Code></a
						> (status 500)</td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>struct</Code></td
					>
					<td class="px-4 py-2"
						><Code>interface</Code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>enum</Code> (unit variants)</td
					>
					<td class="px-4 py-2"
						><Code>"A" | "B"</Code></td
					>
					<td class="px-4 py-2 text-text-muted">string union</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><Code>enum</Code> (data variants)</td
					>
					<td class="px-4 py-2"
						><Code>{'{ A: T }'} | ...</Code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>see <a href="/docs/codegen/serde" class="text-accent-ts hover:underline"
							>Serde Support</a
						> for tagging</td
					>
				</tr>
				<tr>
					<td class="px-4 py-2"
						>Newtype <Code>struct Id(String)</Code></td
					>
					<td class="px-4 py-2"
						><Code>type Id = string</Code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>or branded with <a
							href="/docs/codegen/branded-newtypes"
							class="text-accent-ts hover:underline"
							><Code>branded_newtypes</Code></a
						></td
					>
				</tr>
			</tbody>
		</table>
	</div>

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<p class="text-text-muted text-sm">
		A single struct with every mapping from the table above. Click Fetch to see live values.
	</p>

	<DemoCard>
		<div class="flex items-center gap-3 mb-4">
			<Button onclick={fetchTypes} disabled={loading}>Fetch</Button>
			{#if loading}
				<span class="text-sm text-text-muted">Loading...</span>
			{/if}
		</div>

		{#if result}
			<div class="overflow-x-auto rounded-md border border-border">
				<table class="w-full text-xs font-mono">
					<thead class="bg-bg-code text-text-faint">
						<tr>
							<th class="px-3 py-2 text-left">Rust type</th>
							<th class="px-3 py-2 text-left">TS type</th>
							<th class="px-3 py-2 text-left">Value</th>
						</tr>
					</thead>
					<tbody>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">String</td>
							<td class="px-3 py-2 text-text-muted">string</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.string_val)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">i32</td>
							<td class="px-3 py-2 text-text-muted">number</td>
							<td class="px-3 py-2 text-accent-ts">{result.integer}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">f64</td>
							<td class="px-3 py-2 text-text-muted">number</td>
							<td class="px-3 py-2 text-accent-ts">{result.float}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">bool</td>
							<td class="px-3 py-2 text-text-muted">boolean</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.flag)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">Vec&lt;String&gt;</td>
							<td class="px-3 py-2 text-text-muted">string[]</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.vec_items)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">HashSet&lt;String&gt;</td>
							<td class="px-3 py-2 text-text-muted">string[]</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.hash_set)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">BTreeSet&lt;i32&gt;</td>
							<td class="px-3 py-2 text-text-muted">number[]</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.btree_set)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">Option&lt;String&gt;</td>
							<td class="px-3 py-2 text-text-muted">string | null</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.optional_present)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">Option&lt;String&gt;</td>
							<td class="px-3 py-2 text-text-muted">string | null</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.optional_absent)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">HashMap&lt;String, i32&gt;</td>
							<td class="px-3 py-2 text-text-muted">Record&lt;string, number&gt;</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.hash_map)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">BTreeMap&lt;String, i32&gt;</td>
							<td class="px-3 py-2 text-text-muted">Record&lt;string, number&gt;</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.btree_map)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">Box&lt;String&gt;</td>
							<td class="px-3 py-2 text-text-muted">string</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.boxed)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">Cow&lt;str&gt;</td>
							<td class="px-3 py-2 text-text-muted">string</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.cow)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">(String, i32, bool)</td>
							<td class="px-3 py-2 text-text-muted">[string, number, boolean]</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.tuple)}</td>
						</tr>
						<tr class="border-t border-border/50">
							<td class="px-3 py-2 text-text-muted">[i32; 3]</td>
							<td class="px-3 py-2 text-text-muted">number[]</td>
							<td class="px-3 py-2 text-accent-ts">{JSON.stringify(result.fixed_array)}</td>
						</tr>
					</tbody>
				</table>
			</div>
		{/if}

		<CollapsibleCode html={data.highlighted['typesRust']} />
	</DemoCard>
</div>
