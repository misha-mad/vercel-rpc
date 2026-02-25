<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Serde Support — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-10">
	<h1 class="text-3xl font-bold">Serde Support</h1>
	<p class="text-text-muted leading-relaxed">
		vercel-rpc respects serde attributes on your Rust types and translates them into the
		corresponding TypeScript output. This ensures the generated types match exactly what your API
		serializes.
	</p>

	<!-- rename_all -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">rename_all</h2>
		<p class="text-text-muted text-sm">
			Apply a naming convention to all fields or variants. Supported: <code
				class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">camelCase</code
			>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">snake_case</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">PascalCase</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">SCREAMING_SNAKE_CASE</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">kebab-case</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">lowercase</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">UPPERCASE</code>.
		</p>
		<div class="space-y-3">
			<CodeBlock html={data.highlighted['renameAllRust']} />
			<CodeBlock html={data.highlighted['renameAllTs']} />
		</div>
	</section>

	<!-- rename -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">rename</h2>
		<p class="text-text-muted text-sm">
			Override individual field or variant names. Takes priority over <code
				class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">rename_all</code
			>.
		</p>
		<div class="space-y-3">
			<CodeBlock html={data.highlighted['renameFieldRust']} />
			<CodeBlock html={data.highlighted['renameFieldTs']} />
		</div>
	</section>

	<!-- flatten -->
	<section class="space-y-4">
		<h2 class="text-2xl font-semibold">flatten</h2>
		<p class="text-text-muted text-sm">
			Merge a nested struct's fields into the parent. Produces a TypeScript intersection type.
		</p>
		<div class="space-y-3">
			<CodeBlock html={data.highlighted['flattenRust']} />
			<CodeBlock html={data.highlighted['flattenTs']} />
		</div>
	</section>

	<!-- Enum tagging -->
	<section class="space-y-6">
		<h2 class="text-2xl font-semibold">Enum Tagging</h2>
		<p class="text-text-muted text-sm">
			All four serde enum representations are supported. The default is externally tagged.
		</p>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">External (default)</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumExternalRust']} />
				<CodeBlock html={data.highlighted['enumExternalTs']} />
			</div>
		</div>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">
				Internal <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">tag = "..."</code>
			</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumInternalRust']} />
				<CodeBlock html={data.highlighted['enumInternalTs']} />
			</div>
		</div>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">
				Adjacent <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
					>tag + content</code
				>
			</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumAdjacentRust']} />
				<CodeBlock html={data.highlighted['enumAdjacentTs']} />
			</div>
		</div>

		<div class="space-y-4">
			<h3 class="text-lg font-semibold">Untagged</h3>
			<div class="space-y-3">
				<CodeBlock html={data.highlighted['enumUntaggedRust']} />
				<CodeBlock html={data.highlighted['enumUntaggedTs']} />
			</div>
		</div>
	</section>
</div>
