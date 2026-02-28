# Static Prerender + Component Refactoring — Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Convert the demo site to fully prerendered static pages and extract reusable UI components.

**Architecture:** Enable `prerender = true` globally so SvelteKit runs all `+page.server.ts` at build time and emits static HTML. Extract 8 reusable Svelte components from repeated patterns. Simplify the duplicated highlight boilerplate in 35 `+page.server.ts` files into a shared helper.

**Tech Stack:** SvelteKit 2, Svelte 5, adapter-vercel, Shiki, TailwindCSS

---

### Task 1: Enable prerender globally

**Files:**
- Create: `demo/src/routes/+layout.server.ts`
- Modify: `demo/src/routes/docs/+page.server.ts`

**Step 1: Create root layout server file with prerender**

Create `demo/src/routes/+layout.server.ts`:

```ts
export const prerender = true;
```

**Step 2: Replace server redirect with client-side redirect**

Replace `demo/src/routes/docs/+page.server.ts` with a `+page.ts` that does client-side redirect:

Delete `demo/src/routes/docs/+page.server.ts`.

Create `demo/src/routes/docs/+page.ts`:

```ts
import { redirect } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = () => {
	redirect(301, '/docs/getting-started');
};
```

**Step 3: Build to verify prerender works**

Run: `cd demo && npm run build`
Expected: Build succeeds, all pages are prerendered to `.html` files in `.vercel/output/static/`

**Step 4: Commit**

```
feat(metaxy): enable prerender for all demo pages
```

---

### Task 2: Extract `highlightBlocks` helper

**Files:**
- Modify: `demo/src/lib/highlight.server.ts`
- Modify: All 35 `+page.server.ts` files with highlight pattern

**Step 1: Add `highlightBlocks` function to highlight.server.ts**

Add to end of `demo/src/lib/highlight.server.ts`:

```ts
export async function highlightBlocks(
	codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' | 'shellscript' | 'toml' }>
): Promise<{ highlighted: Record<string, string> }> {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(entries.map(([, { code, lang }]) => highlightCode(code, lang)));
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
}
```

**Step 2: Update all `+page.server.ts` files**

For each of the 35 `+page.server.ts` files (excluding `/docs/+page.server.ts` which was already replaced), change the import and load function.

Before (every file):
```ts
import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: ... }> = { ... };

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(entries.map(([, { code, lang }]) => highlightCode(code, lang)));
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
```

After:
```ts
import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks = { ... };  // same codeBlocks, unchanged

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
```

The complete list of files to update (35 files):
- `demo/src/routes/+page.server.ts`
- `demo/src/routes/docs/getting-started/+page.server.ts`
- `demo/src/routes/docs/procedures/queries/+page.server.ts`
- `demo/src/routes/docs/procedures/mutations/+page.server.ts`
- `demo/src/routes/docs/macros/cache/+page.server.ts`
- `demo/src/routes/docs/macros/stale/+page.server.ts`
- `demo/src/routes/docs/macros/init/+page.server.ts`
- `demo/src/routes/docs/macros/timeout/+page.server.ts`
- `demo/src/routes/docs/macros/idempotent/+page.server.ts`
- `demo/src/routes/docs/codegen/type-mappings/+page.server.ts`
- `demo/src/routes/docs/codegen/serde/+page.server.ts`
- `demo/src/routes/docs/codegen/generics/+page.server.ts`
- `demo/src/routes/docs/codegen/doc-comments/+page.server.ts`
- `demo/src/routes/docs/codegen/field-naming/+page.server.ts`
- `demo/src/routes/docs/codegen/branded-newtypes/+page.server.ts`
- `demo/src/routes/docs/codegen/bigint/+page.server.ts`
- `demo/src/routes/docs/codegen/type-overrides/+page.server.ts`
- `demo/src/routes/docs/client/config/+page.server.ts`
- `demo/src/routes/docs/client/headers/+page.server.ts`
- `demo/src/routes/docs/client/timeout/+page.server.ts`
- `demo/src/routes/docs/client/hooks/+page.server.ts`
- `demo/src/routes/docs/client/retry/+page.server.ts`
- `demo/src/routes/docs/client/deduplication/+page.server.ts`
- `demo/src/routes/docs/client/fetch/+page.server.ts`
- `demo/src/routes/docs/client/serialization/+page.server.ts`
- `demo/src/routes/docs/frameworks/svelte/+page.server.ts`
- `demo/src/routes/docs/frameworks/react/+page.server.ts`
- `demo/src/routes/docs/frameworks/vue/+page.server.ts`
- `demo/src/routes/docs/frameworks/solid/+page.server.ts`
- `demo/src/routes/docs/error-handling/+page.server.ts`
- `demo/src/routes/docs/cli/commands/+page.server.ts`
- `demo/src/routes/docs/cli/generate/+page.server.ts`
- `demo/src/routes/docs/cli/scan/+page.server.ts`
- `demo/src/routes/docs/cli/watch/+page.server.ts`
- `demo/src/routes/docs/config-file/+page.server.ts`

