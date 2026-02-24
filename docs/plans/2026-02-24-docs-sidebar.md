# Docs Sidebar Redesign Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the flat anchor-based docs sidebar with a multi-page hierarchical navigation — each item opens its own route, groups expand/collapse, active page is highlighted.

**Architecture:** Rewrite `docs/+layout.svelte` with a nav data structure supporting groups and leaf items. Each section becomes a SvelteKit file-based route under `docs/`. The current monolithic `docs/+page.svelte` content gets split across individual page files. Active state is derived from `page.url.pathname`. Groups auto-expand when a child is active.

**Tech Stack:** SvelteKit file-based routing, Svelte 5 runes ($state, $derived), svelte/transition (slide), Tailwind CSS v4, existing `CodeBlock` component + `highlightCode` server helper.

**Design doc:** `docs/plans/2026-02-24-docs-sidebar-design.md`

---

### Task 1: Rewrite docs layout with hierarchical sidebar

**Files:**
- Modify: `demo/src/routes/docs/+layout.svelte`

**Step 1: Replace the layout with the new sidebar**

Replace the entire file with:

```svelte
<script lang="ts">
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import { slide } from 'svelte/transition';

	let { children } = $props();

	type NavItem = { label: string; href: string; badge?: string };
	type NavGroup = { label: string; children: NavItem[] };
	type NavEntry = NavItem | NavGroup;

	const nav: NavEntry[] = [
		{ label: 'Getting Started', href: '/docs/getting-started' },
		{ label: 'Procedures', children: [
			{ label: 'Queries', href: '/docs/procedures/queries' },
			{ label: 'Mutations', href: '/docs/procedures/mutations' },
			{ label: 'Streaming', href: '/docs/procedures/streaming', badge: 'soon' },
		]},
		{ label: 'Configuration', children: [
			{ label: 'rpc.config.toml', href: '/docs/configuration/config-file' },
			{ label: 'CLI', href: '/docs/configuration/cli' },
			{ label: 'Macro Attributes', href: '/docs/configuration/macro-attributes' },
			{ label: 'RpcClientConfig', href: '/docs/configuration/client-config' },
			{ label: 'Per-Call Options', href: '/docs/configuration/per-call-options' },
		]},
		{ label: 'Type System', children: [
			{ label: 'Type Mappings', href: '/docs/type-system/type-mappings' },
			{ label: 'Serde Support', href: '/docs/type-system/serde' },
			{ label: 'Generics', href: '/docs/type-system/generics' },
			{ label: 'Branded Newtypes', href: '/docs/type-system/branded-newtypes' },
		]},
		{ label: 'Frameworks', children: [
			{ label: 'Svelte 5', href: '/docs/frameworks/svelte' },
			{ label: 'React', href: '/docs/frameworks/react' },
			{ label: 'Vue 3', href: '/docs/frameworks/vue' },
			{ label: 'SolidJS', href: '/docs/frameworks/solid' },
		]},
		{ label: 'Error Handling', href: '/docs/error-handling' },
	];

	function isGroup(entry: NavEntry): entry is NavGroup {
		return 'children' in entry;
	}

	function isActive(href: string): boolean {
		return page.url.pathname === href;
	}

	function hasActiveChild(group: NavGroup): boolean {
		return group.children.some((child) => isActive(child.href));
	}

	// Track manually toggled groups. Key = group label, value = open/closed override.
	let toggleOverrides: Record<string, boolean> = $state({});

	function isGroupOpen(group: NavGroup): boolean {
		if (group.label in toggleOverrides) return toggleOverrides[group.label];
		return hasActiveChild(group);
	}

	function toggleGroup(group: NavGroup) {
		const current = isGroupOpen(group);
		toggleOverrides[group.label] = !current;
	}

	let sidebarOpen = $state(false);

	$effect(() => {
		document.body.style.overflow = sidebarOpen ? 'hidden' : '';
		return () => { document.body.style.overflow = ''; };
	});

	// Reset manual overrides on navigation (so auto-expand takes over)
	$effect(() => {
		page.url.pathname;
		toggleOverrides = {};
	});
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
		class="fixed top-14 left-0 z-40 h-[calc(100vh-3.5rem)] w-60 overflow-y-auto border-r border-border bg-bg-sidebar p-4 transition-transform lg:translate-x-0 {sidebarOpen ? 'translate-x-0' : '-translate-x-full'}"
	>
		<nav class="flex flex-col gap-0.5">
			{#each nav as entry}
				{#if isGroup(entry)}
					<button
						class="flex w-full items-center justify-between rounded-md px-3 py-1.5 text-sm font-medium text-text-muted transition-colors hover:bg-bg-soft hover:text-text-primary"
						onclick={() => toggleGroup(entry)}
					>
						{entry.label}
						<span class="text-[10px] text-text-faint transition-transform {isGroupOpen(entry) ? 'rotate-90' : ''}"
							>▶</span
						>
					</button>
					{#if isGroupOpen(entry)}
						<div transition:slide={{ duration: 150 }} class="ml-2 flex flex-col gap-0.5 border-l border-border pl-2">
							{#each entry.children as child}
								<a
									href={resolve(child.href)}
									class="flex items-center gap-2 rounded-md px-3 py-1 text-sm transition-colors {isActive(child.href)
										? 'bg-bg-soft text-text-primary border-l-2 border-accent-rust -ml-[2px] pl-[10px]'
										: 'text-text-muted hover:bg-bg-soft hover:text-text-primary'}"
									onclick={() => (sidebarOpen = false)}
								>
									{child.label}
									{#if child.badge}
										<span class="rounded-full bg-accent-rust/20 text-accent-rust text-[10px] px-1.5"
											>{child.badge}</span
										>
									{/if}
								</a>
							{/each}
						</div>
					{/if}
				{:else}
					<a
						href={resolve(entry.href)}
						class="rounded-md px-3 py-1.5 text-sm transition-colors {isActive(entry.href)
							? 'bg-bg-soft text-text-primary border-l-2 border-accent-rust -ml-[2px] pl-[10px] font-medium'
							: 'text-text-muted hover:bg-bg-soft hover:text-text-primary'}"
						onclick={() => (sidebarOpen = false)}
					>
						{entry.label}
					</a>
				{/if}
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
	<div class="min-w-0 flex-1 px-4 py-8 sm:px-8 lg:ml-60 lg:pl-8">
		{@render children()}
	</div>
</div>
```

