# Demo Site Redesign — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rework the demo site with Tailwind CSS v4, dark theme (vite.dev-inspired), keeping only Home and Docs pages.

**Architecture:** Replace vanilla CSS with Tailwind v4 (CSS-first via `@tailwindcss/vite`). Two pages: Home (landing) and Docs (sidebar + hybrid text/interactive examples). Dark theme only, dual accent colors: orange (Rust) + blue (TypeScript).

**Tech Stack:** SvelteKit, Tailwind CSS v4, Vite 7, Svelte 5

---

### Task 1: Install Tailwind CSS v4

**Files:**
- Modify: `demo/package.json`
- Modify: `demo/vite.config.ts`
- Create: `demo/src/app.css`
- Delete: `demo/src/routes/layout.css`

**Step 1: Install Tailwind CSS and Vite plugin**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npm install tailwindcss @tailwindcss/vite
```

**Step 2: Add Tailwind plugin to vite.config.ts**

Replace `demo/vite.config.ts` with:

```typescript
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit()]
});
```

**Step 3: Create `demo/src/app.css` with Tailwind imports and theme**

```css
@import 'tailwindcss';
@import '@fontsource/fira-mono';

@theme {
	--font-body: Arial, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu,
		Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
	--font-mono: 'Fira Mono', monospace;

	--color-bg-primary: #1b1b1f;
	--color-bg-soft: #222227;
	--color-bg-sidebar: #161618;
	--color-bg-header: #0f0f11;
	--color-bg-code: #1e1e2e;

	--color-text-primary: rgba(255, 255, 255, 0.87);
	--color-text-muted: rgba(255, 255, 255, 0.6);
	--color-text-faint: rgba(255, 255, 255, 0.38);

	--color-accent-rust: #f74c00;
	--color-accent-ts: #3178c6;

	--color-border: rgba(255, 255, 255, 0.08);
	--color-border-hover: rgba(255, 255, 255, 0.16);
}
```

**Step 4: Update `demo/src/routes/+layout.svelte` to import `app.css` instead of `layout.css`**

```svelte
<script lang="ts">
	import Header from './Header.svelte';
	import '../app.css';

	let { children } = $props();
</script>

<div class="min-h-screen bg-bg-primary text-text-primary font-body">
	<Header />
	<main>
		{@render children()}
	</main>
	<footer class="text-center py-4 text-text-muted text-sm border-t border-border">
		<p>
			visit <a href="https://github.com/misha-mad/vercel-rpc" class="text-accent-ts hover:underline font-bold">GitHub</a> to learn more about vercel-rpc
		</p>
	</footer>
</div>
```

**Step 5: Delete old layout.css**

```bash
rm demo/src/routes/layout.css
```

**Step 6: Verify dev server starts without errors**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

Expected: Build succeeds (pages will look broken — that's fine, we'll fix them next).

**Step 7: Commit**

```bash
git add demo/package.json demo/package-lock.json demo/vite.config.ts demo/src/app.css demo/src/routes/+layout.svelte
git rm demo/src/routes/layout.css
git commit -m "chore(rpc): replace vanilla CSS with Tailwind CSS v4"
```

---

### Task 2: Rewrite Header component

**Files:**
- Modify: `demo/src/routes/Header.svelte`

**Step 1: Rewrite Header with Tailwind classes, dark theme, 2 nav items (Home, Docs)**

Replace `demo/src/routes/Header.svelte` with:

```svelte
<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import github from '$lib/images/github.svg';
</script>

