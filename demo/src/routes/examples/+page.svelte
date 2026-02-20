<script lang="ts">
	import { rpc } from '$lib/client';
	import { RpcError, createRpcClient } from '$lib/rpc-client';
	import type {
		MathResult,
		Stats,
		ServiceStatus,
		EchoOutput,
		UserProfile,
		TypeShowcase
	} from '$lib/rpc-client';
	import { onMount } from 'svelte';

	// --- Hello (simple query with string input) ---
	let name = $state('World');
	let greeting = $state('');
	let helloLoading = $state(false);

	async function sayHello() {
		helloLoading = true;
		try {
			greeting = await rpc.query('hello', name);
		} catch (e) {
			greeting = `Error: ${e}`;
		} finally {
			helloLoading = false;
		}
	}

	// --- Time (void-input query with struct output) ---
	let time = $state('loading...');

	async function fetchTime() {
		try {
			const res = await rpc.query('time');
			time = new Date(res.timestamp * 1000).toLocaleString();
		} catch (e) {
			time = `Error: ${e}`;
		}
	}

	// --- Status (void-input query with enum in struct) ---
	let status = $state<ServiceStatus | null>(null);
	let statusError = $state('');

	async function fetchStatus() {
		statusError = '';
		try {
			status = await rpc.query('status');
		} catch (e) {
			statusError = `${e}`;
		}
	}

	// --- Math (query with struct input, Result<T, E>, enum) ---
	let mathA = $state(10);
	let mathB = $state(3);
	let mathOp = $state<'Add' | 'Subtract' | 'Multiply' | 'Divide'>('Add');
	let mathResult = $state<MathResult | null>(null);
	let mathError = $state('');
	let mathLoading = $state(false);

	async function calculate() {
		mathLoading = true;
		mathError = '';
		mathResult = null;
		try {
			mathResult = await rpc.query('math', { a: mathA, b: mathB, op: mathOp });
		} catch (e) {
			if (e instanceof RpcError) {
				const data = e.data as { error?: { message?: string } } | undefined;
				mathError = data?.error?.message ?? e.message;
			} else {
				mathError = `${e}`;
			}
		} finally {
			mathLoading = false;
		}
	}

	// --- Stats (query with Vec<f64> input, HashMap output) ---
	let numbersInput = $state('1, 2, 3, 4, 5, 3, 2');
	let statsResult = $state<Stats | null>(null);
	let statsError = $state('');
	let statsLoading = $state(false);

	async function computeStats() {
		statsLoading = true;
		statsError = '';
		statsResult = null;
		try {
			const numbers = numbersInput
				.split(',')
				.map((s) => parseFloat(s.trim()))
				.filter((n) => !isNaN(n));
			statsResult = await rpc.query('stats', numbers);
		} catch (e) {
			if (e instanceof RpcError) {
				const data = e.data as { error?: { message?: string } } | undefined;
				statsError = data?.error?.message ?? e.message;
			} else {
				statsError = `${e}`;
			}
		} finally {
			statsLoading = false;
		}
	}

	// --- Echo (mutation with struct input/output) ---
	let echoMessage = $state('Hello from vercel-rpc!');
	let echoUppercase = $state(false);
	let echoResult = $state<EchoOutput | null>(null);
	let echoLoading = $state(false);

	async function sendEcho() {
		echoLoading = true;
		try {
			echoResult = await rpc.mutate('echo', { message: echoMessage, uppercase: echoUppercase });
		} catch {
			echoResult = null;
		} finally {
			echoLoading = false;
		}
	}

	// --- Profile (serde attributes demo) ---
	let profileId = $state(1);
	let profileResult = $state<UserProfile | null>(null);
	let profileLoading = $state(false);
	let profileError = $state('');

	async function fetchProfile() {
		profileLoading = true;
		profileError = '';
		profileResult = null;
		try {
			profileResult = await rpc.query('profile', profileId);
		} catch (e) {
			profileError = `${e}`;
		} finally {
			profileLoading = false;
		}
	}

	// --- Types (expanded type mappings demo) ---
	let typesCategory = $state('demo');
	let typesResult = $state<TypeShowcase | null>(null);
	let typesLoading = $state(false);
	let typesError = $state('');

	async function fetchTypes() {
		typesLoading = true;
		typesError = '';
		typesResult = null;
		try {
			typesResult = await rpc.query('types', typesCategory);
		} catch (e) {
			typesError = `${e}`;
		} finally {
			typesLoading = false;
		}
	}

	// --- Secret (protected endpoint with RpcClientConfig.headers) ---
	let secretResult = $state('');
	let secretError = $state('');
	let secretLoading = $state(false);

	async function callSecret(withToken: boolean) {
		secretLoading = true;
		secretResult = '';
		secretError = '';
		try {
			const client = createRpcClient(
				withToken
					? { baseUrl: '/api', headers: { Authorization: 'Bearer secret-token-123' } }
					: { baseUrl: '/api' }
			);
			secretResult = await client.query('secret');
		} catch (e) {
			if (e instanceof RpcError) {
				const data = e.data as { error?: { message?: string } } | undefined;
				secretError = data?.error?.message ?? e.message;
			} else {
				secretError = `${e}`;
			}
		} finally {
			secretLoading = false;
		}
	}

	// --- Raw JSON viewer ---
	let rawEndpoint = $state('/api/time');
	let rawResponse = $state('');
	let rawLoading = $state(false);

	async function fetchRaw() {
		rawLoading = true;
		try {
			const res = await fetch(rawEndpoint);
			const json = await res.json();
			rawResponse = JSON.stringify(json, null, 2);
		} catch (e) {
			rawResponse = `Error: ${e}`;
		} finally {
			rawLoading = false;
		}
	}

	// --- Code tabs ---
	let openCode: Record<string, boolean> = $state({});

	function toggleCode(id: string) {
		openCode[id] = !openCode[id];
	}

	onMount(() => {
		fetchTime();
		fetchStatus();
	});
