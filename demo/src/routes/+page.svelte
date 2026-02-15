<svelte:head>
	<title>vercel-rpc</title>
	<meta name="description" content="End-to-end typesafe RPC between Rust lambdas on Vercel and any frontend" />
</svelte:head>

<section>
	<h1>
		<span class="emoji">⚡</span> vercel-rpc
	</h1>

	<p class="tagline">
		End-to-end typesafe RPC between <strong>Rust lambdas</strong> on Vercel and your frontend.
	</p>

	<p>
		Write plain Rust functions, get a fully typed TypeScript client. Zero config, zero boilerplate.
	</p>

	<div class="features">
		<div class="feature">🦀 <strong>Rust functions</strong> with <code>#[rpc_query]</code> / <code>#[rpc_mutation]</code></div>
		<div class="feature">🔄 <strong>Auto-generated</strong> TypeScript types &amp; client</div>
		<div class="feature">👀 <strong>Watch mode</strong> — types regenerate on every save</div>
		<div class="feature">🚀 <strong>Deploy to Vercel</strong> — each function becomes a lambda</div>
		<div class="feature">🛡️ <strong>Type safety</strong> — Rust → TypeScript, no manual sync</div>
	</div>

	<div class="actions">
		<a href="/examples" class="btn primary">Try the Demo →</a>
		<a href="https://github.com/misha-mad/vercel-rpc" class="btn secondary">GitHub</a>
	</div>
</section>

<style>
	section {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		flex: 0.6;
		text-align: center;
	}

	h1 {
		width: 100%;
		font-size: 2.5rem;
		margin-bottom: 0.5rem;
	}

	.emoji {
		font-size: 2rem;
	}

	.tagline {
		font-size: 1.2rem;
		color: var(--color-text);
		margin-bottom: 0.5rem;
	}

	.features {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
		margin: 1.5rem 0;
		text-align: left;
	}

	.feature {
		font-size: 0.95rem;
	}

	code {
		font-size: 0.85em;
		background: rgba(0, 0, 0, 0.07);
		padding: 0.15em 0.35em;
		border-radius: 3px;
	}

	.actions {
		display: flex;
		gap: 1rem;
		margin-top: 1rem;
	}

	.btn {
		padding: 0.6rem 1.5rem;
		border-radius: 6px;
		text-decoration: none;
		font-weight: 600;
		font-size: 0.95rem;
		transition: opacity 0.2s;
	}

	.btn:hover {
		opacity: 0.85;
	}

	.btn.primary {
		background: var(--color-theme-1);
		color: white;
	}

	.btn.secondary {
		background: rgba(0, 0, 0, 0.08);
		color: var(--color-text);
	}
</style>
