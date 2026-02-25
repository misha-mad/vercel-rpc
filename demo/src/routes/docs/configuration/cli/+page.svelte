<svelte:head>
	<title>CLI — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">CLI</h1>
	<p class="text-text-muted leading-relaxed">
		The <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">rpc</code> CLI scans Rust
		source files, parses types via
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code>, and generates
		TypeScript types and a typed client.
	</p>

	<!-- Commands -->
	<section class="space-y-6">
		<h2 class="text-2xl font-semibold">Commands</h2>

		<!-- rpc scan -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold font-mono">rpc scan</h3>
			<p class="text-text-muted text-sm">
				Parse Rust files and print discovered procedures, structs, and enums. Useful for debugging.
			</p>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Flag</th><th class="pb-2 pr-4 font-medium">Default</th
						><th class="pb-2 font-medium">Description</th>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>-d, --dir</code></td><td
							class="py-1.5 pr-4">api</td
						><td class="py-1.5">Rust source directory</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--include</code></td><td
							class="py-1.5 pr-4">**/*.rs</td
						><td class="py-1.5">Glob patterns to include (repeatable)</td></tr
					>
					<tr
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--exclude</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">Glob patterns to exclude (repeatable)</td></tr
					>
				</tbody>
			</table>
		</div>

		<!-- rpc generate -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold font-mono">rpc generate</h3>
			<p class="text-text-muted text-sm">
				Generate TypeScript types and client. One-shot execution.
			</p>
			<h4 class="text-sm font-semibold text-text-muted pt-1">Input</h4>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Flag</th><th class="pb-2 pr-4 font-medium">Default</th
						><th class="pb-2 font-medium">Description</th>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>-d, --dir</code></td><td
							class="py-1.5 pr-4">api</td
						><td class="py-1.5">Rust source directory</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--include</code></td><td
							class="py-1.5 pr-4">**/*.rs</td
						><td class="py-1.5">Glob patterns to include (repeatable)</td></tr
					>
					<tr
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--exclude</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">Glob patterns to exclude (repeatable)</td></tr
					>
				</tbody>
			</table>
			<h4 class="text-sm font-semibold text-text-muted pt-1">Output</h4>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Flag</th><th class="pb-2 pr-4 font-medium">Default</th
						><th class="pb-2 font-medium">Description</th>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>-o, --output</code></td><td
							class="py-1.5 pr-4">src/lib/rpc-types.ts</td
						><td class="py-1.5">Types file path</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>-c, --client-output</code></td><td
							class="py-1.5 pr-4">src/lib/rpc-client.ts</td
						><td class="py-1.5">Client file path</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--svelte-output</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">Svelte 5 wrapper (opt-in)</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--react-output</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">React hooks (opt-in)</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--vue-output</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">Vue 3 composables (opt-in)</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--solid-output</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">SolidJS primitives (opt-in)</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--types-import</code></td><td
							class="py-1.5 pr-4">./rpc-types</td
						><td class="py-1.5">Import path for types in client</td></tr
					>
					<tr
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--extension</code></td><td
							class="py-1.5 pr-4">""</td
						><td class="py-1.5"
							>Import extension, e.g. <code class="text-xs font-mono">".js"</code> for ESM</td
						></tr
					>
				</tbody>
			</table>
			<h4 class="text-sm font-semibold text-text-muted pt-1">Codegen</h4>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Flag</th><th class="pb-2 pr-4 font-medium">Default</th
						><th class="pb-2 font-medium">Description</th>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--preserve-docs</code></td><td
							class="py-1.5 pr-4">false</td
						><td class="py-1.5">Forward Rust <code class="text-xs font-mono">///</code> as JSDoc</td
						></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--branded-newtypes</code></td><td
							class="py-1.5 pr-4">false</td
						><td class="py-1.5">Emit branded types for newtypes</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--fields</code></td><td
							class="py-1.5 pr-4">preserve</td
						><td class="py-1.5"
							><code class="text-xs font-mono">preserve</code> or
							<code class="text-xs font-mono">camelCase</code></td
						></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--type-override</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5">Map Rust type to TS (repeatable)</td></tr
					>
					<tr
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--bigint-type</code></td><td
							class="py-1.5 pr-4">—</td
						><td class="py-1.5"
							>Map integer type to <code class="text-xs font-mono">bigint</code> (repeatable)</td
						></tr
					>
				</tbody>
			</table>
		</div>

		<!-- rpc watch -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold font-mono">rpc watch</h3>
			<p class="text-text-muted text-sm">
				Same as <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">generate</code> but
				watches for <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.rs</code> file
				changes and regenerates automatically. Accepts all
				<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">generate</code> flags plus:
			</p>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Flag</th><th class="pb-2 pr-4 font-medium">Default</th
						><th class="pb-2 font-medium">Description</th>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--debounce-ms</code></td><td
							class="py-1.5 pr-4">200</td
						><td class="py-1.5">Milliseconds to wait after file change before regenerating</td></tr
					>
					<tr
						><td class="py-1.5 pr-4 font-mono text-xs"><code>--clear-screen</code></td><td
							class="py-1.5 pr-4">false</td
						><td class="py-1.5">Clear terminal before each regeneration</td></tr
					>
				</tbody>
			</table>
		</div>
	</section>

	<!-- Global Flags -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">Global Flags</h2>
		<p class="text-text-muted text-sm">Available on all commands.</p>
		<div class="overflow-x-auto rounded-lg border border-border">
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border bg-bg-code">
						<th class="px-4 py-2 font-medium">Flag</th><th class="px-4 py-2 font-medium">Default</th
						><th class="px-4 py-2 font-medium">Description</th>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="px-4 py-1.5 font-mono text-xs"><code>--config</code></td><td
							class="px-4 py-1.5">auto</td
						><td class="px-4 py-1.5">Explicit config file path</td></tr
					>
					<tr
						><td class="px-4 py-1.5 font-mono text-xs"><code>--no-config</code></td><td
							class="px-4 py-1.5">false</td
						><td class="px-4 py-1.5">Disable config file loading</td></tr
					>
				</tbody>
			</table>
		</div>
	</section>
</div>