<header class="sticky top-0 z-50 border-b border-border bg-bg-header/80 backdrop-blur-md">
	<div class="mx-auto flex h-14 max-w-7xl items-center justify-between px-4 sm:px-6">
		<a href={resolve('/')} class="flex items-center gap-2 text-lg font-bold text-text-primary hover:text-accent-rust transition-colors">
			<span class="text-2xl">⚡</span>
			<span>vercel-rpc</span>
		</a>

		<nav class="flex items-center gap-6">
			<a
				href={resolve('/')}
				class="text-sm font-medium transition-colors {page.url.pathname === '/' ? 'text-accent-rust' : 'text-text-muted hover:text-text-primary'}"
			>
				Home
			</a>
			<a
				href={resolve('/docs')}
				class="text-sm font-medium transition-colors {page.url.pathname.startsWith('/docs') ? 'text-accent-rust' : 'text-text-muted hover:text-text-primary'}"
			>
				Docs
			</a>
			<a
				href="https://github.com/misha-mad/vercel-rpc"
				class="flex items-center text-text-muted hover:text-text-primary transition-colors"
				target="_blank"
				rel="noopener"
			>
				<img src={github} alt="GitHub" class="h-5 w-5 invert opacity-70 hover:opacity-100" />
			</a>
		</nav>
	</div>
</header>
```

**Step 2: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add demo/src/routes/Header.svelte
git commit -m "chore(rpc): rewrite Header with Tailwind dark theme"
```

---

### Task 3: Rewrite Home page

**Files:**
- Modify: `demo/src/routes/+page.svelte`

**Step 1: Rewrite Home with Tailwind — hero, features grid, dual-accent CTAs**

Replace `demo/src/routes/+page.svelte` with:

```svelte
<svelte:head>
	<title>vercel-rpc</title>
	<meta name="description" content="End-to-end typesafe RPC between Rust lambdas on Vercel and any frontend" />
</svelte:head>

<section class="flex flex-col items-center justify-center px-4 py-20 text-center">
	<h1 class="text-4xl sm:text-5xl font-bold mb-4">
		<span class="text-3xl sm:text-4xl">⚡</span> vercel-rpc
	</h1>

	<p class="text-lg sm:text-xl text-text-muted max-w-2xl mb-2">
		End-to-end typesafe RPC between <strong class="text-accent-rust">Rust lambdas</strong> on Vercel and your frontend.
	</p>

	<p class="text-text-muted max-w-xl mb-8">
		Write plain Rust functions, get a fully typed TypeScript client. Zero config, zero boilerplate.
	</p>

	<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-10 max-w-xl text-left text-sm">
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span class="text-accent-rust">🦀</span>
			<span><strong>Rust functions</strong> with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_query]</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_mutation]</code></span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span>🔄</span>
			<span><strong>Auto-generated</strong> TypeScript types & client</span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span>👀</span>
			<span><strong>Watch mode</strong> — types regenerate on every save</span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
			<span>🚀</span>
			<span><strong>Deploy to Vercel</strong> — each function becomes a lambda</span>
		</div>
		<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3 sm:col-span-2 sm:max-w-[calc(50%-0.375rem)]">
			<span>🛡️</span>
			<span><strong>Type safety</strong> — Rust → TypeScript, no manual sync</span>
		</div>
	</div>

	<div class="flex gap-4">
		<a href="/docs" class="rounded-lg bg-accent-rust px-6 py-2.5 font-semibold text-white text-sm transition-opacity hover:opacity-85">
			Get Started →
		</a>
		<a href="https://github.com/misha-mad/vercel-rpc" class="rounded-lg border border-accent-ts px-6 py-2.5 font-semibold text-accent-ts text-sm transition-opacity hover:opacity-85">
			GitHub
		</a>
	</div>
</section>
```

**Step 2: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add demo/src/routes/+page.svelte
git commit -m "chore(rpc): rewrite Home page with dark theme"
```

---

### Task 4: Delete old pages (about, examples)

**Files:**
- Delete: `demo/src/routes/about/+page.svelte`
- Delete: `demo/src/routes/about/+page.ts`
- Delete: `demo/src/routes/examples/+page.svelte`

**Step 1: Remove about and examples directories**

```bash
rm -rf demo/src/routes/about demo/src/routes/examples
```

**Step 2: Verify build still works**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add -A demo/src/routes/about demo/src/routes/examples
git commit -m "chore(rpc): remove about and examples pages"
```

---

### Task 5: Create Docs page — layout with sidebar

**Files:**
- Create: `demo/src/routes/docs/+layout.svelte`
- Create: `demo/src/routes/docs/+page.svelte`

This task creates the docs layout shell with sidebar navigation and the main docs page. The sidebar has anchor links to sections. All docs content lives on a single `/docs` page with scrollable sections (not separate routes).

