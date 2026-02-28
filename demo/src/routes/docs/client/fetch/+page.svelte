<script lang="ts">
	import { rpc } from '$lib/client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	let clientResult:
		| { authenticated: boolean; message: string; cookie_value: string | null }
		| undefined = $state();
	let loading = $state(false);

	async function fetchFromClient() {
		loading = true;
		try {
			clientResult = await rpc.query('cookie_demo');
		} catch (e) {
			clientResult = { authenticated: false, message: String(e), cookie_value: null };
		} finally {
			loading = false;
		}
	}

	function setCookie() {
		document.cookie = 'session=demo-session-xyz; path=/; max-age=60';
		fetchFromClient();
	}

	function clearCookie() {
		document.cookie = 'session=; path=/; max-age=0';
		clientResult = undefined;
	}

	let openCode = $state(false);
</script>

<svelte:head>
	<title>Custom Fetch — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Custom Fetch</h1>
	<p class="text-text-muted leading-relaxed">
		Override the <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code> function
		used by the client. Useful for SSR (SvelteKit's platform fetch) and testing (mock fetch).
	</p>

	<h2 class="text-xl font-semibold">Default</h2>
	<p class="text-text-muted text-sm mb-2">
		When not specified, the client uses the global <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code
		>.
	</p>
	<CodeBlock html={data.highlighted['defaultFetch']} />

	<h2 class="text-xl font-semibold">SSR (SvelteKit)</h2>
	<p class="text-text-muted text-sm mb-2">
		In SvelteKit server loads, pass the platform <code
			class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code
		>
		to forward cookies and resolve relative URLs correctly.
	</p>
	<CodeBlock html={data.highlighted['ssrFetch']} />

	<h2 class="text-xl font-semibold">Testing</h2>
	<p class="text-text-muted text-sm mb-2">
		Inject a mock fetch for unit tests without hitting the network.
	</p>
	<CodeBlock html={data.highlighted['testFetch']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>
	<p class="text-text-muted text-sm">
		The server checks for a <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono"
			>session</code
		>
		cookie. SSR <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">event.fetch</code>
		forwards it automatically; browser
		<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">fetch</code> does not (on Vercel,
		cross-origin).
	</p>

	<div class="rounded-lg border border-border bg-bg-soft p-6 space-y-4">
		<div>
			<h3 class="text-lg font-semibold mb-1">SSR Result</h3>
			<p class="text-text-muted text-xs mb-2">
				Called during server load with <code
					class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">event.fetch</code
				>
			</p>
			<div class="rounded-md bg-bg-code p-3 text-xs font-mono">
				{#if data.ssrResult}
					{@const ssr = data.ssrResult as { authenticated: boolean; message: string }}
					<span class={ssr.authenticated ? 'text-green-400' : 'text-red-400'}>
						{ssr.authenticated ? '✓' : '✗'}
						{ssr.message}
					</span>
				{/if}
			</div>
		</div>

		<div>
			<h3 class="text-lg font-semibold mb-1">Client Result</h3>
			<p class="text-text-muted text-xs mb-2">
				Called from browser with default <code
					class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">globalThis.fetch</code
				>
			</p>
			<div class="flex items-center gap-2 mb-2 flex-wrap">
				<button
					onclick={fetchFromClient}
					disabled={loading}
					class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
					>Fetch from client</button
				>
				<button
					onclick={setCookie}
					class="rounded-md bg-green-600 px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85"
					>Set cookie & fetch</button
				>
				<button
					onclick={clearCookie}
					class="rounded-md bg-bg-code px-3 py-1.5 text-sm font-medium text-text-muted transition-opacity hover:opacity-85"
					>Clear cookie</button
				>
			</div>
			{#if clientResult}
				<div class="rounded-md bg-bg-code p-3 text-xs font-mono">
					<span class={clientResult.authenticated ? 'text-green-400' : 'text-red-400'}>
						{clientResult.authenticated ? '✓' : '✗'}
						{clientResult.message}
					</span>
				</div>
			{/if}
		</div>

		<button
			class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
			onclick={() => (openCode = !openCode)}
		>
			{openCode ? '▾ Hide' : '▸ Show'} Rust
		</button>
		{#if openCode}
			<div class="mt-3">
				<CodeBlock html={data.highlighted['cookieDemoRust']} />
			</div>
		{/if}
	</div>
</div>
