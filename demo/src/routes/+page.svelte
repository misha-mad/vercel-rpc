<script lang="ts">
	import { resolve } from '$app/paths';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>metaxy</title>
	<meta
		name="description"
		content="End-to-end typesafe RPC between Rust lambdas on Vercel and any frontend"
	/>
</svelte:head>

<section
	class="relative flex flex-col items-center justify-center px-4 py-20 text-center overflow-hidden"
>
	<img
		src="/hero-left.png"
		alt=""
		class="pointer-events-none absolute left-0 top-1/2 -translate-y-1/2 w-1/3 object-contain invert opacity-70 hidden sm:block"
	/>
	<img
		src="/hero-right.png"
		alt=""
		class="pointer-events-none absolute right-0 top-1/2 -translate-y-1/2 w-1/3 object-contain invert opacity-70 hidden sm:block"
	/>
	<h1 class="relative text-4xl sm:text-5xl font-bold mb-2">metaxy</h1>
	<p class="text-sm text-text-faint mb-6">/mɛˈtæk.si/ · Greek: μεταξύ · <em>the in-between</em></p>

	<p class="text-lg sm:text-xl text-text-muted max-w-2xl mb-10 leading-relaxed">
		End-to-end typesafe RPC between<br />
		<strong class="text-accent-rust">Rust lambdas</strong>
		&amp;
		<strong class="text-accent-ts">TS frontend</strong>
	</p>

	<div class="flex gap-4 mb-16">
		<a
			href={resolve('/docs')}
			class="rounded-lg bg-accent-rust px-6 py-2.5 font-semibold text-white text-sm transition-opacity hover:opacity-85"
		>
			Get Started →
		</a>
		<a
			href="https://github.com/misha-mad/metaxy"
			class="rounded-lg border border-accent-ts px-6 py-2.5 font-semibold text-accent-ts text-sm transition-opacity hover:opacity-85"
		>
			GitHub
		</a>
	</div>
</section>

<section
	class="border-t border-b border-border grid grid-cols-1 sm:grid-cols-3 gap-px bg-border mb-16"
>
	<!-- 1: code left, text right -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 flex-1">
			<CodeBlock html={data.highlighted['typeSafetyRust']} />
			<CodeBlock html={data.highlighted['typeSafetyTs']} />
		</div>
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">End-to-end type safety</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Define a Rust struct once — the CLI generates matching TypeScript interfaces. Rename a field,
			and your frontend won't compile until it's updated. No more <code
				class="bg-bg-code px-1.5 py-0.5 rounded text-xs">any</code
			>, no runtime surprises.
		</p>
	</div>

	<!-- 2: text left, code right -->
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Auto-generated client</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			A fully typed <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.query()</code> /
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.mutate()</code> client with autocomplete
			for every procedure. Input and output types are inferred from your Rust code.
		</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['autoClient']} />
	</div>

	<!-- 3: code left, text right -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['watchMode']} />
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Watch mode</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Save a <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> file and types regenerate
			instantly. Runs alongside your dev server so the frontend always has the latest types without manual
			steps.
		</p>
	</div>

	<!-- 4: text left, code right -->
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Macro-driven</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Annotate with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_query]</code> or
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_mutation]</code> and get CORS, input
			parsing, JSON serialization, error handling, and HTTP method validation — all generated at compile
			time.
		</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['macroDriven']} />
	</div>

	<!-- 5: code left, text right -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['initState']} />
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Init &amp; state injection</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Set up database pools, HTTP clients, or loggers once at cold start. The macro injects shared
			state as <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">&T</code> into your handler automatically.
		</p>
	</div>

	<!-- 6: text left, code right -->
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Serde support</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rename_all</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">skip</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">flatten</code>, and all four enum
			tagging strategies — the codegen reads your serde attributes and generates TypeScript that
			matches the actual JSON output.
		</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['serde']} />
	</div>

	<!-- 7: code left, text right -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['edgeCache']} />
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Edge caching</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Add <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">cache = "1h"</code> and the macro
			generates <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">Cache-Control</code> headers. On
			Vercel, this enables CDN caching with zero infrastructure changes.
		</p>
	</div>

	<!-- 8: text left, code right -->
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Vercel-native</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Each <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> file in
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">api/</code>
			becomes a serverless function. No routing config, no server setup — just deploy with
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">vercel</code>.
		</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['vercelNative']} />
	</div>

	<!-- 9: code left, text right -->
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['frameworks']} />
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">4 framework wrappers</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Opt-in reactive wrappers for Svelte 5, React, Vue 3, and SolidJS. Auto-refetching queries,
			mutation lifecycle callbacks — generated alongside the base client.
		</p>
	</div>

	<!-- 10: text left, code right -->
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">Rich client</h3>
		<p class="text-sm text-text-muted leading-relaxed">
			Retry with backoff, per-request timeout, AbortSignal support, request deduplication, lifecycle
			hooks, async headers — all configurable globally or per-call.
		</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		<CodeBlock html={data.highlighted['richClient']} />
	</div>
</section>
