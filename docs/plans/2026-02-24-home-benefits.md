# Home Page Benefits Section — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace the 5 benefit cards on the home page with 10 feature-overview cards.

**Architecture:** Single-file edit in `+page.svelte`. Replace the grid div (lines 19-40) with 10 cards in a 2-column grid. Same card component style, no new files.

**Tech Stack:** Svelte 5, Tailwind CSS

---

### Task 1: Replace benefit cards grid

**Files:**
- Modify: `demo/src/routes/+page.svelte:19-40`

**Step 1: Replace the grid**

Replace the entire `<div class="grid ...">` block (lines 19-40) with:

```svelte
<div class="grid grid-cols-1 sm:grid-cols-2 gap-3 mb-10 max-w-xl text-left text-sm">
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span>🛡️</span>
		<span><strong>End-to-end type safety</strong> — Rust types → TypeScript, zero manual sync</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span>⚡</span>
		<span><strong>Auto-generated client</strong> — typed <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.query()</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rpc.mutate()</code> with autocomplete</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span>👀</span>
		<span><strong>Watch mode</strong> — types regenerate on every <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> save</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span class="text-accent-rust">🦀</span>
		<span><strong>Macro-driven</strong> — <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_query]</code> / <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">#[rpc_mutation]</code> with CORS, parsing &amp; errors built in</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span class="text-accent-rust">🦀</span>
		<span><strong>Serde support</strong> — <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">rename_all</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">skip</code>, <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">flatten</code>, all 4 enum tagging strategies</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span class="text-accent-rust">🦀</span>
		<span><strong>Init &amp; state</strong> — cold-start setup for DB pools, HTTP clients via <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">init = "setup"</code></span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span class="text-accent-rust">🦀</span>
		<span><strong>Edge caching</strong> — <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">cache = "1h"</code> generates Cache-Control headers for Vercel CDN</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span class="text-accent-ts">🔌</span>
		<span><strong>4 framework wrappers</strong> — Svelte 5, React, Vue 3, SolidJS reactive hooks (opt-in)</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span class="text-accent-ts">🔧</span>
		<span><strong>Rich client</strong> — retry, timeout, dedup, lifecycle hooks, custom headers, AbortSignal</span>
	</div>
	<div class="flex items-start gap-2 rounded-lg bg-bg-soft border border-border p-3">
		<span>🚀</span>
		<span><strong>Vercel-native</strong> — each <code class="bg-bg-code px-1.5 py-0.5 rounded text-xs">.rs</code> file deploys as a serverless lambda</span>
	</div>
</div>
```

**Step 2: Verify build**

Run: `npx vite build`
Expected: build succeeds with no errors

**Step 3: Commit**

```bash
git add demo/src/routes/+page.svelte
git commit -m "feat(rpc): expand home page benefits to 10 feature cards"
```
