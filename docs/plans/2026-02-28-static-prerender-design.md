# Static Prerender + Component Refactoring

## Context

The demo site uses `adapter-vercel` with SSR. The only server-side logic is Shiki syntax highlighting in `+page.server.ts` load functions. All interactive demos call Rust API endpoints (`/api/*`) directly from the browser. There are no SvelteKit API routes, form actions, or server state.

## Goals

1. Switch all pages to prerender — HTML generated at build time, served as static files from CDN
2. Extract reusable components from repeated UI patterns across doc pages
3. Simplify the duplicated `highlightBlocks` pattern in `+page.server.ts` files

## Approach: adapter-vercel + prerender

- Keep `adapter-vercel` — it supports prerendered pages alongside Rust Serverless Functions
- Add `export const prerender = true` in root `+layout.server.ts`
- Replace server-side redirect in `/docs/+page.server.ts` with client-side redirect
- All existing `+page.server.ts` files continue to work — SvelteKit executes them at build time

## Highlight Refactoring

Extract `highlightBlocks()` helper into `$lib/highlight.server.ts`:

```ts
export async function highlightBlocks(
  codeBlocks: Record<string, { code: string; lang: string }>
): Promise<{ highlighted: Record<string, string> }> { ... }
```

Each `+page.server.ts` simplifies to:
```ts
import { highlightBlocks } from '$lib/highlight.server';
const codeBlocks = { ... };
export const load = () => highlightBlocks(codeBlocks);
```

## New Components

### High Priority
| Component | Purpose | Used in |
|---|---|---|
| `Code` | Inline code badge (`bg-bg-code px-1.5 py-0.5 rounded text-xs font-mono`) | 60+ places |
| `Button` | Primary action button with disabled state | All interactive demos |
| `DemoCard` | Demo container (`border bg-bg-soft p-6`) with optional title/subtitle | 7+ cards |
| `CollapsibleCode` | Toggle "Show/Hide Rust" with internal state | cache (x2), type-mappings |
| `OutputBox` | Result display (success/error/neutral variants) | queries, hooks, cache, getting-started |

### Medium Priority
| Component | Purpose |
|---|---|
| `PageHeader` | h1 + lead paragraph for every doc page |
| `SectionHeading` | h2 with two levels (large for "Try it", normal for sections) |
| `FeatureRow` | Landing page code+text pair (10 repetitions) |

## Hosting

- Frontend: Vercel CDN (prerendered static HTML)
- Rust API: Vercel Serverless Functions (unchanged)
- Same Vercel project, same deploy pipeline

## Branch & PR

Work in a separate feature branch, create PR to main when complete.
