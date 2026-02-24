<script lang="ts">
	import { resolve } from '$app/paths';
</script>

<svelte:head>
	<title>vercel-rpc</title>
	<meta name="description" content="End-to-end typesafe RPC between Rust lambdas on Vercel and any frontend" />
</svelte:head>

<section class="flex flex-col items-center justify-center px-4 py-20 text-center">
	<h1 class="text-4xl sm:text-5xl font-bold mb-4">
		<span class="text-3xl sm:text-4xl">⚡</span> vercel-rpc
	</h1>

	<p class="text-lg sm:text-xl text-text-muted max-w-2xl mb-2">
		End-to-end typesafe RPC between <strong class="text-accent-rust">Rust lambdas</strong> on Vercel and your <strong class="text-accent-ts">TypeScript frontend</strong>.
	</p>

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-10 max-w-xl text-left text-sm">
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span class="text-accent-rust">🦀</span>
			<span><strong>Rust functions</strong> with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_query]</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_mutation]</code></span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span>🔄</span>
			<span><strong>Auto-generated</strong> TypeScript types &amp; client</span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span>👀</span>
			<span><strong>Watch mode</strong> — types regenerate on every save</span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span>🚀</span>
			<span><strong>Deploy to Vercel</strong> — each function becomes a lambda</span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3 sm:col-span-2 sm:max-w-[calc(50%-0.375rem)]">
			<span>🛡️</span>
			<span><strong>Type safety</strong> — Rust → TypeScript, no manual sync</span>
		</div>
	</div>

	<div class="flex gap-4">
		<a href={resolve('/docs')} class="rounded-lg bg-accent-rust px-6 py-2.5 font-semibold text-white text-sm transition-opacity hover:opacity-85">
			Get Started →
		</a>
		<a href="https://github.com/misha-mad/vercel-rpc" class="rounded-lg border border-accent-ts px-6 py-2.5 font-semibold text-accent-ts text-sm transition-opacity hover:opacity-85">
			GitHub
		</a>
	</div>
</section>