**Step 3: Build to verify**

Run: `cd demo && npm run build`
Expected: Build succeeds, all pages prerender correctly

**Step 4: Commit**

```
refactor(metaxy): extract highlightBlocks helper, simplify 35 page loaders
```

---

### Task 3: Create `Code` component (inline code badge)

**Files:**
- Create: `demo/src/lib/components/Code.svelte`

**Step 1: Create the component**

Create `demo/src/lib/components/Code.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let { children }: { children: Snippet } = $props();
</script>

<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">{@render children()}</code>
```

**Step 2: Commit**

```
feat(metaxy): add Code component for inline code badges
```

---

### Task 4: Create `Button` component

**Files:**
- Create: `demo/src/lib/components/Button.svelte`

**Step 1: Create the component**

Create `demo/src/lib/components/Button.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		onclick,
		disabled = false,
		variant = 'ts',
		children
	}: {
		onclick: () => void;
		disabled?: boolean;
		variant?: 'ts' | 'rust';
		children: Snippet;
	} = $props();
</script>

<button
	{onclick}
	{disabled}
	class="rounded-md px-3 py-1.5 text-sm font-medium text-white transition-opacity hover:opacity-85 disabled:opacity-50"
	class:bg-accent-ts={variant === 'ts'}
	class:bg-accent-rust={variant === 'rust'}
>
	{@render children()}
</button>
```

**Step 2: Commit**

```
feat(metaxy): add Button component
```

---

### Task 5: Create `DemoCard` component

**Files:**
- Create: `demo/src/lib/components/DemoCard.svelte`

**Step 1: Create the component**

Create `demo/src/lib/components/DemoCard.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		subtitle,
		children
	}: {
		title?: string;
		subtitle?: string;
		children: Snippet;
	} = $props();
</script>

<div class="rounded-lg border border-border bg-bg-soft p-6">
	{#if title}
		<h3 class="text-lg font-semibold mb-2">{title}</h3>
	{/if}
	{#if subtitle}
		<p class="text-text-muted text-sm mb-4">{subtitle}</p>
	{/if}
	{@render children()}
</div>
```

**Step 2: Commit**

```
feat(metaxy): add DemoCard component
```

---

### Task 6: Create `CollapsibleCode` component

**Files:**
- Create: `demo/src/lib/components/CollapsibleCode.svelte`

**Step 1: Create the component**

Create `demo/src/lib/components/CollapsibleCode.svelte`:

```svelte
<script lang="ts">
	import CodeBlock from './CodeBlock.svelte';

	let { html, label = 'Rust' }: { html: string; label?: string } = $props();

	let open = $state(false);
</script>

<button
	class="mt-3 text-xs text-text-faint hover:text-text-muted transition-colors"
	onclick={() => (open = !open)}
>
	{open ? '▾ Hide' : '▸ Show'}
	{label}
</button>
{#if open}
	<div class="mt-3">
		<CodeBlock {html} />
	</div>
{/if}
```

**Step 2: Commit**

```
feat(metaxy): add CollapsibleCode component
```

---

### Task 7: Create `OutputBox` component

**Files:**
- Create: `demo/src/lib/components/OutputBox.svelte`

**Step 1: Create the component**

Create `demo/src/lib/components/OutputBox.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		status = 'neutral',
		mono = false,
		children
	}: {
		status?: 'neutral' | 'success' | 'error';
		mono?: boolean;
		children: Snippet;
	} = $props();
</script>

<div
	class="rounded-md bg-bg-code p-3 text-sm space-y-1 overflow-x-auto"
	class:font-mono={mono}
	class:text-text-muted={status === 'neutral'}
	class:text-green-400={status === 'success'}
	class:text-red-400={status === 'error'}
>
	{@render children()}
</div>
```