**Step 2: Verify build compiles**

Run: `cd demo && npm run build 2>&1 | head -20`
Expected: No Svelte/TS compilation errors in `+layout.svelte`

**Step 3: Commit**

```bash
git add demo/src/routes/docs/+layout.svelte
git commit -m "feat(rpc): rewrite docs sidebar with hierarchical nav and active state"
```

---

### Task 2: Add docs root redirect and getting-started page

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte` (replace with redirect)
- Create: `demo/src/routes/docs/getting-started/+page.server.ts`
- Create: `demo/src/routes/docs/getting-started/+page.svelte`

**Step 1: Replace docs root page with redirect**

Replace `demo/src/routes/docs/+page.svelte` entirely:

```svelte
<script lang="ts">
	import { redirect } from '@sveltejs/kit';
	import { resolve } from '$app/paths';
</script>

<script context="module" lang="ts">
	import { redirect as sRedirect } from '@sveltejs/kit';
</script>
```

Actually, the correct SvelteKit way to redirect from a page is via `+page.server.ts` or `+page.ts` load function. Replace the approach:

Replace `demo/src/routes/docs/+page.server.ts` with a redirect:

```ts
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = () => {
	redirect(301, '/docs/getting-started');
};
```

Replace `demo/src/routes/docs/+page.svelte` with an empty placeholder (SvelteKit requires it if +page.server.ts exists):

```svelte
```

(Empty file — the redirect fires before it renders.)

**Step 2: Create getting-started page server**

Create `demo/src/routes/docs/getting-started/+page.server.ts`:

```ts
import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	gettingStartedRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`
	},
	gettingStartedTs: {
		lang: 'typescript',
		code: `import { createRpcClient } from './rpc-client';

const rpc = createRpcClient({ baseUrl: '/api' });
const greeting = await rpc.query('hello', 'World');
// greeting: string — "Hello, World from Rust on Vercel!"`
	},
	gettingStartedSvelte: {
		lang: 'typescript',
		code: `import { createQuery } from './rpc.svelte';

let name = $state('World');
const hello = createQuery(rpc, 'hello', () => name);
// hello.data reactively updates when 'name' changes`
	}
};

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(
		entries.map(([, { code, lang }]) => highlightCode(code, lang))
	);
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
```

**Step 3: Create getting-started page**

Create `demo/src/routes/docs/getting-started/+page.svelte`:

```svelte
<script lang="ts">
	import CodeBlock from '$lib/components/CodeBlock.svelte';

	let { data } = $props();
</script>

<svelte:head>
	<title>Getting Started — vercel-rpc</title>
</svelte:head>

