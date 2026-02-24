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
		<p class="text-text-muted">Documentation coming in next tasks...</p>
	</section>

	<section id="mutations">
		<h2 class="text-2xl font-bold mb-4">Mutations</h2>
		<p class="text-text-muted">Documentation coming in next tasks...</p>
	</section>

	<section id="type-mappings">
		<h2 class="text-2xl font-bold mb-4">Type Mappings</h2>
		<p class="text-text-muted">Documentation coming in next tasks...</p>
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
