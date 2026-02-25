<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery } from '$lib/rpc.svelte';
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();

	// --- Hello (reactive query with string input) ---
	let name = $state('World');
	const hello = createQuery(rpc, 'hello', () => name);

	// --- Time (void-input reactive query) ---
	const time = createQuery(rpc, 'time');

	// --- Status (void-input reactive query) ---
	const status = createQuery(rpc, 'status');

	// --- Math (reactive query with struct input, Result<T, E>) ---
	let mathA = $state(10);
	let mathB = $state(3);
	let mathOp = $state<'Add' | 'Subtract' | 'Multiply' | 'Divide'>('Add');
	const math = createQuery(rpc, 'math', () => ({ a: mathA, b: mathB, op: mathOp }));

	// --- Stats (reactive query with Vec<f64> input, enabled guard) ---
	let numbersInput = $state('1, 2, 3, 4, 5, 3, 2');
	const parsedNumbers = $derived(
		numbersInput
			.split(',')
			.map((s) => parseFloat(s.trim()))
			.filter((n) => !isNaN(n))
	);
	const stats = createQuery(rpc, 'stats', () => parsedNumbers, {
		enabled: () => parsedNumbers.length > 0
	});

	// --- Profile (reactive query, serde attributes demo) ---
	let profileId = $state(1);
	const profile = createQuery(rpc, 'profile', () => profileId);

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>Queries — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Queries</h1>
	<p class="text-text-muted leading-relaxed">
		Use <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> for read-only operations.
		Wrap them with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createQuery</code> in Svelte for reactive, auto-refetching data.
	</p>

	<!-- Code examples -->
	<div class="space-y-3">
		<CodeBlock html={data.highlighted['exampleRust']} />
		<CodeBlock html={data.highlighted['exampleTs']} />
		<CodeBlock html={data.highlighted['exampleSvelte']} />
	</div>

	<!-- Per-Call Options -->
	<h2 class="text-xl font-semibold mt-8">Per-Call Options</h2>
	<p class="text-text-muted text-sm">
		Every <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">query()</code> call accepts an optional trailing <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">CallOptions</code> object to override client-level defaults.
	</p>
	<CodeBlock html={data.highlighted['callOptionsType']} />
	<CodeBlock html={data.highlighted['callOptionsUsage']} />

	<!-- Try it -->
	<h2 class="text-2xl font-bold mt-12">Try it</h2>

	<!-- Hello: Simple string query -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Hello — Reactive String Query</h3>
		<p class="text-text-muted text-sm mb-4">
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createQuery(rpc, "hello", () => name)</code> — auto-refetches as you type.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<input type="text" bind:value={name} placeholder="Enter your name"
				class="rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts" />
			<button onclick={() => hello.refetch()} disabled={hello.isLoading}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50">Refetch</button>
		</div>
		{#if hello.isLoading && !hello.data}
			<div class="rounded-md bg-bg-code p-3 text-sm text-text-muted">Loading...</div>
		{:else if hello.data}
			<div class="rounded-md bg-bg-code p-3 text-sm text-green-400">{hello.data}</div>
		{/if}
		{#if hello.isError}
			<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{hello.error?.message}</div>
		{/if}
		<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('hello')}>
			{openCode['hello'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['hello']}
			<div class="mt-3 space-y-3">
				<a href="https://github.com/misha-mad/vercel-rpc/blob/main/demo/api/hello.rs" target="_blank" class="text-xs text-text-faint hover:text-accent-rust transition-colors mb-1 block">api/hello.rs</a>
				<CodeBlock html={data.highlighted['helloRust']} />
				<CodeBlock html={data.highlighted['helloTs']} />
			</div>
		{/if}
	</div>

	<!-- Time: Void-input query with struct -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Time — Void Input, Struct Output</h3>
		<p class="text-text-muted text-sm mb-4">
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createQuery(rpc, "time")</code> — auto-fetches on mount, no input needed.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<span class="text-sm text-text-muted">Server time: <strong class="text-text-primary">
				{#if time.isLoading && !time.data}loading...{:else if time.data}{new Date(time.data.timestamp * 1000).toLocaleString()}{/if}
			</strong></span>
			<button onclick={() => time.refetch()}
				class="rounded-md bg-accent-ts px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85">Refresh</button>
		</div>
		<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('time')}>
			{openCode['time'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['time']}
			<div class="mt-3 space-y-3">
				<a href="https://github.com/misha-mad/vercel-rpc/blob/main/demo/api/time.rs" target="_blank" class="text-xs text-text-faint hover:text-accent-rust transition-colors mb-1 block">api/time.rs</a>
				<CodeBlock html={data.highlighted['timeRust']} />
				<CodeBlock html={data.highlighted['timeTs']} />
			</div>
		{/if}
	</div>

	<!-- Status: Enum in struct -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Status — Enum in Struct</h3>
		<p class="text-text-muted text-sm mb-4">
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">HealthStatus</code> enum maps to
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">"Healthy" | "Degraded" | "Down"</code>.
		</p>
		{#if status.data}
			<div class="rounded-md bg-bg-code p-3 text-sm space-y-1">
				<div><span class="text-text-muted">Service:</span> <span class="text-text-primary">{status.data.name}</span></div>
				<div><span class="text-text-muted">Status:</span> <span class="text-green-400">{status.data.status}</span></div>
				<div><span class="text-text-muted">Uptime:</span> <span class="text-text-primary">{status.data.uptime_secs}s</span></div>
			</div>
		{:else if status.isLoading}
			<div class="rounded-md bg-bg-code p-3 text-sm text-text-muted">Loading...</div>
		{/if}
		<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('status')}>
			{openCode['status'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
		</button>
		{#if openCode['status']}
			<div class="mt-3 space-y-3">
				<a href="https://github.com/misha-mad/vercel-rpc/blob/main/demo/api/status.rs" target="_blank" class="text-xs text-text-faint hover:text-accent-rust transition-colors mb-1 block">api/status.rs</a>
				<CodeBlock html={data.highlighted['statusRust']} />
				<CodeBlock html={data.highlighted['statusTs']} />
			</div>
		{/if}
	</div>

	<!-- Math: Struct input, Result<T, E> -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Math — Struct Input, Result&lt;T, E&gt;</h3>
		<p class="text-text-muted text-sm mb-4">
			Demonstrates struct input and <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Result&lt;T, E&gt;</code> error handling.
		</p>
		<div class="flex flex-wrap items-center gap-3 mb-3">
			<input type="number" bind:value={mathA}
				class="w-20 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts" />
			<select bind:value={mathOp}
				class="rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts">
				<option>Add</option><option>Subtract</option><option>Multiply</option><option>Divide</option>
			</select>
			<input type="number" bind:value={mathB}
				class="w-20 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts" />
		</div>
		{#if math.data !== undefined}
			<div class="rounded-md bg-bg-code p-3 text-sm text-green-400">Result: {math.data}</div>
		{/if}
		{#if math.isError}
			<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{math.error?.message}</div>
		{/if}
	</div>

	<!-- Stats: Vec input, enabled guard -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Stats — Vec Input &amp; Enabled Guard</h3>
		<p class="text-text-muted text-sm mb-4">
			Accepts <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Vec&lt;f64&gt;</code>, uses
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">enabled</code> option to skip empty input.
		</p>
		<input type="text" bind:value={numbersInput} placeholder="Comma-separated numbers"
			class="w-full rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts mb-3" />
		{#if stats.data}
			<div class="rounded-md bg-bg-code p-3 text-sm space-y-1">
				<div><span class="text-text-muted">Count:</span> <span class="text-text-primary">{stats.data.count}</span></div>
				<div><span class="text-text-muted">Mean:</span> <span class="text-text-primary">{stats.data.mean.toFixed(2)}</span></div>
				<div><span class="text-text-muted">Min:</span> <span class="text-text-primary">{stats.data.min}</span> / <span class="text-text-muted">Max:</span> <span class="text-text-primary">{stats.data.max}</span></div>
			</div>
		{/if}
	</div>

	<!-- Profile: serde attributes -->
	<div class="rounded-lg border border-border bg-bg-soft p-6">
		<h3 class="text-lg font-semibold mb-2">Profile — Serde Attributes</h3>
		<p class="text-text-muted text-sm mb-4">
			Demonstrates <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(rename_all)</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(rename)</code>,
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(skip)</code>, and
			<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Option&lt;T&gt;</code>.
		</p>
		<div class="flex items-center gap-3 mb-3">
			<label class="text-sm text-text-muted">Profile ID:
				<input type="number" bind:value={profileId} min="1" max="5"
					class="w-20 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts" />
			</label>
		</div>
		{#if profile.data}
			<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{JSON.stringify(profile.data, null, 2)}</pre>
		{/if}
	</div>
</div>
