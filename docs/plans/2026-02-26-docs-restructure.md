# Docs Restructure Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Restructure docs sidebar from 7 sections to 9 by splitting Configuration into CLI, Macros, and Client top-level groups.

**Architecture:** Pure file moves + sidebar config update. No content changes. SvelteKit file-based routing — moving directories changes URLs automatically.

**Tech Stack:** SvelteKit 5 file-based routing, Svelte components

---

### Task 1: Move `getting-started/` → `start/`

**Files:**
- Move: `demo/src/routes/docs/getting-started/` → `demo/src/routes/docs/start/`

**Step 1: Move the directory**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
mv demo/src/routes/docs/getting-started demo/src/routes/docs/start
```

**Step 2: Commit**

```bash
git add -A demo/src/routes/docs/getting-started demo/src/routes/docs/start
git commit -m "docs: rename getting-started → start"
```

---

### Task 2: Create CLI section from Configuration pages

**Files:**
- Move: `demo/src/routes/docs/configuration/config-file/` → `demo/src/routes/docs/cli/config-file/`
- Move: `demo/src/routes/docs/configuration/cli/` → `demo/src/routes/docs/cli/commands/`

**Step 1: Create target directory and move files**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
mkdir -p demo/src/routes/docs/cli
mv demo/src/routes/docs/configuration/config-file demo/src/routes/docs/cli/config-file
mv demo/src/routes/docs/configuration/cli demo/src/routes/docs/cli/commands
```

**Step 2: Commit**

```bash
git add -A demo/src/routes/docs/configuration/config-file demo/src/routes/docs/configuration/cli demo/src/routes/docs/cli
git commit -m "docs: move config-file and cli into cli/ section"
```

---

### Task 3: Create Macros section from Configuration

**Files:**
- Move: `demo/src/routes/docs/configuration/macro-attributes/` → `demo/src/routes/docs/macros/attributes/`

**Step 1: Move directory**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
mkdir -p demo/src/routes/docs/macros
mv demo/src/routes/docs/configuration/macro-attributes demo/src/routes/docs/macros/attributes
```

**Step 2: Commit**

```bash
git add -A demo/src/routes/docs/configuration/macro-attributes demo/src/routes/docs/macros
git commit -m "docs: move macro-attributes into macros/ section"
```

---

### Task 4: Create Client section from Configuration

**Files:**
- Move: `demo/src/routes/docs/configuration/client-config/` → `demo/src/routes/docs/client/config/`
- Move: `demo/src/routes/docs/configuration/retry/` → `demo/src/routes/docs/client/retry/`
- Move: `demo/src/routes/docs/configuration/deduplication/` → `demo/src/routes/docs/client/deduplication/`

**Step 1: Move directories**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
mkdir -p demo/src/routes/docs/client
mv demo/src/routes/docs/configuration/client-config demo/src/routes/docs/client/config
mv demo/src/routes/docs/configuration/retry demo/src/routes/docs/client/retry
mv demo/src/routes/docs/configuration/deduplication demo/src/routes/docs/client/deduplication
```

**Step 2: Remove the now-empty configuration directory**

```bash
rmdir demo/src/routes/docs/configuration
```

**Step 3: Commit**

```bash
git add -A demo/src/routes/docs/configuration demo/src/routes/docs/client
git commit -m "docs: move client-config, retry, deduplication into client/ section"
```

---

### Task 5: Move `error-handling/` → `error/`

**Files:**
- Move: `demo/src/routes/docs/error-handling/` → `demo/src/routes/docs/error/`

**Step 1: Move the directory**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy
mv demo/src/routes/docs/error-handling demo/src/routes/docs/error
```

**Step 2: Commit**

```bash
git add -A demo/src/routes/docs/error-handling demo/src/routes/docs/error
git commit -m "docs: rename error-handling → error"
```

---

### Task 6: Update sidebar nav config

**Files:**
- Modify: `demo/src/routes/docs/+layout.svelte:8-57`

**Step 1: Replace the `nav` array with the new structure**

```typescript
const nav = [
	{ label: 'Start', href: '/docs/start' },
	{
		label: 'Procedures',
		children: [
			{ label: 'Queries', href: '/docs/procedures/queries' },
			{ label: 'Mutations', href: '/docs/procedures/mutations' },
			{ label: 'Streaming', href: '/docs/procedures/streaming', badge: 'soon' }
		]
	},
	{
		label: 'CLI',
		children: [
			{ label: 'Config File', href: '/docs/cli/config-file' },
			{ label: 'Commands', href: '/docs/cli/commands' }
		]
	},
	{
		label: 'Macros',
		children: [
			{ label: 'Attributes', href: '/docs/macros/attributes' }
		]
	},
	{
		label: 'Client',
		children: [
			{ label: 'RpcClientConfig', href: '/docs/client/config' },
			{ label: 'Retry', href: '/docs/client/retry' },
			{ label: 'Deduplication', href: '/docs/client/deduplication' }
		]
	},
	{
		label: 'Type System',
		children: [
			{ label: 'Type Mappings', href: '/docs/type-system/type-mappings' },
			{ label: 'Serde Support', href: '/docs/type-system/serde' },
			{ label: 'Generics', href: '/docs/type-system/generics' },
			{ label: 'Branded Newtypes', href: '/docs/type-system/branded-newtypes' },
			{ label: 'Type Overrides', href: '/docs/type-system/type-overrides' },
			{ label: 'BigInt Mapping', href: '/docs/type-system/bigint' }
		]
	},
	{
		label: 'Codegen',
		children: [
			{ label: 'Doc Comments', href: '/docs/codegen/doc-comments' },
			{ label: 'Field Naming', href: '/docs/codegen/field-naming' }
		]
	},
	{
		label: 'Frameworks',
		children: [
			{ label: 'Svelte 5', href: '/docs/frameworks/svelte' },
			{ label: 'React', href: '/docs/frameworks/react' },
			{ label: 'Vue 3', href: '/docs/frameworks/vue' },
			{ label: 'SolidJS', href: '/docs/frameworks/solid' }
		]
	},
	{ label: 'Error', href: '/docs/error' }
] as const;
```

**Step 2: Commit**

```bash
git add demo/src/routes/docs/+layout.svelte
git commit -m "docs: update sidebar nav for new section structure"
```

---

### Task 7: Update `/docs/` redirect

**Files:**
- Modify: `demo/src/routes/docs/+page.server.ts`

**Step 1: Change redirect target from `/docs/getting-started` to `/docs/start`**

```typescript
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = () => {
	redirect(301, '/docs/start');
};
```

**Step 2: Commit**

```bash
git add demo/src/routes/docs/+page.server.ts
git commit -m "docs: update /docs/ redirect to /docs/start"
```

---

### Task 8: Verify the site builds and all routes work

**Step 1: Run dev server and check routes**

```bash
cd /Users/mikhailzakharov/RustroverProjects/metaxy/demo
npm run build
```

Expected: Build succeeds with no errors.

**Step 2: Spot-check key routes exist in build output**

Verify these routes appear in the build:
- `/docs/start`
- `/docs/cli/config-file`
- `/docs/cli/commands`
- `/docs/macros/attributes`
- `/docs/client/config`
- `/docs/client/retry`
- `/docs/client/deduplication`
- `/docs/error`
