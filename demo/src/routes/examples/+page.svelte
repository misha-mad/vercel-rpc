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

	// --- Echo (mutation with struct input/output) ---
	let echoMessage = $state('Hello from vercel-rpc!');
	let echoUppercase = $state(false);
	const echo = createMutation(rpc, 'echo');

	// --- Profile (reactive query, serde attributes demo) ---
	let profileId = $state(1);
	const profile = createQuery(rpc, 'profile', () => profileId);

	// --- Types (reactive query, expanded type mappings demo) ---
	let typesCategory = $state('demo');
	const types = createQuery(rpc, 'types', () => typesCategory);

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
		<h2>🔤 Hello — Reactive Query</h2>
		<p class="desc">
			<code>createQuery(rpc, "hello", () => name)</code> — auto-refetches as you type. No button needed!
		</p>
		<div class="row">
			<input type="text" bind:value={name} placeholder="Enter your name" />
			<button onclick={() => hello.refetch()} disabled={hello.isLoading}>Refetch</button>
		</div>
		{#if hello.isLoading && !hello.data}
			<div class="result success">Loading...</div>
		{:else if hello.data}
			<div class="result success">{hello.data}</div>
		{/if}
		{#if hello.isError}
			<div class="result error">{hello.error?.message}</div>
		{/if}
		<pre class="code">createQuery(rpc, "hello", () => "{name}")</pre>
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
					<span class="code-label">🟦 Svelte 5 Reactive Wrapper</span>
					<pre class="code ts">{`// rpc.svelte.ts — auto-generated
const hello = createQuery(rpc, "hello", () => name);

// Reactive — updates when 'name' changes
hello.data       // string | undefined
hello.isLoading  // boolean
hello.isError    // boolean
hello.error      // RpcError | undefined
hello.refetch()  // manual refetch`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Time: Void-input query with struct -->
	<section class="card">
		<h2>🕐 Time — Void Input, Struct Output</h2>
		<p class="desc">
			<code>createQuery(rpc, "time")</code> — auto-fetches on mount, no input needed.
		</p>
		<div class="row">
			<span
				>Server time: <strong>
					{#if time.isLoading && !time.data}
						loading...
					{:else if time.data}
						{new Date(time.data.timestamp * 1000).toLocaleString()}
					{/if}
				</strong></span
			>
			<button onclick={() => time.refetch()}>Refresh</button>
		</div>
		<pre class="code">createQuery(rpc, "time") → TimeResponse</pre>
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
const time = createQuery(rpc, "time");
//    ^ QueryResult — .data?.timestamp, .isLoading`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Status: Enum in struct -->
	<section class="card">
		<h2>🩺 Status — Enum in Struct</h2>
		<p class="desc">
			<code>createQuery(rpc, "status")</code> — auto-fetches. <code>HealthStatus</code> enum maps to
			<code>type HealthStatus = "Healthy" | "Degraded" | "Down"</code>.
		</p>
		{#if status.data}
			<div class="result success">
				<div class="grid">
					<span class="label">Service:</span><span>{status.data.name}</span>
					<span class="label">Status:</span><span
						class="badge"
						class:healthy={status.data.status === 'Healthy'}>{status.data.status}</span
					>
					<span class="label">Version:</span><span>{status.data.version}</span>
				</div>
			</div>
		{/if}
		{#if status.isError}
			<div class="result error">{status.error?.message}</div>
		{/if}
		<div class="row">
			<button onclick={() => status.refetch()}>Refresh Status</button>
		</div>
		<pre class="code">createQuery(rpc, "status") → ServiceStatus</pre>
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
const status = createQuery(rpc, "status");
if (status.data?.status === "Healthy") { ... } // ← autocomplete`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Math: Struct input with enum, Result<T, E> -->
	<section class="card">
		<h2>🧮 Math — Reactive Calc, Result&lt;T, E&gt;</h2>
		<p class="desc">
			<code>createQuery(rpc, "math", () => ({'{ a, b, op }'})</code> — recalculates live as you change
			inputs. Try dividing by zero!
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
			{#if math.isLoading}
				<span>...</span>
			{/if}
		</div>
		{#if math.data}
			<div class="result success">{math.data.expression}</div>
		{/if}
		{#if math.isError}
			<div class="result error">
				⚠️ {(math.error?.data as { error?: { message?: string } })?.error?.message ??
					math.error?.message}
			</div>
		{/if}
		<pre
			class="code">createQuery(rpc, "math", () => {`{ a: ${mathA}, b: ${mathB}, op: "${mathOp}" }`})</pre>
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

// Reactive — recalculates when a, b, or op changes
const math = createQuery(rpc, "math", () => ({ a, b, op }));
math.data       // MathResult | undefined
math.isError    // true on division by zero`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Stats: Vec input, HashMap output -->
	<section class="card">
		<h2>📊 Stats — Vec&lt;f64&gt; Input, Enabled Guard</h2>
		<p class="desc">
			<code>createQuery(rpc, "stats", () => parsed, {'{ enabled: () => valid }'})</code> —
			reactively recomputes. The <code>enabled</code> guard prevents fetching with an empty list.
		</p>
		<div class="row">
			<input type="text" bind:value={numbersInput} placeholder="1, 2, 3, 4, 5" class="wide" />
			{#if stats.isLoading}
				<span>...</span>
			{/if}
		</div>
		{#if stats.data}
			<div class="result success">
				<div class="grid">
					<span class="label">Count:</span><span>{stats.data.count}</span>
					<span class="label">Sum:</span><span>{stats.data.sum}</span>
					<span class="label">Mean:</span><span>{stats.data.mean.toFixed(2)}</span>
					<span class="label">Min:</span><span>{stats.data.min}</span>
					<span class="label">Max:</span><span>{stats.data.max}</span>
					<span class="label">Frequencies:</span>
					<span
						>{Object.entries(stats.data.frequencies)
							.map(([k, v]) => `${k}×${v}`)
							.join(', ')}</span
					>
				</div>
			</div>
		{/if}
		{#if stats.isError}
			<div class="result error">
				⚠️ {(stats.error?.data as { error?: { message?: string } })?.error?.message ??
					stats.error?.message}
			</div>
		{/if}
		<pre
			class="code">createQuery(rpc, "stats", () => [{numbersInput}], {'{ enabled: () => len > 0 }'})</pre>
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

// Reactive with enabled guard
const parsed = $derived(input.split(",").map(Number).filter(isFinite));
const stats = createQuery(rpc, "stats", () => parsed, {
  enabled: () => parsed.length > 0,
});`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Echo: Mutation -->
	<section class="card">
		<h2>📤 Echo — Mutation (POST)</h2>
		<p class="desc">
			<code>createMutation(rpc, "echo")</code> — call <code>echo.mutate(...)</code> on button click. Mutations
			are not reactive; they fire explicitly.
		</p>
		<div class="row">
			<input type="text" bind:value={echoMessage} placeholder="Type a message" class="wide" />
			<label class="checkbox">
				<input type="checkbox" bind:checked={echoUppercase} />
				Uppercase
			</label>
			<button
				onclick={() => echo.mutate({ message: echoMessage, uppercase: echoUppercase })}
				disabled={echo.isLoading}
			>
				{echo.isLoading ? '...' : 'Send'}
			</button>
		</div>
		{#if echo.data}
			<div class="result success">
				<div class="grid">
					<span class="label">Original:</span><span>{echo.data.original}</span>
					<span class="label">Transformed:</span><span
						><strong>{echo.data.transformed}</strong></span
					>
					<span class="label">Length:</span><span>{echo.data.length}</span>
				</div>
			</div>
		{/if}
		{#if echo.isError}
			<div class="result error">{echo.error?.message}</div>
		{/if}
		<pre class="code">const echo = createMutation(rpc, "echo");
echo.mutate({`{ message: "...", uppercase: ${echoUppercase} }`})</pre>
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
					<span class="code-label">🟦 Svelte 5 Mutation Wrapper</span>
					<pre class="code ts">{`// createMutation — fire on demand, not reactive
const echo = createMutation(rpc, "echo");

// Call on button click
echo.mutate({ message: "Hello!", uppercase: true });

echo.data       // EchoOutput | undefined
echo.isLoading  // boolean
echo.isError    // boolean
echo.reset()    // clear state`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Types: Expanded type mappings demo -->
	<section class="card highlight">
		<h2>📦 Types — Reactive Expanded Types</h2>
		<p class="desc">
			<code>createQuery(rpc, "types", () => category)</code> — live refetch as you type.
			Demonstrates <code>HashSet</code>, <code>BTreeSet</code>, <code>Box</code>, <code>Cow</code>.
		</p>
		<div class="row">
			<input type="text" bind:value={typesCategory} placeholder="Enter category" />
			<button onclick={() => types.refetch()} disabled={types.isLoading}>Refetch</button>
		</div>
		{#if types.data}
			<div class="result success">
				<div class="grid">
					<span class="label">tags:</span><span>{JSON.stringify(types.data.tags)}</span>
					<span class="label">sorted_ids:</span><span>{JSON.stringify(types.data.sorted_ids)}</span>
					<span class="label">boxed_label:</span><span>{types.data.boxed_label}</span>
					<span class="label">cow_message:</span><span>{types.data.cow_message}</span>
				</div>
			</div>
		{/if}
		{#if types.isError}
			<div class="result error">{types.error?.message}</div>
		{/if}
		<pre class="code">createQuery(rpc, "types", () => "{typesCategory}") → TypeShowcase</pre>
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

// Reactive — refetches when category changes
const types = createQuery(rpc, "types", () => category);
types.data?.tags       // string[]
types.data?.sorted_ids // number[]`}</pre>
				</div>
			</div>
		{/if}
	</section>

	<!-- Profile: Serde attributes demo -->
	<section class="card highlight">
		<h2>🏷️ Profile — Reactive Serde Attributes</h2>
		<p class="desc">
			<code>createQuery(rpc, "profile", () => profileId)</code> — change the ID and see the profile
			update live. Demonstrates <code>#[serde(rename_all, rename, skip, default)]</code>.
		</p>
		<div class="row">
			<label>User ID: <input type="number" bind:value={profileId} class="num" /></label>
			<button onclick={() => profile.refetch()} disabled={profile.isLoading}>Refetch</button>
		</div>
		{#if profile.data}
			<div class="result success">
				<div class="grid">
					<span class="label">userId:</span><span>{profile.data.userId}</span>
					<span class="label">displayName:</span><span>{profile.data.displayName}</span>
					<span class="label">emailAddress:</span><span>{profile.data.emailAddress}</span>
					<span class="label">role:</span><span class="badge">{profile.data.role}</span>
					<span class="label">lastEvent:</span><span class="badge">{profile.data.lastEvent}</span>
					<span class="label">profile_url:</span><span>{profile.data.profile_url}</span>
					<span class="label">avatarUrl:</span><span>{profile.data.avatarUrl ?? '(null)'}</span>
				</div>
			</div>
		{/if}
		{#if profile.isError}
			<div class="result error">{profile.error?.message}</div>
		{/if}
		<pre class="code">createQuery(rpc, "profile", () => {profileId}) → UserProfile</pre>
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
