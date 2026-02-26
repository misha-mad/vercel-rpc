<svelte:head>
	<title>Macro Attributes — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Macro Attributes</h1>
	<p class="text-text-muted leading-relaxed">
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> and
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code> accept optional
		attributes that control caching, initialization, timeouts, and retry behavior.
	</p>

	<div class="overflow-x-auto rounded-lg border border-border">
		<table class="w-full text-sm text-left">
			<thead class="bg-bg-code text-text-muted text-xs uppercase">
				<tr>
					<th class="px-4 py-3">Attribute</th>
					<th class="px-4 py-3">#[rpc_query]</th>
					<th class="px-4 py-3">#[rpc_mutation]</th>
					<th class="px-4 py-3">Description</th>
				</tr>
			</thead>
			<tbody class="text-text-primary">
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><a href="/docs/macros/cache" class="text-accent-ts hover:underline font-mono text-xs"
							>cache</a
						></td
					>
					<td class="px-4 py-2 text-accent-ts">yes</td>
					<td class="px-4 py-2 text-red-400">no</td>
					<td class="px-4 py-2 text-text-muted text-xs">Cache-Control headers on responses</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><a href="/docs/macros/stale" class="text-accent-ts hover:underline font-mono text-xs"
							>stale</a
						></td
					>
					<td class="px-4 py-2 text-accent-ts">yes (requires cache)</td>
					<td class="px-4 py-2 text-red-400">no</td>
					<td class="px-4 py-2 text-text-muted text-xs">stale-while-revalidate</td>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><a href="/docs/macros/init" class="text-accent-ts hover:underline font-mono text-xs"
							>init</a
						></td
					>
					<td class="px-4 py-2 text-accent-ts">yes</td>
					<td class="px-4 py-2 text-accent-ts">yes</td>
					<td class="px-4 py-2 text-text-muted text-xs">Cold-start initialization + shared state</td
					>
				</tr>
				<tr class="border-b border-border">
					<td class="px-4 py-2"
						><a href="/docs/macros/timeout" class="text-accent-ts hover:underline font-mono text-xs"
							>timeout</a
						></td
					>
					<td class="px-4 py-2 text-accent-ts">yes</td>
					<td class="px-4 py-2 text-accent-ts">yes</td>
					<td class="px-4 py-2 text-text-muted text-xs">Server-side timeout (504 on exceed)</td>
				</tr>
				<tr>
					<td class="px-4 py-2"
						><a
							href="/docs/macros/idempotent"
							class="text-accent-ts hover:underline font-mono text-xs">idempotent</a
						></td
					>
					<td class="px-4 py-2 text-red-400">no</td>
					<td class="px-4 py-2 text-accent-ts">yes</td>
					<td class="px-4 py-2 text-text-muted text-xs">Opt mutation into retry</td>
				</tr>
			</tbody>
		</table>
	</div>
</div>
