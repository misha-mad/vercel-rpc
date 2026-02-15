<svelte:head>
	<title>About</title>
	<meta name="description" content="About vercel-rpc" />
</svelte:head>

<div class="text-column">
	<h1>About vercel-rpc</h1>

	<p>
		<strong>vercel-rpc</strong> is an end-to-end typesafe RPC toolkit for building serverless APIs
		with Rust on Vercel. It auto-generates TypeScript types and a fully typed client from your Rust
		source code — no manual sync required.
	</p>

	<h2>How it works</h2>
	<ol>
		<li>Annotate Rust functions with <code>#[rpc_query]</code> or <code>#[rpc_mutation]</code></li>
		<li>The CLI scans your <code>api/</code> directory and parses Rust types via <code>syn</code></li>
		<li>TypeScript types and a typed client are generated automatically</li>
		<li>Each Rust file deploys as a serverless lambda on Vercel</li>
	</ol>

	<h2>Tech stack</h2>
	<ul>
		<li><strong>Rust</strong> — proc-macro (<code>syn</code>, <code>quote</code>) + CLI (<code>clap</code>)</li>
		<li><strong>TypeScript</strong> — auto-generated types and client</li>
		<li><strong>Vercel</strong> — serverless Rust lambdas</li>
	</ul>

	<p>
		Check out the <a href="/rpc">RPC demo</a> to see it in action, or visit the
		<a href="https://github.com/misha-mad/vercel-rpc">GitHub repository</a> for the full source code.
	</p>
</div>
