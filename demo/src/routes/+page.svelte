<script lang="ts">
	import { resolve } from '$app/paths';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>vercel-rpc</title>
	<meta name="description" content="End-to-end typesafe RPC between Rust lambdas on Vercel and any frontend" />
</svelte:head>

<section class="flex flex-col items-center justify-center px-4 py-20 text-center">
	<h1 class="text-4xl sm:text-5xl font-bold mb-4">
		vercel-rpc
	</h1>

	<p class="text-lg sm:text-xl text-text-muted max-w-2xl mb-10 leading-relaxed">
		End-to-end typesafe RPC between<br />
		<strong class="text-accent-rust">Rust lambdas</strong>
		&amp;
		<strong class="text-accent-ts">TS frontend</strong>
	</p>

	<div class="flex gap-4 mb-16">
		<a href={resolve('/docs')} class="rounded-lg bg-accent-rust px-6 py-2.5 font-semibold text-white text-sm transition-opacity hover:opacity-85">
			Get Started →
		</a>
		<a href="https://github.com/misha-mad/vercel-rpc" class="rounded-lg border border-accent-ts px-6 py-2.5 font-semibold text-accent-ts text-sm transition-opacity hover:opacity-85">
			GitHub
		</a>
	</div>
</section>

<section class="border-t border-border grid grid-cols-1 sm:grid-cols-3 gap-px bg-border">

	<!-- Row 1: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">End-to-end type safety</h3>
		<p class="text-sm text-text-muted">Rust types become TypeScript interfaces — zero manual sync.</p>
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mt-2">
			<CodeBlock html={data.highlighted['typeSafetyRust']} />
			<CodeBlock html={data.highlighted['typeSafetyTs']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary"></div>

	<!-- Row 2: right -->
	<div class="hidden sm:block bg-bg-primary"></div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Auto-generated client</h3>
		<p class="text-sm text-text-muted">Typed <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.query()</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.mutate()</code> with autocomplete.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['autoClient']} />
		</div>
	</div>

	<!-- Row 3: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Watch mode</h3>
		<p class="text-sm text-text-muted">Types regenerate on every <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> save — your frontend stays in sync automatically.</p>
	</div>
	<div class="hidden sm:block bg-bg-primary"></div>

	<!-- Row 4: right -->
	<div class="hidden sm:block bg-bg-primary"></div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Macro-driven</h3>
		<p class="text-sm text-text-muted"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_query]</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_mutation]</code> — CORS, parsing & errors built in.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['macroDriven']} />
		</div>
	</div>

	<!-- Row 5: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Serde support</h3>
		<p class="text-sm text-text-muted"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rename_all</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">skip</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">flatten</code> & all enum tagging strategies.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['serde']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary"></div>

	<!-- Row 6: right -->
	<div class="hidden sm:block bg-bg-primary"></div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Init &amp; state injection</h3>
		<p class="text-sm text-text-muted">Cold-start setup for DB pools, HTTP clients via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">init = "setup"</code>.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['initState']} />
		</div>
	</div>

	<!-- Row 7: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Edge caching</h3>
		<p class="text-sm text-text-muted"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">cache = "1h"</code> generates Cache-Control headers for Vercel CDN.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['edgeCache']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary"></div>

	<!-- Row 8: right -->
	<div class="hidden sm:block bg-bg-primary"></div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">4 framework wrappers</h3>
		<p class="text-sm text-text-muted">Svelte 5, React, Vue 3, SolidJS — reactive hooks, opt-in.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['frameworks']} />
		</div>
	</div>

	<!-- Row 9: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Rich client</h3>
		<p class="text-sm text-text-muted">Retry, timeout, dedup, lifecycle hooks, custom headers, AbortSignal.</p>
		<div class="mt-2">
			<CodeBlock html={data.highlighted['richClient']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary"></div>

	<!-- Row 10: right -->
	<div class="hidden sm:block bg-bg-primary"></div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Vercel-native</h3>
		<p class="text-sm text-text-muted">Each <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> file deploys as a serverless lambda — no infra config needed.</p>
	</div>

</section>
