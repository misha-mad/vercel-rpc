<svelte:head>
	<title>Type Mappings — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Type Mappings</h1>
	<p class="text-text-muted leading-relaxed">
		Every Rust type is automatically mapped to its TypeScript equivalent during code generation.
		Here's the complete reference.
	</p>

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
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">String</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&amp;str</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">char</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">string</code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i8</code>–<code
							class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i128</code
						>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u8</code>–<code
							class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u128</code
						>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">f32</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">f64</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>or <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint</code> via
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bigint_types</code></td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bool</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">boolean</code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">()</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">void</code></td
					>
					<td class="px-4 py-2 text-text-muted">no-input procedures</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Vec&lt;T&gt;</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">HashSet&lt;T&gt;</code
						>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">BTreeSet&lt;T&gt;</code
						></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T[]</code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Option&lt;T&gt;</code
						></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T | null</code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>serde(default)</code
						>:
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">field?: T | null</code
						></td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>HashMap&lt;K, V&gt;</code
						>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>BTreeMap&lt;K, V&gt;</code
						></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>Record&lt;K, V&gt;</code
						></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Box&lt;T&gt;</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Arc&lt;T&gt;</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Rc&lt;T&gt;</code>,
						<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cow&lt;T&gt;</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T</code></td
					>
					<td class="px-4 py-2 text-text-muted">transparent unwrap</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">(A, B, C)</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">[A, B, C]</code></td
					>
					<td class="px-4 py-2 text-text-muted">tuples</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">[T; N]</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T[]</code></td
					>
					<td class="px-4 py-2 text-text-muted">fixed-size arrays</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>Result&lt;T, E&gt;</code
						></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T</code></td
					>
					<td class="px-4 py-2 text-text-muted"
						>error handled at runtime via <code
							class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">RpcError</code
						></td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">struct</code></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">interface</code></td
					>
					<td class="px-4 py-2 text-text-muted"></td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">enum</code> (unit variants)</td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">"A" | "B"</code></td
					>
					<td class="px-4 py-2 text-text-muted">string union</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">enum</code> (data variants)</td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>{'{ A: T }'} | ...</code
						></td
					>
					<td class="px-4 py-2 text-text-muted">see Serde Support for tagging</td>
				</tr>
				<tr>
					<td class="px-4 py-2"
						>Newtype <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>struct Id(String)</code
						></td
					>
					<td class="px-4 py-2"
						><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">type Id = string</code
						></td
					>
					<td class="px-4 py-2 text-text-muted"
						>or branded with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
							>branded_newtypes</code
						></td
					>
				</tr>
			</tbody>
		</table>
	</div>
</div>