**Step 1: Create `demo/src/routes/docs/+layout.svelte`**

```svelte
<script lang="ts">
	let { children } = $props();

	const sections = [
		{ id: 'getting-started', label: 'Getting Started' },
		{ id: 'queries', label: 'Queries' },
		{ id: 'mutations', label: 'Mutations' },
		{ id: 'type-mappings', label: 'Type Mappings' },
		{ id: 'error-handling', label: 'Error Handling' },
		{ id: 'streaming', label: 'Streaming' }
	];

	let sidebarOpen = $state(false);
</script>

<div class="mx-auto flex max-w-7xl">
	<!-- Mobile toggle -->
	<button
		class="fixed bottom-4 right-4 z-50 flex h-10 w-10 items-center justify-center rounded-lg bg-accent-rust text-white shadow-lg lg:hidden"
		onclick={() => (sidebarOpen = !sidebarOpen)}
	>
		{sidebarOpen ? '✕' : '☰'}
	</button>

	<!-- Sidebar -->
	<aside
		class="fixed top-14 left-0 z-40 h-[calc(100vh-3.5rem)] w-60 overflow-y-auto border-r border-border bg-bg-sidebar p-4 transition-transform lg:sticky lg:translate-x-0 {sidebarOpen ? 'translate-x-0' : '-translate-x-full'}"
	>
		<nav class="flex flex-col gap-1">
			{#each sections as section}
				<a
					href="#{section.id}"
					class="rounded-md px-3 py-1.5 text-sm text-text-muted transition-colors hover:bg-bg-soft hover:text-text-primary"
					onclick={() => (sidebarOpen = false)}
				>
					{section.label}
				</a>
			{/each}
		</nav>
	</aside>

	<!-- Backdrop for mobile -->
	{#if sidebarOpen}
		<button
			class="fixed inset-0 z-30 bg-black/50 lg:hidden"
			onclick={() => (sidebarOpen = false)}
			aria-label="Close sidebar"
		></button>
	{/if}

	<!-- Content -->
	<div class="min-w-0 flex-1 px-4 py-8 sm:px-8 lg:pl-8">
		{@render children()}
	</div>
</div>
```

**Step 2: Create `demo/src/routes/docs/+page.svelte` (scaffold with section headings)**

```svelte
<svelte:head>
	<title>Docs — vercel-rpc</title>
	<meta name="description" content="vercel-rpc documentation" />
</svelte:head>

<div class="max-w-3xl space-y-16">
	<section id="getting-started">
		<h1 class="text-3xl font-bold mb-4">Getting Started</h1>
		<p class="text-text-muted">Documentation coming in next tasks...</p>
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
```

**Step 3: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 4: Commit**

```bash
git add demo/src/routes/docs/
git commit -m "feat(rpc): add docs page layout with sidebar navigation"
```

---

### Task 6: Populate Docs — Getting Started section

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte`

**Step 1: Replace the "getting-started" section placeholder with actual content**

The Getting Started section should include:
- What vercel-rpc is (from current about page content)
- How it works (3-step explanation)
- Quick code example showing a Rust function + generated TS client

Content reference: merge `demo/src/routes/about/+page.svelte` about text with overview from home.

Use Tailwind classes for code blocks (`bg-bg-code rounded-lg p-4 font-mono text-sm overflow-x-auto`), headings, and prose spacing.

**Step 2: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/+page.svelte
git commit -m "docs(rpc): add Getting Started section content"
```

---

### Task 7: Populate Docs — Queries section with interactive examples

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte`

**Step 1: Add imports and reactive state for query examples at the top of `<script>`**

Migrate from `examples/+page.svelte`:
- `hello` query (string input, reactive)
- `time` query (void input, struct output)
- `status` query (enum in struct)
- `math` query (struct input, Result<T,E>)
- `stats` query (Vec input, enabled guard)
- `profile` query (serde attributes)
- `types` query (expanded type mappings)
- `openCode` toggle state

**Step 2: Replace "queries" section placeholder with documentation + interactive demos**

Each query example should have:
- Text description explaining the pattern
- Live interactive demo (input + result display)
- Expandable Rust/TypeScript code panels

Use Tailwind card styles: `rounded-lg border border-border bg-bg-soft p-6`

**Step 3: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 4: Commit**

```bash
git add demo/src/routes/docs/+page.svelte
git commit -m "docs(rpc): add Queries section with interactive examples"
```

---

### Task 8: Populate Docs — Mutations section with interactive example

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte`