</script>

<div class="container">
	<h1>⚡ vercel-rpc Examples</h1>
	<p class="subtitle">End-to-end typesafe RPC between Rust lambdas and your frontend</p>

	<!-- Type Mapping Reference -->
	<section class="card highlight">
		<h2>📖 Type Mapping Reference</h2>
		<p class="desc">
			Every Rust type is automatically mapped to its TypeScript equivalent during code generation.
		</p>
		<div class="table-wrap">
			<table>
				<thead>
					<tr><th>Rust</th><th>TypeScript</th><th>Example</th></tr>
				</thead>
				<tbody>
					<tr
						><td><code>String</code>, <code>&amp;str</code></td><td><code>string</code></td><td
							>hello endpoint</td
						></tr
					>
					<tr
						><td><code>i32</code>, <code>u64</code>, <code>f64</code></td><td
							><code>number</code></td
						><td>math, time</td></tr
					>
					<tr><td><code>bool</code></td><td><code>boolean</code></td><td>echo (uppercase)</td></tr>
					<tr><td><code>()</code> (no input)</td><td><code>void</code></td><td>time, status</td></tr
					>
					<tr
						><td
							><code>Vec&lt;T&gt;</code>, <code>HashSet&lt;T&gt;</code>,
							<code>BTreeSet&lt;T&gt;</code></td
						><td><code>T[]</code></td><td>stats, types (tags, sorted_ids)</td></tr
					>
					<tr
						><td><code>Option&lt;T&gt;</code></td><td><code>T | null</code></td><td
							>profile (avatarUrl)</td
						></tr
					>
					<tr
						><td><code>HashMap&lt;K, V&gt;</code></td><td><code>Record&lt;K, V&gt;</code></td><td
							>stats (frequencies)</td
						></tr
					>
					<tr
						><td><code>Box&lt;T&gt;</code>, <code>Cow&lt;T&gt;</code></td><td
							><code>T</code> (transparent)</td
						><td>types (boxed_label, cow_message)</td></tr
					>
					<tr
						><td><code>Result&lt;T, E&gt;</code></td><td><code>T</code> (error at runtime)</td><td
							>math, stats</td
						></tr
					>
					<tr
						><td><code>struct</code></td><td><code>interface</code></td><td>TimeResponse, Stats</td
						></tr
					>
					<tr
						><td><code>enum</code> (unit)</td><td><code>"A" | "B"</code></td><td
							>HealthStatus, Operation</td
						></tr
					>
					<tr
						><td><code>serde(rename_all)</code></td><td>field/variant names transformed</td><td
							>profile (camelCase, snake_case, kebab-case)</td
						></tr
					>
					<tr
						><td><code>serde(rename)</code></td><td>exact name override</td><td
							>profile (profile_url, "anonymous")</td
						></tr
					>
					<tr
						><td><code>serde(skip)</code></td><td>field omitted</td><td>profile (internal_score)</td
						></tr
					>
					<tr
						><td><code>serde(default)</code> + <code>Option&lt;T&gt;</code></td><td
							><code>field?: T | null</code></td
						><td>profile (avatarUrl)</td></tr
					>
				</tbody>
			</table>
		</div>
	</section>

	<!-- Hello: Simple string query -->
	<section class="card">
		<h2>🔤 Hello — Simple Query</h2>
		<p class="desc">
			<code>#[rpc_query]</code> with <code>String</code> input → <code>String</code> output. Sent as
			<code>GET /api/hello?input="name"</code>.
		</p>
		<div class="row">
			<input type="text" bind:value={name} placeholder="Enter your name" />
			<button onclick={sayHello} disabled={helloLoading}>
				{helloLoading ? 'Sending...' : 'Say Hello'}
			</button>
		</div>
		{#if greeting}
			<div class="result success">{greeting}</div>
		{/if}
		<pre class="code">rpc.query("hello", "{name}")</pre>
		<button class="toggle-code" onclick={() => toggleCode('hello')}>
			{openCode['hello'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['hello']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/hello.rs</span>
					<pre class="code rust">{`#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`// rpc-types.ts
hello: { input: string; output: string };

// Usage
const greeting = await rpc.query("hello", "World");
//    ^ string — fully typed!`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Time: Void-input query with struct -->
	<section class="card">
		<h2>🕐 Time — Void Input, Struct Output</h2>
		<p class="desc">
			<code>#[rpc_query]</code> with no input → <code>TimeResponse</code> struct. Auto-generated as
			<code>interface TimeResponse</code> with typed fields.
		</p>
		<div class="row">
			<span>Server time: <strong>{time}</strong></span>
			<button onclick={fetchTime}>Refresh</button>
		</div>
		<pre class="code">rpc.query("time") → TimeResponse</pre>
		<button class="toggle-code" onclick={() => toggleCode('time')}>
			{openCode['time'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['time']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/time.rs</span>
					<pre class="code rust">{`#[derive(Serialize)]
pub struct TimeResponse {
    pub timestamp: u64,
    pub message: String,
}

#[rpc_query]
async fn time() -> TimeResponse {
    TimeResponse { timestamp: now, message: "..." }
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`// rpc-types.ts
export interface TimeResponse {
  timestamp: number;  // u64 → number
  message: string;    // String → string
}

// Usage — no input argument needed
const res = await rpc.query("time");
//    ^ TimeResponse — .timestamp, .message`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Status: Enum in struct -->
	<section class="card">
		<h2>🩺 Status — Enum in Struct</h2>
		<p class="desc">
			Returns <code>ServiceStatus</code> with a <code>HealthStatus</code> enum field. Enum maps to
			<code>type HealthStatus = "Healthy" | "Degraded" | "Down"</code>.
		</p>
		{#if status}
			<div class="result success">
				<div class="grid">
					<span class="label">Service:</span><span>{status.name}</span>
					<span class="label">Status:</span><span
						class="badge"
						class:healthy={status.status === 'Healthy'}>{status.status}</span
					>
					<span class="label">Version:</span><span>{status.version}</span>
				</div>
			</div>
		{/if}
		{#if statusError}
			<div class="result error">{statusError}</div>
		{/if}
		<div class="row">
			<button onclick={fetchStatus}>Refresh Status</button>
		</div>
		<pre class="code">rpc.query("status") → ServiceStatus</pre>
		<button class="toggle-code" onclick={() => toggleCode('status')}>
			{openCode['status'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['status']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/status.rs</span>
					<pre class="code rust">{`#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Down,
}

#[derive(Serialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: HealthStatus,
    pub uptime_secs: u64,
    pub version: String,
}

#[rpc_query]
async fn status() -> ServiceStatus { ... }`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`// Unit enum → string literal union
export type HealthStatus = "Healthy" | "Degraded" | "Down";

export interface ServiceStatus {
  name: string;
  status: HealthStatus;  // enum field!
  uptime_secs: number;
  version: string;
}

// Usage
const s = await rpc.query("status");
if (s.status === "Healthy") { ... } // ← autocomplete`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Math: Struct input with enum, Result<T, E> -->
	<section class="card">
		<h2>🧮 Math — Enum Input, Result&lt;T, E&gt;</h2>
		<p class="desc">
			<code>MathInput</code> struct with <code>Operation</code> enum. Returns
			<code>Result&lt;MathResult, String&gt;</code> — try dividing by zero!
		</p>
		<div class="row">
			<input type="number" bind:value={mathA} class="num" />
			<select bind:value={mathOp}>
				<option value="Add">+</option>
				<option value="Subtract">−</option>
				<option value="Multiply">×</option>
				<option value="Divide">÷</option>
			</select>
			<input type="number" bind:value={mathB} class="num" />
			<button onclick={calculate} disabled={mathLoading}>
				{mathLoading ? '...' : '= Calculate'}
			</button>
		</div>
		{#if mathResult}
			<div class="result success">{mathResult.expression}</div>
		{/if}
		{#if mathError}
			<div class="result error">⚠️ {mathError}</div>
		{/if}
		<pre class="code">rpc.query("math", {`{ a: ${mathA}, b: ${mathB}, op: "${mathOp}" }`})</pre>
		<button class="toggle-code" onclick={() => toggleCode('math')}>
			{openCode['math'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['math']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/math.rs</span>
					<pre class="code rust">{`#[derive(Deserialize, Serialize)]
pub enum Operation {
    Add, Subtract, Multiply, Divide,
}

#[derive(Deserialize, Serialize)]
pub struct MathInput {
    pub a: f64, pub b: f64, pub op: Operation,
}

#[derive(Serialize)]
pub struct MathResult {
    pub result: f64, pub expression: String,
}

#[rpc_query]
async fn math(input: MathInput) -> Result<MathResult, String> {
    match input.op {
        Operation::Divide if input.b == 0.0 =>
            Err("Division by zero".to_string()),
        _ => Ok(MathResult { ... })
    }
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`export type Operation = "Add" | "Subtract" | "Multiply" | "Divide";

export interface MathInput {
  a: number;       // f64 → number
  b: number;
  op: Operation;   // enum as input!
}

export interface MathResult {
  result: number;
  expression: string;
}

// Result<T, E> → T (error thrown as RpcError)
try {
  const r = await rpc.query("math", { a: 10, b: 0, op: "Divide" });
} catch (e) {
  if (e instanceof RpcError) { ... } // 400 + JSON error
}`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Stats: Vec input, HashMap output -->
	<section class="card">
		<h2>📊 Stats — Vec&lt;f64&gt; Input, HashMap Output</h2>
		<p class="desc">
			Accepts <code>Vec&lt;f64&gt;</code> (mapped to <code>number[]</code>). Returns
			<code>Stats</code>
			with <code>frequencies: Record&lt;string, number&gt;</code>.
		</p>
		<div class="row">
			<input type="text" bind:value={numbersInput} placeholder="1, 2, 3, 4, 5" class="wide" />
			<button onclick={computeStats} disabled={statsLoading}>
				{statsLoading ? '...' : 'Compute'}
			</button>
		</div>
		{#if statsResult}
			<div class="result success">
				<div class="grid">
					<span class="label">Count:</span><span>{statsResult.count}</span>
					<span class="label">Sum:</span><span>{statsResult.sum}</span>
					<span class="label">Mean:</span><span>{statsResult.mean.toFixed(2)}</span>
					<span class="label">Min:</span><span>{statsResult.min}</span>
					<span class="label">Max:</span><span>{statsResult.max}</span>
					<span class="label">Frequencies:</span>
					<span
						>{Object.entries(statsResult.frequencies)
							.map(([k, v]) => `${k}×${v}`)
							.join(', ')}</span
					>
				</div>
			</div>
		{/if}
		{#if statsError}
			<div class="result error">⚠️ {statsError}</div>
		{/if}
		<pre class="code">rpc.query("stats", [{numbersInput}])</pre>
		<button class="toggle-code" onclick={() => toggleCode('stats')}>
			{openCode['stats'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['stats']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/stats.rs</span>
					<pre class="code rust">{`#[derive(Serialize)]
pub struct Stats {
    pub count: u32,
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub frequencies: HashMap<String, u32>,
}

#[rpc_query]
async fn stats(numbers: Vec<f64>) -> Result<Stats, String> {
    if numbers.is_empty() {
        return Err("Cannot compute stats for empty list".into());
    }
    // ... compute stats
    Ok(Stats { count, sum, mean, min, max, frequencies })
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`export interface Stats {
  count: number;
  sum: number;
  mean: number;
  min: number;
  max: number;
  frequencies: Record<string, number>; // HashMap → Record
}

// Vec<f64> → number[]
const stats = await rpc.query("stats", [1, 2, 3, 4, 5]);
//    ^ Stats — all fields typed
console.log(stats.frequencies); // Record<string, number>`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Echo: Mutation -->
	<section class="card">
		<h2>📤 Echo — Mutation (POST)</h2>
		<p class="desc">
			<code>#[rpc_mutation]</code> — sent as <code>POST /api/echo</code> with JSON body. Accepts
			<code>EchoInput</code>
			(message + uppercase), returns <code>EchoOutput</code>.
		</p>
		<div class="row">
			<input type="text" bind:value={echoMessage} placeholder="Type a message" class="wide" />
			<label class="checkbox">
				<input type="checkbox" bind:checked={echoUppercase} />
				Uppercase
			</label>
			<button onclick={sendEcho} disabled={echoLoading}>
				{echoLoading ? '...' : 'Send'}
			</button>
		</div>
		{#if echoResult}
			<div class="result success">
				<div class="grid">
					<span class="label">Original:</span><span>{echoResult.original}</span>
					<span class="label">Transformed:</span><span
						><strong>{echoResult.transformed}</strong></span
					>
					<span class="label">Length:</span><span>{echoResult.length}</span>
				</div>
			</div>
		{/if}
		<pre class="code">rpc.mutate("echo", {`{ message: "...", uppercase: ${echoUppercase} }`})</pre>
		<button class="toggle-code" onclick={() => toggleCode('echo')}>
			{openCode['echo'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['echo']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/echo.rs</span>
					<pre class="code rust">{`#[derive(Deserialize, Serialize)]
pub struct EchoInput {
    pub message: String,
    pub uppercase: bool,
}

#[derive(Serialize)]
pub struct EchoOutput {
    pub original: String,
    pub transformed: String,
    pub length: u32,
}

#[rpc_mutation]  // ← POST instead of GET
async fn echo(input: EchoInput) -> EchoOutput {
    let transformed = if input.uppercase {
        input.message.to_uppercase()
    } else { input.message.clone() };
    EchoOutput { original: input.message, transformed, length: ... }
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`export interface EchoInput {
  message: string;
  uppercase: boolean;  // bool → boolean
}

export interface EchoOutput {
  original: string;
  transformed: string;
  length: number;      // u32 → number
}

// Mutation uses POST with JSON body
const result = await rpc.mutate("echo", {
  message: "Hello!", uppercase: true
});
//    ^ EchoOutput — fully typed`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Types: Expanded type mappings demo -->
	<section class="card highlight">
		<h2>📦 Types — Expanded Type Mappings</h2>
		<p class="desc">
			Demonstrates expanded type support: <code>HashSet&lt;T&gt;</code> and
			<code>BTreeSet&lt;T&gt;</code> map to <code>T[]</code>, while <code>Box&lt;T&gt;</code> and
			<code>Cow&lt;T&gt;</code> unwrap transparently to <code>T</code>. The generated TypeScript
			matches the actual JSON serialization.
		</p>
		<div class="row">
			<input type="text" bind:value={typesCategory} placeholder="Enter category" />
			<button onclick={fetchTypes} disabled={typesLoading}>
				{typesLoading ? '...' : 'Fetch Types'}
			</button>
		</div>
		{#if typesResult}
			<div class="result success">
				<div class="grid">
					<span class="label">tags:</span><span>{JSON.stringify(typesResult.tags)}</span>
					<span class="label">sorted_ids:</span><span>{JSON.stringify(typesResult.sorted_ids)}</span
					>
					<span class="label">boxed_label:</span><span>{typesResult.boxed_label}</span>
					<span class="label">cow_message:</span><span>{typesResult.cow_message}</span>
				</div>
			</div>
		{/if}
		{#if typesError}
			<div class="result error">{typesError}</div>
		{/if}
		<pre class="code">rpc.query("types", "{typesCategory}") → TypeShowcase</pre>
		<button class="toggle-code" onclick={() => toggleCode('types')}>
			{openCode['types'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['types']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/types.rs</span>
					<pre class="code rust">{`use std::collections::{HashSet, BTreeSet};
use std::borrow::Cow;

#[derive(Serialize)]
pub struct TypeShowcase {
    pub tags: HashSet<String>,         // → string[]
    pub sorted_ids: BTreeSet<i32>,     // → number[]
    pub boxed_label: Box<String>,      // → string
    pub cow_message: Cow<'static, str>, // → string
}

#[rpc_query]
async fn types(category: String) -> TypeShowcase {
    TypeShowcase {
        tags: HashSet::from(["rust", "typescript", "rpc"]),
        sorted_ids: BTreeSet::from([3, 1, 2]),
        boxed_label: Box::new(format!("Category: {}", category)),
        cow_message: Cow::Borrowed("Hello from Cow!"),
    }
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`// HashSet<String> → string[], BTreeSet<i32> → number[]
// Box<String> → string, Cow<str> → string
export interface TypeShowcase {
  tags: string[];        // HashSet<String> → string[]
  sorted_ids: number[];  // BTreeSet<i32> → number[]
  boxed_label: string;   // Box<String> → string
  cow_message: string;   // Cow<str> → string
}

// Usage
const res = await rpc.query("types", "demo");
res.tags       // string[] — not HashSet<string>!
res.sorted_ids // number[] — sorted in JSON output
res.boxed_label // string — Box unwrapped
res.cow_message // string — Cow unwrapped`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Profile: Serde attributes demo -->
	<section class="card highlight">
		<h2>🏷️ Profile — Serde Attributes</h2>
		<p class="desc">
			Demonstrates <code>#[serde(rename_all, rename, skip, default)]</code> on structs and enums. The
			generated TypeScript matches actual JSON serialization — camelCase fields, renamed variants, skipped
			internals, and optional fields.
		</p>
		<div class="row">
			<label>User ID: <input type="number" bind:value={profileId} class="num" /></label>
			<button onclick={fetchProfile} disabled={profileLoading}>
				{profileLoading ? '...' : 'Fetch Profile'}
			</button>
		</div>
		{#if profileResult}
			<div class="result success">
				<div class="grid">
					<span class="label">userId:</span><span>{profileResult.userId}</span>
					<span class="label">displayName:</span><span>{profileResult.displayName}</span>
					<span class="label">emailAddress:</span><span>{profileResult.emailAddress}</span>
					<span class="label">role:</span><span class="badge">{profileResult.role}</span>
					<span class="label">lastEvent:</span><span class="badge">{profileResult.lastEvent}</span>
					<span class="label">profile_url:</span><span>{profileResult.profile_url}</span>
					<span class="label">avatarUrl:</span><span>{profileResult.avatarUrl ?? '(null)'}</span>
				</div>
			</div>
		{/if}
		{#if profileError}
			<div class="result error">{profileError}</div>
		{/if}
		<pre class="code">rpc.query("profile", {profileId}) → UserProfile</pre>
		<button class="toggle-code" onclick={() => toggleCode('profile')}>
			{openCode['profile'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['profile']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/profile.rs</span>
					<pre class="code rust">{`#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Admin,                        // → "admin"
    PowerUser,                    // → "power_user"
    #[serde(rename = "anonymous")]
    Guest,                        // → "anonymous" (override)
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum EventKind {
    SignIn,                       // → "sign-in"
    SignOut,                      // → "sign-out"
    PasswordReset,                // → "password-reset"
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub user_id: u64,             // → userId
    pub display_name: String,     // → displayName

    #[serde(rename = "profile_url")]
    pub profile_url: String,      // → profile_url (override)

    #[serde(skip)]
    pub internal_score: f64,      // → omitted from JSON

    #[serde(default)]
    pub avatar_url: Option<String>, // → avatarUrl?: string | null
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated TypeScript</span>
					<pre class="code ts">{`// rename_all = "snake_case" + variant rename
export type UserRole = "admin" | "power_user" | "anonymous";

// rename_all = "kebab-case"
export type EventKind = "sign-in" | "sign-out" | "password-reset";

// rename_all = "camelCase" + field overrides
export interface UserProfile {
  userId: number;           // rename_all applied
  displayName: string;
  emailAddress: string;
  role: UserRole;
  lastEvent: EventKind;
  profile_url: string;      // #[serde(rename)] wins
  // internal_score omitted  — #[serde(skip)]
  avatarUrl?: string | null; // #[serde(default)] + Option<T>
}`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- JSDoc: Doc comments preserved -->
	<section class="card highlight">
		<h2>📝 JSDoc — Doc Comments Preserved</h2>
		<p class="desc">
			With <code>preserve_docs = true</code> in <code>[codegen]</code>, Rust <code>///</code> doc
			comments are forwarded as JSDoc (<code>/** ... */</code>) in the generated TypeScript. This
			gives you <strong>editor tooltips</strong> and inline documentation on the TypeScript side.
		</p>
		<button class="toggle-code" onclick={() => toggleCode('jsdoc')}>
			{openCode['jsdoc'] ? '▾ Hide' : '▸ Show'} Rust → TypeScript JSDoc
		</button>
		{#if openCode['jsdoc']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/time.rs</span>
					<pre class="code rust">{`/// Server timestamp with a human-readable message.
#[derive(Serialize)]
pub struct TimeResponse {
    pub timestamp: u64,
    pub message: String,
}

/// Returns the current server time as a Unix timestamp.
#[rpc_query]
async fn time() -> TimeResponse { ... }`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated rpc-types.ts</span>
					<pre class="code ts">{`/** Server timestamp with a human-readable message. */
export interface TimeResponse {
  timestamp: number;
  message: string;
}

export type Procedures = {
  queries: {
    /** Returns the current server time as a Unix timestamp. */
    time: { input: void; output: TimeResponse };
  };
};`}</pre>
				</div>
			</div>
			<div class="code-panels" style="margin-top: 0.5rem;">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/hello.rs (multi-line)</span>
					<pre class="code rust">{`/// Greet a user by name.
/// Returns a personalized greeting string.
#[rpc_query]
async fn hello(name: String) -> String { ... }`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 Generated rpc-client.ts</span>
					<pre class="code ts">{`export interface RpcClient {
  /**
   * Greet a user by name.
   * Returns a personalized greeting string.
   */
  query(key: "hello", input: string): Promise<string>;
}`}</pre>
				</div>
			</div>
			<div class="code-panels" style="margin-top: 0.5rem;">
				<div class="code-panel">
					<span class="code-label">⚙️ rpc.config.toml</span>
					<pre class="code rust">{`[codegen]
preserve_docs = true  # default: false`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">💡 What gets documented</span>
					<pre class="code ts">{`/// on a function  → JSDoc on Procedures entry + RpcClient overload
/// on a struct    → JSDoc above export interface
/// on an enum     → JSDoc above export type`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Field Naming: camelCase config -->
	<section class="card highlight">
		<h2>🔤 Field Naming — snake_case to camelCase</h2>
		<p class="desc">
			With <code>fields = "camelCase"</code> in <code>[codegen.naming]</code>, Rust snake_case field
			names are automatically converted to camelCase in generated TypeScript. This matches
			JavaScript conventions while keeping Rust code idiomatic.
		</p>
		<button class="toggle-code" onclick={() => toggleCode('naming')}>
			{openCode['naming'] ? '▾ Hide' : '▸ Show'} Config & Generated Output
		</button>
		{#if openCode['naming']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">⚙️ rpc.config.toml</span>
					<pre class="code rust">{`[codegen.naming]
fields = "camelCase"   # default: "preserve"`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/status.rs</span>
					<pre class="code rust">{`#[derive(Serialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: HealthStatus,
    pub uptime_secs: u64,
    pub version: String,
}

#[derive(Serialize)]
enum Event {
    Click { page_x: i32, page_y: i32 },
}`}</pre>
				</div>
			</div>
			<div class="code-panels" style="margin-top: 0.5rem;">
				<div class="code-panel">
					<span class="code-label">🟦 fields = "preserve" (default)</span>
					<pre class="code ts">{`export interface ServiceStatus {
  name: string;
  status: HealthStatus;
  uptime_secs: number;   // ← kept as-is
  version: string;
}

export type Event = { Click: { page_x: number; page_y: number } };`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 fields = "camelCase"</span>
					<pre class="code ts">{`export interface ServiceStatus {
  name: string;
  status: HealthStatus;
  uptimeSecs: number;    // ← converted!
  version: string;
}

export type Event = { Click: { pageX: number; pageY: number } };`}</pre>
				</div>
			</div>
			<div class="code-panels" style="margin-top: 0.5rem;">
				<div class="code-panel full-width">
					<span class="code-label">💡 What gets transformed</span>
					<pre class="code ts">{`uptime_secs  → uptimeSecs     // struct fields
page_x       → pageX          // enum struct variant fields
api_version  → apiVersion     // multi-segment names
message      → message        // no underscore — unchanged
HealthStatus → HealthStatus   // enum variant names — NOT affected
create_item  → create_item    // procedure names — NOT affected`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Secret: Protected endpoint with RpcClientConfig.headers -->
	<section class="card highlight">
		<h2>🔐 Secret — Protected Endpoint (RpcClientConfig.headers)</h2>
		<p class="desc">
			Demonstrates <code>RpcClientConfig.headers</code> — call a protected endpoint without a token
			(401 error) or with a valid <code>Authorization</code> header (success). The Rust handler uses
			<code>Headers</code> to access the <code>Authorization</code> header and validate the Bearer token.
		</p>
		<div class="row">
			<button onclick={() => callSecret(false)} disabled={secretLoading}>
				{secretLoading ? '...' : 'Call without token'}
			</button>
			<button onclick={() => callSecret(true)} disabled={secretLoading}>
				{secretLoading ? '...' : 'Call with token'}
			</button>
		</div>
		{#if secretResult}
			<div class="result success">{secretResult}</div>
		{/if}
		{#if secretError}
			<div class="result error">{secretError}</div>
		{/if}
		<pre class="code">{`// Without token — 401
const client = createRpcClient({ baseUrl: "/api" });
await client.query("secret"); // throws RpcError

// With token — success
const client = createRpcClient({
  baseUrl: "/api",
  headers: { Authorization: "Bearer secret-token-123" },
});
await client.query("secret"); // "Top secret: the cake is a lie."`}</pre>
		<button class="toggle-code" onclick={() => toggleCode('secret')}>
			{openCode['secret'] ? '▾ Hide' : '▸ Show'} Rust & TypeScript
		</button>
		{#if openCode['secret']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">🦀 Rust — api/secret.rs</span>
					<pre class="code rust">{`/// Access a protected secret.
/// Requires a valid Bearer token in the Authorization header.
#[rpc_query]
async fn secret() -> String {
    "Top secret: the cake is a lie.".to_string()
}`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">🟦 TypeScript — RpcClientConfig.headers</span>
					<pre class="code ts">{`export interface RpcClientConfig {
  baseUrl: string;
  fetch?: typeof globalThis.fetch;
  headers?:
    | Record<string, string>                          // static
    | (() => Record<string, string>                   // sync fn
         | Promise<Record<string, string>>);          // async fn
}

// Static headers
const client = createRpcClient({
  baseUrl: "/api",
  headers: { Authorization: "Bearer token" },
});

// Dynamic headers (e.g. refresh token)
const client = createRpcClient({
  baseUrl: "/api",
  headers: async () => ({
    Authorization: \`Bearer \${await getToken()}\`,
  }),
});`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Raw JSON viewer -->
	<section class="card">
		<h2>🔍 Raw Response Viewer</h2>
		<p class="desc">
			Inspect the raw JSON response from any endpoint. All responses follow the format <code
				>{`{ result: { type: "response", data: ... } }`}</code
			>.
		</p>
		<div class="row">
			<select bind:value={rawEndpoint}>
				<option value="/api/time">GET /api/time</option>
				<option value="/api/status">GET /api/status</option>
				<option value="/api/hello?input=%22World%22">GET /api/hello?input="World"</option>
				<option value="/api/math?input=%7B%22a%22:10,%22b%22:3,%22op%22:%22Add%22%7D"
					>GET /api/math (10+3)</option
				>
				<option value="/api/math?input=%7B%22a%22:10,%22b%22:0,%22op%22:%22Divide%22%7D"
					>GET /api/math (10÷0) — error!</option
				>
				<option value="/api/stats?input=%5B1,2,3,4,5%5D">GET /api/stats ([1,2,3,4,5])</option>
				<option value="/api/types?input=%22demo%22">GET /api/types (expanded types)</option>
				<option value="/api/profile?input=1">GET /api/profile (serde attrs)</option>
				<option value="/api/secret">GET /api/secret (no token — 401)</option>
			</select>
			<button onclick={fetchRaw} disabled={rawLoading}>
				{rawLoading ? '...' : 'Fetch'}
			</button>
		</div>
		{#if rawResponse}
			<pre class="json">{rawResponse}</pre>
		{/if}
	</section>

	<!-- Generated Files Overview -->
	<section class="card highlight">
		<h2>📁 Generated Files</h2>
		<p class="desc">
			These files are auto-generated from the Rust source code in <code>api/</code>.
		</p>
		<button class="toggle-code" onclick={() => toggleCode('generated')}>
			{openCode['generated'] ? '▾ Hide' : '▸ Show'} rpc-types.ts & rpc-client.ts
		</button>
		{#if openCode['generated']}
			<div class="code-panels">
				<div class="code-panel">
					<span class="code-label">rpc-types.ts (with preserve_docs = true)</span>
					<pre class="code ts">{`/** Input for the echo mutation. */
export interface EchoInput {
  message: string;
  uppercase: boolean;
}
/** Output returned by the echo mutation. */
export interface EchoOutput {
  original: string;
  transformed: string;
  length: number;
}
/** Input for a math calculation. */
export interface MathInput {
  a: number;
  b: number;
  op: Operation;
}
/** Result of a math calculation with a formatted expression. */
export interface MathResult {
  result: number;
  expression: string;
}
/** Snapshot of service health and version info. */
export interface ServiceStatus {
  name: string;
  status: HealthStatus;
  uptime_secs: number;
  version: string;
}
/** Descriptive statistics for a list of numbers. */
export interface Stats {
  count: number;
  sum: number;
  mean: number;
  min: number;
  max: number;
  frequencies: Record<string, number>;
}
/** Server timestamp with a human-readable message. */
export interface TimeResponse {
  timestamp: number;
  message: string;
}
/** Overall health of the service. */
export type HealthStatus = "Healthy" | "Degraded" | "Down";
/** Arithmetic operation to perform. */
export type Operation = "Add" | "Subtract" | "Multiply" | "Divide";

// Expanded type mappings (HashSet, BTreeSet, Box, Cow)
export interface TypeShowcase {
  tags: string[];        // HashSet<String>
  sorted_ids: number[];  // BTreeSet<i32>
  boxed_label: string;   // Box<String>
  cow_message: string;   // Cow<str>
}

export type Procedures = {
  queries: {
    /** Greet a user by name. Returns a personalized greeting string. */
    hello: { input: string; output: string };
    /** Perform a math operation. Returns an error on division by zero. */
    math: { input: MathInput; output: MathResult };
    /** Compute descriptive statistics for a list of numbers. */
    stats: { input: number[]; output: Stats };
    /** Returns current service health, uptime, and version. */
    status: { input: void; output: ServiceStatus };
    /** Returns the current server time as a Unix timestamp. */
    time: { input: void; output: TimeResponse };
    /** Return a type showcase demonstrating expanded type mappings. */
    types: { input: string; output: TypeShowcase };
  };
  mutations: {
    /** Echo a message back, optionally transforming it to uppercase. */
    echo: { input: EchoInput; output: EchoOutput };
  };
};`}</pre>
				</div>
				<div class="code-panel">
					<span class="code-label">rpc-client.ts (interface with JSDoc)</span>
					<pre class="code ts">{`export interface RpcClient {
  /** Returns current service health, uptime, and version. */
  query(key: "status"): Promise<ServiceStatus>;
  /** Returns the current server time as a Unix timestamp. */
  query(key: "time"): Promise<TimeResponse>;

  /** Greet a user by name. Returns a personalized greeting string. */
  query(key: "hello", input: string): Promise<string>;
  /** Perform a math operation. Returns an error on division by zero. */
  query(key: "math", input: MathInput): Promise<MathResult>;
  /** Compute descriptive statistics for a list of numbers. */
  query(key: "stats", input: number[]): Promise<Stats>;
  /** Return a type showcase demonstrating expanded type mappings. */
  query(key: "types", input: string): Promise<TypeShowcase>;

  /** Echo a message back, optionally transforming it to uppercase. */
  mutate(key: "echo", input: EchoInput): Promise<EchoOutput>;
}

export function createRpcClient(config: RpcClientConfig): RpcClient;`}</pre>
				</div>
			</div>
		{/if}
	</section>
</div>

<style>
	.container {
		max-width: 720px;
		margin: 2rem auto;
		padding: 0 1rem;
		font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
	}

	h1 {
		margin-bottom: 0.25rem;
	}

	.subtitle {
		color: #666;
		margin-top: 0;
		margin-bottom: 2rem;
		text-align: center;
	}

	.card {
		border: 1px solid #e0e0e0;
		border-radius: 12px;
		padding: 1.25rem;
		margin-bottom: 1.5rem;
		background: #fafafa;
	}

	.card.highlight {
		border-color: #90caf9;
		background: #f5f9ff;
	}

	.card h2 {
		margin-top: 0;
		margin-bottom: 0.5rem;
		font-size: 1.15rem;
	}

	.desc {
		color: #555;
		font-size: 0.9rem;
		margin-bottom: 1rem;
		line-height: 1.5;
	}

	.desc code {
		background: #eee;
		padding: 0.15em 0.4em;
		border-radius: 4px;
		font-size: 0.85em;
	}

	.row {
		display: flex;
		gap: 0.5rem;
		align-items: center;
		flex-wrap: wrap;
	}

	input[type='text'],
	input[type='number'] {
		padding: 0.5rem 0.75rem;
		border: 1px solid #ccc;
		border-radius: 6px;
		font-size: 0.95rem;
	}

	input.num {
		width: 80px;
	}

	input.wide {
		flex: 1;
		min-width: 150px;
	}

	select {
		padding: 0.5rem;
		border: 1px solid #ccc;
		border-radius: 6px;
		font-size: 0.95rem;
		background: white;
	}

	button {
		padding: 0.5rem 1rem;
		border: none;
		border-radius: 6px;
		background: #333;
		color: white;
		cursor: pointer;
		font-size: 0.9rem;
		white-space: nowrap;
	}

	button:hover:not(:disabled) {
		background: #555;
	}

	button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.toggle-code {
		margin-top: 0.75rem;
		background: transparent;
		color: #1976d2;
		border: 1px solid #90caf9;
		font-size: 0.8rem;
		padding: 0.35rem 0.75rem;
	}

	.toggle-code:hover {
		background: #e3f2fd;
		color: #1565c0;
	}

	.checkbox {
		display: flex;
		align-items: center;
		gap: 0.3rem;
		font-size: 0.9rem;
		white-space: nowrap;
	}

	.result {
		margin-top: 0.75rem;
		padding: 0.75rem 1rem;
		border-radius: 8px;
		font-size: 0.95rem;
	}

	.result.success {
		background: #e8f5e9;
		border-left: 4px solid #4caf50;
	}

	.result.error {
		background: #fce4ec;
		border-left: 4px solid #e53935;
		color: #c62828;
	}

	.grid {
		display: grid;
		grid-template-columns: auto 1fr;
		gap: 0.25rem 0.75rem;
	}

	.label {
		font-weight: 600;
		color: #555;
	}

	.badge {
		display: inline-block;
		padding: 0.1em 0.5em;
		border-radius: 4px;
		font-size: 0.85em;
		font-weight: 600;
		background: #eee;
	}

	.badge.healthy {
		background: #c8e6c9;
		color: #2e7d32;
	}

	.code {
		margin-top: 0.75rem;
		padding: 0.5rem 0.75rem;
		background: #263238;
		color: #80cbc4;
		border-radius: 6px;
		font-size: 0.8rem;
		overflow-x: auto;
		white-space: pre;
	}

	.code.rust {
		color: #ffcc80;
	}

	.code.ts {
		color: #90caf9;
	}

	.code-panels {
		display: grid;
		grid-template-columns: 1fr;
		gap: 0.5rem;
		margin-top: 0.5rem;
	}

	.code-panel {
		display: flex;
		flex-direction: column;
	}

	.code-label {
		font-size: 0.75rem;
		font-weight: 600;
		color: #888;
		margin-bottom: 0;
	}

	.code-panel .code {
		margin-top: 0.25rem;
		flex: 1;
	}

	.json {
		margin-top: 0.75rem;
		padding: 0.75rem;
		background: #263238;
		color: #a5d6a7;
		border-radius: 6px;
		font-size: 0.8rem;
		overflow-x: auto;
		max-height: 300px;
		overflow-y: auto;
	}

	.table-wrap {
		overflow-x: auto;
	}

	table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.85rem;
	}

	th,
	td {
		padding: 0.4rem 0.6rem;
		text-align: left;
		border-bottom: 1px solid #e0e0e0;
	}

	th {
		background: #e8eaf6;
		font-weight: 600;
	}

	td code {
		background: #eee;
		padding: 0.1em 0.3em;
		border-radius: 3px;
		font-size: 0.9em;
	}
</style>
