<svelte:head>
	<title>CLI — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">CLI</h1>
	<p class="text-text-muted leading-relaxed">
		The <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">rpc</code> CLI scans Rust source files, parses types via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code>, and generates TypeScript types and a typed client.
	</p>

	<!-- Commands -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">Commands</h2>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold font-mono">rpc scan</h3>
			<p class="text-text-muted text-sm">Parse Rust files and print discovered procedures, structs, and enums. Useful for debugging.</p>
		</div>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold font-mono">rpc generate</h3>
			<p class="text-text-muted text-sm">Generate TypeScript types and client. One-shot execution.</p>
		</div>

		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-2">
			<h3 class="text-lg font-semibold font-mono">rpc watch</h3>
			<p class="text-text-muted text-sm">Same as <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">generate</code> but watches for <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.rs</code> file changes and regenerates automatically.</p>
		</div>
	</section>

	<!-- Flags -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">Flags</h2>

		<div class="overflow-x-auto rounded-lg border border-border">
			<table class="w-full text-sm text-left">
				<thead class="bg-bg-code text-text-muted text-xs uppercase">
					<tr>
						<th class="px-4 py-3">Flag</th>
						<th class="px-4 py-3">Default</th>
						<th class="px-4 py-3">Description</th>
					</tr>
				</thead>
				<tbody class="text-text-primary">
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>-d, --dir</code></td>
						<td class="px-4 py-2 text-text-muted">api</td>
						<td class="px-4 py-2 text-text-muted">Rust source directory</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>-o, --output</code></td>
						<td class="px-4 py-2 text-text-muted">src/lib/rpc-types.ts</td>
						<td class="px-4 py-2 text-text-muted">Types file path</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>-c, --client-output</code></td>
						<td class="px-4 py-2 text-text-muted">src/lib/rpc-client.ts</td>
						<td class="px-4 py-2 text-text-muted">Client file path</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--svelte-output</code></td>
						<td class="px-4 py-2 text-text-muted">—</td>
						<td class="px-4 py-2 text-text-muted">Svelte 5 wrapper (opt-in)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--react-output</code></td>
						<td class="px-4 py-2 text-text-muted">—</td>
						<td class="px-4 py-2 text-text-muted">React hooks (opt-in)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--vue-output</code></td>
						<td class="px-4 py-2 text-text-muted">—</td>
						<td class="px-4 py-2 text-text-muted">Vue 3 composables (opt-in)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--solid-output</code></td>
						<td class="px-4 py-2 text-text-muted">—</td>
						<td class="px-4 py-2 text-text-muted">SolidJS primitives (opt-in)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--types-import</code></td>
						<td class="px-4 py-2 text-text-muted">./rpc-types</td>
						<td class="px-4 py-2 text-text-muted">Import path for types in client</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--preserve-docs</code></td>
						<td class="px-4 py-2 text-text-muted">false</td>
						<td class="px-4 py-2 text-text-muted">Forward Rust <code class="bg-bg-code px-1 py-0.5 rounded text-xs">///</code> as JSDoc</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--fields</code></td>
						<td class="px-4 py-2 text-text-muted">preserve</td>
						<td class="px-4 py-2 text-text-muted"><code class="bg-bg-code px-1 py-0.5 rounded text-xs">preserve</code> or <code class="bg-bg-code px-1 py-0.5 rounded text-xs">camelCase</code></td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--branded-newtypes</code></td>
						<td class="px-4 py-2 text-text-muted">false</td>
						<td class="px-4 py-2 text-text-muted">Enable branded types for newtypes</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--type-override</code></td>
						<td class="px-4 py-2 text-text-muted">—</td>
						<td class="px-4 py-2 text-text-muted">Map Rust type to TS (repeatable)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--bigint-type</code></td>
						<td class="px-4 py-2 text-text-muted">—</td>
						<td class="px-4 py-2 text-text-muted">Map integer type to bigint (repeatable)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2 font-mono text-xs"><code>--config</code></td>
						<td class="px-4 py-2 text-text-muted">auto</td>
						<td class="px-4 py-2 text-text-muted">Explicit config file path</td>
					</tr>
					<tr>
						<td class="px-4 py-2 font-mono text-xs"><code>--no-config</code></td>
						<td class="px-4 py-2 text-text-muted">false</td>
						<td class="px-4 py-2 text-text-muted">Disable config file loading</td>
					</tr>
				</tbody>
			</table>
		</div>
	</section>
</div>