**Step 1: Add echo mutation state (already imported in Task 7)**

Migrate from `examples/+page.svelte`:
- `echo` mutation (struct input/output)

**Step 2: Replace "mutations" section placeholder with documentation + interactive demo**

Explain `createMutation` vs `createQuery`. Show echo example with:
- Text input + uppercase checkbox
- Mutate button + loading/result display
- Expandable Rust/TypeScript code

**Step 3: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 4: Commit**

```bash
git add demo/src/routes/docs/+page.svelte
git commit -m "docs(rpc): add Mutations section with interactive echo example"
```

---

### Task 9: Populate Docs — Type Mappings section

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte`

**Step 1: Replace "type-mappings" section with the type mapping reference table**

Migrate the full type mapping table from `examples/+page.svelte` (the `<table>` inside the "Type Mapping Reference" card). Style with Tailwind:
- Table: `w-full text-sm text-left`
- Header: `bg-bg-code text-text-muted`
- Cells: `border-b border-border px-4 py-2`
- Code: `bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono`

**Step 2: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/+page.svelte
git commit -m "docs(rpc): add Type Mappings reference table"
```

---

### Task 10: Populate Docs — Error Handling section with interactive example

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte`

**Step 1: Add secret endpoint state (from examples page)**

Migrate:
- `callSecret()` function
- `secretResult`, `secretError`, `secretLoading` state

**Step 2: Replace "error-handling" section with documentation + secret endpoint demo**

Explain `RpcError`, error shapes, how `Result<T, E>` errors propagate. Show:
- Two buttons: "Call with token" / "Call without token"
- Success/error result display
- Expandable Rust/TypeScript code

**Step 3: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 4: Commit**

```bash
git add demo/src/routes/docs/+page.svelte
git commit -m "docs(rpc): add Error Handling section with interactive example"
```

---

### Task 11: Populate Docs — Streaming section

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte`

**Step 1: Replace "streaming" section with documentation about `#[rpc_stream]`**

This section is documentation-only (no interactive demo needed unless there's a streaming endpoint in the demo API). Describe:
- What `#[rpc_stream]` does
- How Server-Sent Events work
- Code example showing Rust stream function + TypeScript consumer

Reference the project's existing documentation on streaming from the recent commit `4a64419`.

**Step 2: Verify build**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add demo/src/routes/docs/+page.svelte
git commit -m "docs(rpc): add Streaming section documentation"
```

---

### Task 12: Remove unused dependencies and clean up

**Files:**
- Modify: `demo/package.json`

**Step 1: Remove `@neoconfetti/svelte` (was only used in SvelteKit template, not in our pages)**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npm uninstall @neoconfetti/svelte
```

**Step 2: Verify build one final time**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite build 2>&1 | head -20
```

**Step 3: Commit**

```bash
git add demo/package.json demo/package-lock.json
git commit -m "chore(rpc): remove unused @neoconfetti/svelte dependency"
```

---

### Task 13: Visual review and polish

**Files:**
- Potentially modify any of the files from previous tasks

**Step 1: Start the dev server**

```bash
cd /Users/mikhailzakharov/RustroverProjects/svelte-rust/demo && npx vite dev --port 5173
```

**Step 2: Review each page visually**

- `/` — Check hero layout, feature cards, CTA buttons, header, footer
- `/docs` — Check sidebar, section content, interactive examples, code blocks, table
- Mobile responsiveness — sidebar collapse, stacking

**Step 3: Fix any visual issues discovered**

Adjust spacing, colors, responsiveness as needed.

**Step 4: Commit fixes**

```bash
git add -A demo/src/
git commit -m "style(rpc): polish demo site visual appearance"
```
