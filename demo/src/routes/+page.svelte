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

<!-- SVG pattern definitions -->
<svg class="hidden">
	<defs>
		<!-- 1: Diagonal lines -->
		<pattern id="pat-diag" width="20" height="20" patternUnits="userSpaceOnUse">
			<line x1="0" y1="20" x2="20" y2="0" stroke="black" stroke-width="1" />
		</pattern>
		<!-- 2: Horizontal waves -->
		<pattern id="pat-wave" width="40" height="20" patternUnits="userSpaceOnUse">
			<path d="M0 10 Q10 0 20 10 Q30 20 40 10" fill="none" stroke="black" stroke-width="1" />
		</pattern>
		<!-- 3: Concentric circles -->
		<pattern id="pat-circles" width="60" height="60" patternUnits="userSpaceOnUse">
			<circle cx="30" cy="30" r="5" fill="none" stroke="black" stroke-width="0.5" />
			<circle cx="30" cy="30" r="12" fill="none" stroke="black" stroke-width="0.5" />
			<circle cx="30" cy="30" r="19" fill="none" stroke="black" stroke-width="0.5" />
			<circle cx="30" cy="30" r="26" fill="none" stroke="black" stroke-width="0.5" />
		</pattern>
		<!-- 4: Zigzag -->
		<pattern id="pat-zigzag" width="20" height="20" patternUnits="userSpaceOnUse">
			<polyline points="0,10 5,0 10,10 15,0 20,10" fill="none" stroke="black" stroke-width="1" />
		</pattern>
		<!-- 5: Horizontal lines varying thickness -->
		<pattern id="pat-hlines" width="10" height="24" patternUnits="userSpaceOnUse">
			<line x1="0" y1="4" x2="10" y2="4" stroke="black" stroke-width="0.5" />
			<line x1="0" y1="12" x2="10" y2="12" stroke="black" stroke-width="1.5" />
			<line x1="0" y1="20" x2="10" y2="20" stroke="black" stroke-width="0.5" />
		</pattern>
		<!-- 6: Dots grid -->
		<pattern id="pat-dots" width="16" height="16" patternUnits="userSpaceOnUse">
			<circle cx="8" cy="8" r="1.5" fill="black" />
		</pattern>
		<!-- 7: Cross-hatch -->
		<pattern id="pat-cross" width="16" height="16" patternUnits="userSpaceOnUse">
			<line x1="0" y1="16" x2="16" y2="0" stroke="black" stroke-width="0.5" />
			<line x1="0" y1="0" x2="16" y2="16" stroke="black" stroke-width="0.5" />
		</pattern>
		<!-- 8: Vertical lines -->
		<pattern id="pat-vlines" width="12" height="10" patternUnits="userSpaceOnUse">
			<line x1="4" y1="0" x2="4" y2="10" stroke="black" stroke-width="0.5" />
			<line x1="8" y1="0" x2="8" y2="10" stroke="black" stroke-width="1.5" />
		</pattern>
		<!-- 9: Arcs -->
		<pattern id="pat-arcs" width="30" height="30" patternUnits="userSpaceOnUse">
			<path d="M0 30 A30 30 0 0 1 30 0" fill="none" stroke="black" stroke-width="0.8" />
			<path d="M0 15 A15 15 0 0 1 15 0" fill="none" stroke="black" stroke-width="0.8" />
		</pattern>
		<!-- 10: Diamond grid -->
		<pattern id="pat-diamond" width="24" height="24" patternUnits="userSpaceOnUse">
			<polygon points="12,2 22,12 12,22 2,12" fill="none" stroke="black" stroke-width="0.7" />
		</pattern>
	</defs>
</svg>

<section class="border-t border-border grid grid-cols-1 sm:grid-cols-3 gap-px bg-border">

	<!-- Row 1: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">End-to-end type safety</h3>
		<p class="text-sm text-text-muted">Rust types become TypeScript interfaces — zero manual sync.</p>
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mt-2 flex-1">
			<CodeBlock html={data.highlighted['typeSafetyRust']} />
			<CodeBlock html={data.highlighted['typeSafetyTs']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-diag)" /></svg>
	</div>

	<!-- Row 2: right -->
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-wave)" /></svg>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Auto-generated client</h3>
		<p class="text-sm text-text-muted">Typed <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.query()</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.mutate()</code> with autocomplete.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['autoClient']} />
		</div>
	</div>

	<!-- Row 3: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Watch mode</h3>
		<p class="text-sm text-text-muted">Types regenerate on every <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> save — your frontend stays in sync automatically.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['watchMode']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-circles)" /></svg>
	</div>

	<!-- Row 4: right -->
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-zigzag)" /></svg>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Macro-driven</h3>
		<p class="text-sm text-text-muted"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_query]</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_mutation]</code> — CORS, parsing & errors built in.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['macroDriven']} />
		</div>
	</div>

	<!-- Row 5: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Serde support</h3>
		<p class="text-sm text-text-muted"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rename_all</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">skip</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">flatten</code> & all enum tagging strategies.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['serde']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-hlines)" /></svg>
	</div>

	<!-- Row 6: right -->
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-dots)" /></svg>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Init &amp; state injection</h3>
		<p class="text-sm text-text-muted">Cold-start setup for DB pools, HTTP clients via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">init = "setup"</code>.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['initState']} />
		</div>
	</div>

	<!-- Row 7: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Edge caching</h3>
		<p class="text-sm text-text-muted"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">cache = "1h"</code> generates Cache-Control headers for Vercel CDN.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['edgeCache']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-cross)" /></svg>
	</div>

	<!-- Row 8: right -->
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-vlines)" /></svg>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">4 framework wrappers</h3>
		<p class="text-sm text-text-muted">Svelte 5, React, Vue 3, SolidJS — reactive hooks, opt-in.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['frameworks']} />
		</div>
	</div>

	<!-- Row 9: left -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Rich client</h3>
		<p class="text-sm text-text-muted">Retry, timeout, dedup, lifecycle hooks, custom headers, AbortSignal.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['richClient']} />
		</div>
	</div>
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-arcs)" /></svg>
	</div>

	<!-- Row 10: right -->
	<div class="hidden sm:block bg-bg-primary overflow-hidden">
		<svg class="h-full w-full"><rect width="100%" height="100%" fill="url(#pat-diamond)" /></svg>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3 sm:min-h-[576px]">
		<h3 class="text-lg font-semibold">Vercel-native</h3>
		<p class="text-sm text-text-muted">Each <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> file deploys as a serverless lambda — no infra config needed.</p>
		<div class="mt-2 flex-1 flex flex-col">
			<CodeBlock html={data.highlighted['vercelNative']} />
		</div>
	</div>

</section>