**Step 2: Commit**

```
feat(metaxy): add OutputBox component
```

---

### Task 8: Create `PageHeader` and `SectionHeading` components

**Files:**
- Create: `demo/src/lib/components/PageHeader.svelte`
- Create: `demo/src/lib/components/SectionHeading.svelte`

**Step 1: Create PageHeader**

Create `demo/src/lib/components/PageHeader.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		mono = false,
		children
	}: {
		title: string;
		mono?: boolean;
		children?: Snippet;
	} = $props();
</script>

<h1 class="text-3xl font-bold" class:font-mono={mono}>{title}</h1>
{#if children}
	<p class="text-text-muted leading-relaxed">{@render children()}</p>
{/if}
```

**Step 2: Create SectionHeading**

Create `demo/src/lib/components/SectionHeading.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		level = 'normal',
		children
	}: {
		level?: 'normal' | 'large';
		children: Snippet;
	} = $props();
</script>

{#if level === 'large'}
	<h2 class="text-2xl font-bold mt-12">{@render children()}</h2>
{:else}
	<h2 class="text-xl font-semibold mt-8">{@render children()}</h2>
{/if}
```

**Step 3: Commit**

```
feat(metaxy): add PageHeader and SectionHeading components
```

---

### Task 9: Create `FeatureRow` component for landing page

**Files:**
- Create: `demo/src/lib/components/FeatureRow.svelte`

**Step 1: Create the component**

The landing page alternates between "code-left/text-right" and "text-left/code-right" rows. Each row is a 3-column grid: 2 cols for code, 1 for text. Some rows have 2 code blocks side-by-side (type safety).

Create `demo/src/lib/components/FeatureRow.svelte`:

```svelte
<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		title,
		reverse = false,
		code,
		children
	}: {
		title: string;
		reverse?: boolean;
		code: Snippet;
		children: Snippet;
	} = $props();
</script>

{#if reverse}
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">{title}</h3>
		<p class="text-sm text-text-muted leading-relaxed">{@render children()}</p>
	</div>
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		{@render code()}
	</div>
{:else}
	<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
		{@render code()}
	</div>
	<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
		<h3 class="text-lg font-semibold">{title}</h3>
		<p class="text-sm text-text-muted leading-relaxed">{@render children()}</p>
	</div>
{/if}
```

**Step 2: Commit**

```
feat(metaxy): add FeatureRow component for landing page
```

---

### Task 10: Apply components to `getting-started` page

**Files:**
- Modify: `demo/src/routes/docs/getting-started/+page.svelte`

**Step 1: Refactor the page**

Replace repeated patterns with new components: `Code`, `Button`, `DemoCard`, `OutputBox`, `PageHeader`, `SectionHeading`.

Key replacements:
- `<h1 class="text-3xl font-bold">Getting Started</h1>` → `<PageHeader title="Getting Started">`
- `<code class="bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono">...</code>` → `<Code>...</Code>`
- `<h2 class="text-2xl font-semibold">...</h2>` → `<SectionHeading level="large">...</SectionHeading>`
- `<h3 class="text-xl font-semibold mt-8">...</h3>` → `<SectionHeading>...</SectionHeading>`
- `<div class="rounded-lg border border-border bg-bg-soft p-6">` → `<DemoCard>`
- The button with long class → `<Button>`
- `<div class="rounded-md bg-bg-code p-3 text-sm text-green-400">` → `<OutputBox status="success">`
- `<div class="rounded-md bg-bg-code p-3 text-sm text-red-400">` → `<OutputBox status="error">`
- `<div class="rounded-md bg-bg-code p-3 text-sm text-text-muted">` → `<OutputBox>`

**Step 2: Build to verify**