<div class="max-w-3xl space-y-8">
	<h1 class="text-3xl font-bold">Getting Started</h1>

	<p class="text-text-muted leading-relaxed">
		<strong class="text-text-primary">vercel-rpc</strong> is an end-to-end typesafe RPC toolkit for building
		serverless APIs with Rust on Vercel. Write plain Rust functions, and get a fully typed TypeScript
		client — no manual sync required.
	</p>

	<h3 class="text-xl font-semibold mt-8">How it works</h3>

	<ol class="list-decimal list-inside space-y-2 text-text-muted">
		<li>Annotate Rust functions with <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_query]</code> or <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">#[rpc_mutation]</code></li>
		<li>The CLI scans your <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">api/</code> directory and parses Rust types via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">syn</code></li>
		<li>TypeScript types and a typed client are generated automatically</li>
		<li>Each Rust file deploys as a serverless lambda on Vercel</li>
	</ol>

	<h3 class="text-xl font-semibold mt-8">Quick example</h3>

	<p class="text-text-muted text-sm mb-2">Rust — <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">api/hello.rs</code></p>
	<CodeBlock html={data.highlighted['gettingStartedRust']} large />

	<p class="text-text-muted text-sm mb-2">Generated TypeScript client</p>
	<CodeBlock html={data.highlighted['gettingStartedTs']} large />

	<p class="text-text-muted text-sm mb-2">Or with Svelte 5 reactive wrapper</p>
	<CodeBlock html={data.highlighted['gettingStartedSvelte']} large />
