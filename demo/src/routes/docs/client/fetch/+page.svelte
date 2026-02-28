<script lang="ts">
	import { rpc } from '$lib/client';
	import CodeBlock from '$lib/components/CodeBlock.svelte';
	import CollapsibleCode from '$lib/components/CollapsibleCode.svelte';
	import Code from '$lib/components/Code.svelte';
	import Button from '$lib/components/Button.svelte';
	import PageHeader from '$lib/components/PageHeader.svelte';
	import SectionHeading from '$lib/components/SectionHeading.svelte';
	import DemoCard from '$lib/components/DemoCard.svelte';
	import OutputBox from '$lib/components/OutputBox.svelte';

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
</script>

<svelte:head>
	<title>Custom Fetch — metaxy</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<PageHeader title="Custom Fetch">
		Override the <Code>fetch</Code> function
		used by the client. Useful for SSR (SvelteKit's platform fetch) and testing (mock fetch).
	</PageHeader>

	<SectionHeading>Default</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		When not specified, the client uses the global <Code>fetch</Code>.
	</p>
	<CodeBlock html={data.highlighted['defaultFetch']} />

	<SectionHeading>SSR (SvelteKit)</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		In SvelteKit server loads, pass the platform <Code>fetch</Code>
		to forward cookies and resolve relative URLs correctly.
	</p>
	<CodeBlock html={data.highlighted['ssrFetch']} />

	<SectionHeading>Testing</SectionHeading>
	<p class="text-text-muted text-sm mb-2">
		Inject a mock fetch for unit tests without hitting the network.
	</p>
	<CodeBlock html={data.highlighted['testFetch']} />

	<!-- Try it -->
	<SectionHeading level="large">Try it</SectionHeading>
	<p class="text-text-muted text-sm">
		The server checks for a <Code>session</Code>
		cookie. SSR <Code>event.fetch</Code>
		forwards it automatically; browser
		<Code>fetch</Code> does not (on Vercel,
		cross-origin).
	</p>

	<DemoCard>
		<div class="space-y-4">
			<div>
				<h3 class="text-lg font-semibold mb-1">SSR Result</h3>
				<p class="text-text-muted text-xs mb-2">
					Called during server load with <Code>event.fetch</Code>
				</p>
				<OutputBox mono>
					{#if data.ssrResult}
						{@const ssr = data.ssrResult as { authenticated: boolean; message: string }}
						<span class={ssr.authenticated ? 'text-green-400' : 'text-red-400'}>
							{ssr.authenticated ? '✓' : '✗'}
							{ssr.message}
						</span>
					{/if}
				</OutputBox>
			</div>

			<div>
				<h3 class="text-lg font-semibold mb-1">Client Result</h3>
				<p class="text-text-muted text-xs mb-2">
					Called from browser with default <Code>globalThis.fetch</Code>
				</p>
				<div class="flex items-center gap-2 mb-2 flex-wrap">
					<Button onclick={fetchFromClient} disabled={loading}>Fetch from client</Button>
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
					<OutputBox mono>
						<span class={clientResult.authenticated ? 'text-green-400' : 'text-red-400'}>
							{clientResult.authenticated ? '✓' : '✗'}
							{clientResult.message}
						</span>
					</OutputBox>
				{/if}
			</div>
		</div>

		<CollapsibleCode html={data.highlighted['cookieDemoRust']} />
	</DemoCard>
</div>