Run: `cd demo && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```
refactor(metaxy): apply shared components to getting-started page
```

---

### Task 11: Apply components to `queries` page

**Files:**
- Modify: `demo/src/routes/docs/procedures/queries/+page.svelte`

**Step 1: Refactor**

Same pattern as Task 10: replace inline code badges, headings, demo card, output boxes, button.

**Step 2: Build to verify**

**Step 3: Commit**

```
refactor(metaxy): apply shared components to queries page
```

---

### Task 12: Apply components to `hooks` page

**Files:**
- Modify: `demo/src/routes/docs/client/hooks/+page.svelte`

**Step 1: Refactor**

Replace Code, Button, DemoCard, OutputBox, PageHeader, SectionHeading.

**Step 2: Build to verify**

**Step 3: Commit**

```
refactor(metaxy): apply shared components to hooks page
```

---

### Task 13: Apply components to `cache` page

**Files:**
- Modify: `demo/src/routes/docs/macros/cache/+page.svelte`

**Step 1: Refactor**

Replace Code, Button, DemoCard, CollapsibleCode, OutputBox, PageHeader, SectionHeading. This page has 2 demo cards with CollapsibleCode — replace the manual toggle state with the component.

Remove the `openCode` state and `toggleCode` function from `<script>` — they are encapsulated in CollapsibleCode now.

**Step 2: Build to verify**

**Step 3: Commit**

```
refactor(metaxy): apply shared components to cache page
```

---

### Task 14: Apply components to `type-mappings` page

**Files:**
- Modify: `demo/src/routes/docs/codegen/type-mappings/+page.svelte`

**Step 1: Refactor**

This page has the most inline `<code>` badges (~30). Replace with Code component. Also replace Button, DemoCard, CollapsibleCode, OutputBox, PageHeader. Remove the `openCode` state.

**Step 2: Build to verify**

**Step 3: Commit**

```
refactor(metaxy): apply shared components to type-mappings page
```

---

### Task 15: Apply components to remaining doc pages

**Files:**
- Modify: All remaining `+page.svelte` files under `demo/src/routes/docs/`

**Step 1: Batch refactor remaining pages**

Apply Code, PageHeader, SectionHeading to all remaining doc pages. Not all pages have interactive demos, so Button/DemoCard/OutputBox only apply where relevant.

Pages with interactive demos (need full component set):
- `mutations/+page.svelte`
- `stale/+page.svelte`
- `init/+page.svelte`
- `timeout/+page.svelte`
- `idempotent/+page.svelte`
- `bigint/+page.svelte`
- `deduplication/+page.svelte`
- `headers/+page.svelte`
- `retry/+page.svelte`
- `serialization/+page.svelte`

Pages that are primarily documentation (Code, PageHeader, SectionHeading only):
- All remaining pages under codegen/, client/, cli/, frameworks/, error-handling, config-file

**Step 2: Build to verify**

Run: `cd demo && npm run build`
Expected: Build succeeds

**Step 3: Commit**

```
refactor(metaxy): apply shared components to all remaining doc pages
```

---

### Task 16: Apply FeatureRow to landing page

**Files:**
- Modify: `demo/src/routes/+page.svelte`

**Step 1: Refactor landing page**

Replace the 10 alternating code/text blocks with FeatureRow components. Also replace inline `<code>` badges with Code.

Before (repeated 10x):
```svelte
<div class="sm:col-span-2 bg-bg-primary p-5 sm:p-10 flex flex-col gap-3">
	<CodeBlock html={data.highlighted['...']} />
</div>
<div class="bg-bg-primary p-6 sm:p-10 flex flex-col justify-center gap-3">
	<h3 class="text-lg font-semibold">...</h3>
	<p class="text-sm text-text-muted leading-relaxed">...</p>
</div>
```

After:
```svelte
<FeatureRow title="..." code={codeSnippet}>
	{#snippet codeSnippet()}<CodeBlock html={data.highlighted['...']} />{/snippet}
	Description text...
</FeatureRow>
```

Special case: Row 1 (type safety) has 2 side-by-side CodeBlocks in a grid — the `code` snippet handles this.

**Step 2: Build to verify**

**Step 3: Commit**

```
refactor(metaxy): apply FeatureRow to landing page
```

---

### Task 17: Final verification

**Step 1: Full build**

Run: `cd demo && npm run build`
Expected: Build succeeds, all pages prerendered

**Step 2: Preview and spot-check**

Run: `cd demo && npm run preview`
Check: Landing page, getting-started, queries, cache, type-mappings pages render correctly

**Step 3: Run linter**

Run: `cd demo && npm run lint`
Expected: No lint errors

**Step 4: Run integration tests**

Run: `cd demo && npm run test:integration`
Expected: All tests pass

**Step 5: Push and create PR**

```
git push -u origin feat/static-prerender
gh pr create --title "feat: prerender all demo pages + extract shared components" --body "..."
```
