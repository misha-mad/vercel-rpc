<script lang="ts">
	import { rpc } from '$lib/client';
	import { createQuery, createMutation, RpcError } from '$lib/rpc.svelte';
	import { createRpcClient } from '$lib/rpc-client';

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

	// --- Types (reactive query, expanded type mappings demo) ---
	let typesCategory = $state('demo');
	const types = createQuery(rpc, 'types', () => typesCategory);

	// --- Echo (mutation with struct input/output) ---
	let echoMessage = $state('Hello from vercel-rpc!');
	let echoUppercase = $state(false);
	const echo = createMutation(rpc, 'echo');

	// --- Code toggle ---
	let openCode: Record<string, boolean> = $state({});
	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}
</script>

<svelte:head>
	<title>Docs — vercel-rpc</title>
	<meta name="description" content="vercel-rpc documentation" />
</svelte:head>

<div class="max-w-3xl space-y-16">
	<section id="getting-started">
		<h1 class="text-3xl font-bold mb-4">Getting Started</h1>

		<p class="text-text-muted leading-relaxed mb-4">
			<strong class="text-text-primary">vercel-rpc</strong> is an end-to-end typesafe RPC toolkit for building
			serverless APIs with Rust on Vercel. Write plain Rust functions, and get a fully typed TypeScript
			client — no manual sync required.
		</p>

		<h3 class="text-xl font-semibold mb-3 mt-8">How it works</h3>

		<ol class="list-decimal list-inside space-y-2 text-text-muted mb-6">
			<li>Annotate Rust functions with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> or <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code></li>
			<li>The CLI scans your <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">api/</code> directory and parses Rust types via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code></li>
			<li>TypeScript types and a typed client are generated automatically</li>
			<li>Each Rust file deploys as a serverless lambda on Vercel</li>
		</ol>

		<h3 class="text-xl font-semibold mb-3 mt-8">Quick example</h3>

		<p class="text-text-muted text-sm mb-2">Rust — <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">api/hello.rs</code></p>
		<pre class="bg-bg-code rounded-lg p-4 font-mono text-sm overflow-x-auto mb-4 text-text-primary">{`#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`}</pre>

		<p class="text-text-muted text-sm mb-2">Generated TypeScript client</p>
		<pre class="bg-bg-code rounded-lg p-4 font-mono text-sm overflow-x-auto mb-4 text-text-primary">{`import { createRpcClient } from './rpc-client';

const rpc = createRpcClient({ baseUrl: '/api' });
const greeting = await rpc.query('hello', 'World');
// greeting: string — "Hello, World from Rust on Vercel!"`}</pre>

		<p class="text-text-muted text-sm mb-2">Or with Svelte 5 reactive wrapper</p>
		<pre class="bg-bg-code rounded-lg p-4 font-mono text-sm overflow-x-auto mb-4 text-text-primary">{`import { createQuery } from './rpc.svelte';

let name = $state('World');
const hello = createQuery(rpc, 'hello', () => name);
// hello.data reactively updates when 'name' changes`}</pre>
	</section>

	<section id="queries">
		<h2 class="text-2xl font-bold mb-4">Queries</h2>
		<p class="text-text-muted leading-relaxed mb-6">
			Use <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> for read-only operations.
			Wrap them with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createQuery</code> in Svelte for reactive, auto-refetching data.
		</p>

		<!-- Hello: Simple string query -->
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
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
				<div class="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
					<div>
						<span class="text-xs text-accent-rust mb-1 block">Rust — api/hello.rs</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`}</pre>
					</div>
					<div>
						<span class="text-xs text-accent-ts mb-1 block">Svelte 5 Reactive Wrapper</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`const hello = createQuery(rpc, "hello", () => name);

