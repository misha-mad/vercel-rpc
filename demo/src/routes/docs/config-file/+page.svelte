<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Config File — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Config File</h1>
	<p class="text-text-muted leading-relaxed">
		Place an <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>metaxy.config.toml</code
		>
		in your project root. The CLI auto-discovers it by walking up from the current directory. Use
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">--config &lt;path&gt;</code>
		to override or
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">--no-config</code> to disable.
	</p>

	<p class="text-text-muted text-sm">
		Resolution order (highest wins): CLI flag > config file > built-in default.
	</p>

	<CodeBlock html={data.highlighted['fullConfig']} large />

	<div class="space-y-6">
		<h2 class="text-2xl font-semibold">Sections</h2>

		<!-- [input] -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold">[input]</h3>
			<p class="text-text-muted text-sm">
				Where to find Rust source files. See <a
					href="/docs/cli/generate"
					class="text-accent-ts hover:underline">metaxy generate</a
				>
				and <a href="/docs/cli/scan" class="text-accent-ts hover:underline">metaxy scan</a> for CLI equivalents.
			</p>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Field</th><th class="pb-2 pr-4 font-medium">Type</th
						><th class="pb-2 pr-4 font-medium">Default</th><th class="pb-2 font-medium"
							>Description</th
						>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">dir</code></td
						><td class="py-1.5 pr-4">string</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">"api"</code></td
						><td class="py-1.5">Root directory with Rust source files</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">include</code></td
						><td class="py-1.5 pr-4">string[]</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">["**/*.rs"]</code></td
						><td class="py-1.5">Glob patterns to include</td></tr
					>
					<tr
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">exclude</code></td
						><td class="py-1.5 pr-4">string[]</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">[]</code></td
						><td class="py-1.5">Glob patterns to exclude</td></tr
					>
				</tbody>
			</table>
		</div>

		<!-- [output] -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold">[output]</h3>
			<p class="text-text-muted text-sm">
				Generated file paths. Framework wrappers are opt-in — set a path to enable. See <a
					href="/docs/cli/generate"
					class="text-accent-ts hover:underline">metaxy generate</a
				> for CLI equivalents.
			</p>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Field</th><th class="pb-2 pr-4 font-medium">Type</th
						><th class="pb-2 pr-4 font-medium">Default</th><th class="pb-2 font-medium"
							>Description</th
						>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">types</code></td
						><td class="py-1.5 pr-4">string</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">"src/lib/rpc-types.ts"</code></td
						><td class="py-1.5">Generated TypeScript types</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">client</code></td
						><td class="py-1.5 pr-4">string</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">"src/lib/rpc-client.ts"</code></td
						><td class="py-1.5">Generated RPC client</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">svelte</code></td
						><td class="py-1.5 pr-4">string?</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">—</code></td
						><td class="py-1.5">Svelte 5 reactive wrapper (opt-in)</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">react</code></td
						><td class="py-1.5 pr-4">string?</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">—</code></td
						><td class="py-1.5">React hooks wrapper (opt-in)</td></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">vue</code></td
						><td class="py-1.5 pr-4">string?</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">—</code></td
						><td class="py-1.5">Vue 3 composable wrapper (opt-in)</td></tr
					>
					<tr
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">solid</code></td
						><td class="py-1.5 pr-4">string?</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">—</code></td
						><td class="py-1.5">SolidJS wrapper (opt-in)</td></tr
					>
				</tbody>
			</table>

			<h4 class="text-base font-semibold pt-2">[output.imports]</h4>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Field</th><th class="pb-2 pr-4 font-medium">Type</th
						><th class="pb-2 pr-4 font-medium">Default</th><th class="pb-2 font-medium"
							>Description</th
						>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">types_path</code></td
						><td class="py-1.5 pr-4">string</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">"./rpc-types"</code></td
						><td class="py-1.5">Import path for the types module</td></tr
					>
					<tr
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">extension</code></td
						><td class="py-1.5 pr-4">string</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">""</code></td
						><td class="py-1.5"
							>File extension for imports, e.g. <code class="text-xs font-mono">".js"</code> for ESM</td
						></tr
					>
				</tbody>
			</table>
		</div>

		<!-- [codegen] -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold">[codegen]</h3>
			<p class="text-text-muted text-sm">
				Code generation options. See <a
					href="/docs/codegen/doc-comments"
					class="text-accent-ts hover:underline">Codegen</a
				> section for details on each option.
			</p>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Field</th><th class="pb-2 pr-4 font-medium">Type</th
						><th class="pb-2 pr-4 font-medium">Default</th><th class="pb-2 font-medium"
							>Description</th
						>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">preserve_docs</code
							></td
						><td class="py-1.5 pr-4">bool</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">false</code></td
						><td class="py-1.5"
							><a href="/docs/codegen/doc-comments" class="text-accent-ts hover:underline"
								>Forward Rust doc comments as JSDoc</a
							></td
						></tr
					>
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">branded_newtypes</code
							></td
						><td class="py-1.5 pr-4">bool</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">false</code></td
						><td class="py-1.5"
							><a href="/docs/codegen/branded-newtypes" class="text-accent-ts hover:underline"
								>Emit nominal (branded) types for Rust newtypes</a
							></td
						></tr
					>
					<tr
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">bigint_types</code
							></td
						><td class="py-1.5 pr-4">string[]</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">[]</code></td
						><td class="py-1.5"
							><a href="/docs/codegen/bigint" class="text-accent-ts hover:underline"
								>Rust types mapped to <code class="text-xs font-mono">bigint</code></a
							>, e.g.
							<code class="text-xs font-mono">["i64", "u64"]</code></td
						></tr
					>
				</tbody>
			</table>

			<h4 class="text-base font-semibold pt-2">[codegen.naming]</h4>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Field</th><th class="pb-2 pr-4 font-medium">Type</th
						><th class="pb-2 pr-4 font-medium">Default</th><th class="pb-2 font-medium"
							>Description</th
						>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">fields</code></td
						><td class="py-1.5 pr-4">string</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">"preserve"</code></td
						><td class="py-1.5"
							><a href="/docs/codegen/field-naming" class="text-accent-ts hover:underline"
								><code class="text-xs font-mono">"preserve"</code> keeps snake_case,
								<code class="text-xs font-mono">"camelCase"</code> converts</a
							></td
						></tr
					>
				</tbody>
			</table>

			<h4 class="text-base font-semibold pt-2">[codegen.type_overrides]</h4>
			<p class="text-text-muted text-sm">
				<a href="/docs/codegen/type-overrides" class="text-accent-ts hover:underline"
					>Map external crate types to TypeScript types.</a
				> Keys are fully-qualified Rust paths, values are TS type names.
			</p>
			<div class="text-text-muted text-sm font-mono bg-bg-code rounded-md px-3 py-2">
				"chrono::DateTime" = "string"<br />
				"uuid::Uuid" = "string"
			</div>
		</div>

		<!-- [watch] -->
		<div class="rounded-lg border border-border bg-bg-soft p-5 space-y-3">
			<h3 class="text-lg font-semibold">[watch]</h3>
			<p class="text-text-muted text-sm">
				Watch mode settings for <a href="/docs/cli/watch" class="text-accent-ts hover:underline"
					><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">metaxy watch</code></a
				>.
			</p>
			<table class="w-full text-sm">
				<thead
					><tr class="text-left text-text-faint border-b border-border">
						<th class="pb-2 pr-4 font-medium">Field</th><th class="pb-2 pr-4 font-medium">Type</th
						><th class="pb-2 pr-4 font-medium">Default</th><th class="pb-2 font-medium"
							>Description</th
						>
					</tr></thead
				>
				<tbody class="text-text-muted">
					<tr class="border-b border-border/50"
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">debounce_ms</code></td
						><td class="py-1.5 pr-4">number</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">200</code></td
						><td class="py-1.5">Milliseconds to wait after a file change before regenerating</td
						></tr
					>
					<tr
						><td class="py-1.5 pr-4"
							><code class="bg-bg-code px-1 py-0.5 rounded text-xs font-mono">clear_screen</code
							></td
						><td class="py-1.5 pr-4">bool</td><td class="py-1.5 pr-4"
							><code class="text-xs font-mono">false</code></td
						><td class="py-1.5">Clear terminal before each regeneration</td></tr
					>
				</tbody>
			</table>
		</div>
	</div>
</div>