</div>
```

**Step 4: Verify build**

Run: `cd demo && npm run build 2>&1 | head -30`
Expected: Compiles, `/docs` redirects to `/docs/getting-started`

**Step 5: Commit**

```bash
git add demo/src/routes/docs/+page.svelte demo/src/routes/docs/+page.server.ts demo/src/routes/docs/getting-started/
git commit -m "feat(rpc): add getting-started page and docs root redirect"
```

---

### Task 3: Create Procedures pages (queries, mutations, streaming)

**Files:**
- Create: `demo/src/routes/docs/procedures/queries/+page.server.ts`
- Create: `demo/src/routes/docs/procedures/queries/+page.svelte`
- Create: `demo/src/routes/docs/procedures/mutations/+page.server.ts`
- Create: `demo/src/routes/docs/procedures/mutations/+page.svelte`
- Create: `demo/src/routes/docs/procedures/streaming/+page.server.ts`
- Create: `demo/src/routes/docs/procedures/streaming/+page.svelte`

**Context:** Move the existing interactive demos from the old `+page.svelte`:
- Queries page: gets Hello, Time, Status, Math, Stats, Profile demos
- Mutations page: gets Echo demo
- Streaming page: gets the "coming soon" streaming section

Each page needs its own `+page.server.ts` with the relevant code blocks for highlighting, and `+page.svelte` with the content. Copy the relevant sections from the existing `demo/src/routes/docs/+page.svelte` (lines 117-285 for queries, 287-340 for mutations, 494-527 for streaming) and their corresponding code blocks from `demo/src/routes/docs/+page.server.ts`.

The Svelte pages need `import { rpc } from '$lib/client'`, `import { createQuery, createMutation, RpcError } from '$lib/rpc.svelte'` etc. as needed per page. Reuse existing code — just split, don't rewrite.

**Step 1: Create queries page server**

Create `demo/src/routes/docs/procedures/queries/+page.server.ts` with code blocks: `helloRust`, `helloTs`, `timeRust`, `timeTs`, `statusRust`, `statusTs` (copy from existing `+page.server.ts` lines 31-97).

**Step 2: Create queries page**

Create `demo/src/routes/docs/procedures/queries/+page.svelte` — copy the "Queries" section content (lines 117-285 of old `+page.svelte`), wrapped with `<svelte:head><title>Queries — vercel-rpc</title></svelte:head>` and `<div class="max-w-3xl space-y-8">`.

**Step 3: Create mutations page server and page**

Same pattern — `echoRust`, `echoTs` code blocks, Echo demo section (lines 287-340).

**Step 4: Create streaming page server and page**

`streamRust`, `streamTs` code blocks, streaming content (lines 494-527).

**Step 5: Verify build**

Run: `cd demo && npm run build 2>&1 | head -30`
Expected: All three pages compile

**Step 6: Commit**

```bash
git add demo/src/routes/docs/procedures/
git commit -m "feat(rpc): add procedures pages (queries, mutations, streaming)"
```

---

### Task 4: Create Type System pages

**Files:**
- Create: `demo/src/routes/docs/type-system/type-mappings/+page.svelte`
- Create: `demo/src/routes/docs/type-system/serde/+page.svelte`
- Create: `demo/src/routes/docs/type-system/generics/+page.svelte`
- Create: `demo/src/routes/docs/type-system/branded-newtypes/+page.svelte`

**Context:**
- Type Mappings: move the existing table from old `+page.svelte` (lines 342-436)
- Serde, Generics, Branded Newtypes: these are new pages — create placeholder content explaining each feature with code examples. Reference the project's existing RFC docs and feature descriptions for accurate content. These pages don't need interactive demos initially, just code blocks explaining the feature.

**Step 1: Create type-mappings page**

Move the existing type mappings table. No `+page.server.ts` needed if using inline `<code>` tags (no shiki highlighting). If code blocks are desired, add server file.

**Step 2: Create serde page (placeholder)**

Explain rename_all, rename, skip, flatten, enum tagging. Use code examples.

**Step 3: Create generics and branded-newtypes pages (placeholders)**

Brief explanations with code examples.

**Step 4: Verify build**

Run: `cd demo && npm run build 2>&1 | head -30`

**Step 5: Commit**

```bash
git add demo/src/routes/docs/type-system/
git commit -m "feat(rpc): add type system pages (mappings, serde, generics, branded)"
```

---

### Task 5: Create Configuration pages

**Files:**
- Create: `demo/src/routes/docs/configuration/config-file/+page.svelte`
- Create: `demo/src/routes/docs/configuration/cli/+page.svelte`
- Create: `demo/src/routes/docs/configuration/macro-attributes/+page.svelte`
- Create: `demo/src/routes/docs/configuration/client-config/+page.svelte`
- Create: `demo/src/routes/docs/configuration/per-call-options/+page.svelte`

**Context:** These are all new pages. Content should reference existing project features:
- **config-file**: `rpc.config.toml` format — input/output/codegen/watch sections
- **CLI**: `rpc generate` and `rpc watch` commands with all flags
- **macro-attributes**: `cache`, `init`, `timeout`, `idempotent` attributes
- **client-config**: `RpcClientConfig` options (baseUrl, headers, retry, timeout, hooks, etc.)
- **per-call-options**: `CallOptions` (signal, headers, timeout, dedup overrides)

Each page uses code blocks (with `+page.server.ts` for highlighting) showing config/code examples.

**Step 1-5: Create each page with server file and content**

**Step 6: Verify build**

Run: `cd demo && npm run build 2>&1 | head -30`

**Step 7: Commit**

```bash
git add demo/src/routes/docs/configuration/
git commit -m "feat(rpc): add configuration pages (config-file, cli, macros, client, per-call)"
```

---

### Task 6: Create Frameworks pages

**Files:**
- Create: `demo/src/routes/docs/frameworks/svelte/+page.svelte`
- Create: `demo/src/routes/docs/frameworks/react/+page.svelte`
- Create: `demo/src/routes/docs/frameworks/vue/+page.svelte`
- Create: `demo/src/routes/docs/frameworks/solid/+page.svelte`

**Context:** Each page covers the reactive wrapper for that framework:
- **Svelte 5**: `createQuery`, `createMutation`, runes integration
- **React**: `useQuery`, `useMutation` hooks
- **Vue 3**: `useQuery`, `useMutation` composables
- **SolidJS**: `createQuery`, `createMutation` primitives

Use code examples with `+page.server.ts` for highlighting. Show the API surface and a usage example for each.

**Step 1-4: Create each framework page**

**Step 5: Verify build**

Run: `cd demo && npm run build 2>&1 | head -30`

**Step 6: Commit**

```bash
git add demo/src/routes/docs/frameworks/
git commit -m "feat(rpc): add framework wrapper pages (svelte, react, vue, solid)"
```

---

### Task 7: Create Error Handling page

**Files:**
- Create: `demo/src/routes/docs/error-handling/+page.server.ts`
- Create: `demo/src/routes/docs/error-handling/+page.svelte`

**Context:** Move the existing error handling section (old `+page.svelte` lines 438-492) with the Secret demo. Also add explanation of `Result<T, E>` → `RpcError` flow.

**Step 1: Create server file with secretRust, secretTs code blocks**

**Step 2: Create page with Secret demo and RpcError explanation**

Needs imports: `createRpcClient` from `$lib/rpc-client`, `RpcError` from `$lib/rpc.svelte`.

**Step 3: Verify build**

Run: `cd demo && npm run build 2>&1 | head -30`

**Step 4: Commit**

```bash
git add demo/src/routes/docs/error-handling/
git commit -m "feat(rpc): add error handling page with secret demo"
```

---

### Task 8: Clean up old monolithic docs page

**Files:**
- Modify: `demo/src/routes/docs/+page.svelte` (should already be empty from Task 2)
- Modify: `demo/src/routes/docs/+page.server.ts` (should already be redirect from Task 2)

**Step 1: Verify the old content is fully migrated**

Check that every section from the original `+page.svelte` has been moved to its new page. The old file should only contain the redirect.

**Step 2: Run full build and dev server**

Run: `cd demo && npm run build 2>&1 | tail -10`
Expected: Clean build, no errors.

Run: `cd demo && npm run dev` and manually verify:
- `/docs` redirects to `/docs/getting-started`
- Sidebar shows all groups with expand/collapse
- Active page is highlighted
- Navigation between pages works
- Mobile hamburger menu works

**Step 3: Commit if any cleanup needed**

```bash
git add demo/src/routes/docs/
git commit -m "chore(rpc): clean up old monolithic docs page"
```