hello.data       // string | undefined
hello.isLoading  // boolean
hello.isError    // boolean
hello.refetch()  // manual refetch`}</pre>
					</div>
				</div>
			{/if}
		</div>

		<!-- Time: Void-input query with struct -->
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
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
				<div class="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
					<div>
						<span class="text-xs text-accent-rust mb-1 block">Rust — api/time.rs</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`#[derive(Serialize)]
pub struct TimeResponse {
    pub timestamp: u64,
    pub message: String,
}

#[rpc_query]
async fn time() -> TimeResponse {
    TimeResponse { timestamp: now, message: "..." }
}`}</pre>
					</div>
					<div>
						<span class="text-xs text-accent-ts mb-1 block">Generated TypeScript</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`interface TimeResponse {
  timestamp: number;  // u64 → number
  message: string;    // String → string
}

const time = createQuery(rpc, "time");
// time.data?.timestamp, time.isLoading`}</pre>
					</div>
				</div>
			{/if}
		</div>

		<!-- Status: Enum in struct -->
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
			<h3 class="text-lg font-semibold mb-2">Status — Enum in Struct</h3>
			<p class="text-text-muted text-sm mb-4">
				<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">HealthStatus</code> enum maps to
				<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">"Healthy" | "Degraded" | "Down"</code>.
			</p>
			{#if status.data}
				<div class="rounded-md bg-bg-code p-3 text-sm space-y-1">
					<div><span class="text-text-muted">Service:</span> <span class="text-text-primary">{status.data.name}</span></div>
					<div><span class="text-text-muted">Status:</span> <span class="text-green-400">{status.data.status}</span></div>
					<div><span class="text-text-muted">Uptime:</span> <span class="text-text-primary">{status.data.uptime_seconds}s</span></div>
				</div>
			{:else if status.isLoading}
				<div class="rounded-md bg-bg-code p-3 text-sm text-text-muted">Loading...</div>
			{/if}
			<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('status')}>
				{openCode['status'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
			</button>
			{#if openCode['status']}
				<div class="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
					<div>
						<span class="text-xs text-accent-rust mb-1 block">Rust — api/status.rs</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`#[derive(Serialize)]
pub enum HealthStatus {
    Healthy, Degraded, Down,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub name: String,
    pub status: HealthStatus,
    pub uptime_seconds: u64,
}`}</pre>
					</div>
					<div>
						<span class="text-xs text-accent-ts mb-1 block">Generated TypeScript</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`type HealthStatus = "Healthy" | "Degraded" | "Down";

interface StatusResponse {
  name: string;
  status: HealthStatus;
  uptime_seconds: number;
}`}</pre>
					</div>
				</div>
			{/if}
		</div>

		<!-- Math: Struct input, Result<T, E> -->
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
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
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
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
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
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
	</section>

	<section id="mutations">
		<h2 class="text-2xl font-bold mb-4">Mutations</h2>
		<p class="text-text-muted leading-relaxed mb-6">
			Use <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code> for write operations (create, update, delete).
			Unlike queries, mutations are triggered explicitly via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">.mutate()</code> — they don't auto-refetch.
		</p>

		<!-- Echo: mutation with struct input/output -->
		<div class="rounded-lg border border-border bg-bg-soft p-6 mb-6">
			<h3 class="text-lg font-semibold mb-2">Echo — Struct Mutation</h3>
			<p class="text-text-muted text-sm mb-4">
				Send a message, optionally uppercase it. Demonstrates <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">createMutation</code> with struct input/output.
			</p>
			<div class="flex flex-wrap items-center gap-3 mb-3">
				<input type="text" bind:value={echoMessage} placeholder="Enter message"
					class="flex-1 min-w-48 rounded-md border border-border bg-bg-code px-3 py-1.5 text-sm text-text-primary outline-none focus:border-accent-ts" />
				<label class="flex items-center gap-2 text-sm text-text-muted">
					<input type="checkbox" bind:checked={echoUppercase} class="rounded" />
					Uppercase
				</label>
				<button
					onclick={() => echo.mutate({ message: echoMessage, uppercase: echoUppercase })}
					disabled={echo.isPending}
					class="rounded-md bg-accent-rust px-4 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50">
					{echo.isPending ? 'Sending...' : 'Send'}
				</button>
			</div>
			{#if echo.data}
				<div class="rounded-md bg-bg-code p-3 text-sm space-y-1">
					<div><span class="text-text-muted">Response:</span> <span class="text-green-400">{echo.data.message}</span></div>
					<div><span class="text-text-muted">Length:</span> <span class="text-text-primary">{echo.data.length}</span></div>
				</div>
			{/if}
			{#if echo.isError}
				<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">{echo.error?.message}</div>
			{/if}
			<button class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors" onclick={() => toggleCode('echo')}>
				{openCode['echo'] ? '▾ Hide' : '▸ Show'} Rust &amp; TypeScript
			</button>
			{#if openCode['echo']}
				<div class="mt-3 grid grid-cols-1 md:grid-cols-2 gap-3">
					<div>
						<span class="text-xs text-accent-rust mb-1 block">Rust — api/echo.rs</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`#[derive(Deserialize)]
pub struct EchoInput {
    pub message: String,
    pub uppercase: bool,
}

#[derive(Serialize)]
pub struct EchoOutput {
    pub message: String,
    pub length: usize,
}

#[rpc_mutation]
async fn echo(input: EchoInput) -> EchoOutput {
    let msg = if input.uppercase {
        input.message.to_uppercase()
    } else {
        input.message
    };
    EchoOutput { message: msg.clone(), length: msg.len() }
}`}</pre>
					</div>
					<div>
						<span class="text-xs text-accent-ts mb-1 block">Svelte 5 Usage</span>
						<pre class="bg-bg-code rounded-lg p-3 font-mono text-xs overflow-x-auto text-text-primary">{`const echo = createMutation(rpc, "echo");

// Trigger explicitly:
echo.mutate({ message: "Hello", uppercase: true });

echo.data       // EchoOutput | undefined
echo.isPending  // boolean (loading)
echo.isError    // boolean
echo.error      // RpcError | undefined`}</pre>
					</div>
				</div>
			{/if}
		</div>
	</section>

	<section id="type-mappings">
		<h2 class="text-2xl font-bold mb-4">Type Mappings</h2>
		<p class="text-text-muted leading-relaxed mb-6">
			Every Rust type is automatically mapped to its TypeScript equivalent during code generation. Here's the complete reference.
		</p>

		<div class="overflow-x-auto rounded-lg border border-border">
			<table class="w-full text-sm text-left">
				<thead class="bg-bg-code text-text-muted text-xs uppercase">
					<tr>
						<th class="px-4 py-3">Rust</th>
						<th class="px-4 py-3">TypeScript</th>
						<th class="px-4 py-3">Example</th>
					</tr>
				</thead>
				<tbody class="text-text-primary">
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">String</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">&amp;str</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">string</code></td>
						<td class="px-4 py-2 text-text-muted">hello endpoint</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">i32</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">u64</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">f64</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">number</code></td>
						<td class="px-4 py-2 text-text-muted">math, time</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">bool</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">boolean</code></td>
						<td class="px-4 py-2 text-text-muted">echo (uppercase)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">()</code> (no input)</td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">void</code></td>
						<td class="px-4 py-2 text-text-muted">time, status</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Vec&lt;T&gt;</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">HashSet&lt;T&gt;</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T[]</code></td>
						<td class="px-4 py-2 text-text-muted">stats, types</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Option&lt;T&gt;</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T | null</code></td>
						<td class="px-4 py-2 text-text-muted">profile (avatarUrl)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">HashMap&lt;K, V&gt;</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Record&lt;K, V&gt;</code></td>
						<td class="px-4 py-2 text-text-muted">stats (frequencies)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Box&lt;T&gt;</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Cow&lt;T&gt;</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T</code> (transparent)</td>
						<td class="px-4 py-2 text-text-muted">types (boxed_label)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Result&lt;T, E&gt;</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">T</code> (error at runtime)</td>
						<td class="px-4 py-2 text-text-muted">math, stats</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">struct</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">interface</code></td>
						<td class="px-4 py-2 text-text-muted">TimeResponse, Stats</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">enum</code> (unit)</td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">"A" | "B"</code></td>
						<td class="px-4 py-2 text-text-muted">HealthStatus, Operation</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(rename_all)</code></td>
						<td class="px-4 py-2">field/variant names transformed</td>
						<td class="px-4 py-2 text-text-muted">profile (camelCase)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(rename)</code></td>
						<td class="px-4 py-2">exact name override</td>
						<td class="px-4 py-2 text-text-muted">profile (profile_url)</td>
					</tr>
					<tr class="border-b border-border">
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(skip)</code></td>
						<td class="px-4 py-2">field omitted</td>
						<td class="px-4 py-2 text-text-muted">profile (internal_score)</td>
					</tr>
					<tr>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">serde(default)</code> + <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">Option&lt;T&gt;</code></td>
						<td class="px-4 py-2"><code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">field?: T | null</code></td>
						<td class="px-4 py-2 text-text-muted">profile (avatarUrl)</td>
					</tr>
				</tbody>
			</table>
		</div>
	</section>

	<section id="error-handling">
		<h2 class="text-2xl font-bold mb-4">Error Handling</h2>
		<p class="text-text-muted">Documentation coming in next tasks...</p>
	</section>

	<section id="streaming">
		<h2 class="text-2xl font-bold mb-4">Streaming</h2>
		<p class="text-text-muted">Documentation coming in next tasks...</p>
	</section>
</div>
