# Demo Site Redesign — Design Document

## Goal

Rework the demo site to use Tailwind CSS v4 with a dark theme inspired by vite.dev. Reduce pages to Home and Docs (hybrid: text documentation + interactive RPC examples).

## Tech Stack

- **Tailwind CSS v4** via `@tailwindcss/vite` (CSS-first, no config file)
- **SvelteKit** (existing)
- Remove vanilla CSS variables and `layout.css`

## Color Palette

| Token | Value | Usage |
|-------|-------|-------|
| bg-primary | `#1b1b1f` | Main background |
| bg-sidebar | `#161618` | Sidebar background |
| bg-header | `#0f0f11` | Header background |
| bg-code | `#1e1e2e` | Code blocks |
| text-primary | `rgba(255,255,255,0.87)` | Main text |
| text-muted | `rgba(255,255,255,0.6)` | Secondary text |
| accent-rust | `#f74c00` | Rust/orange accent (CTA, highlights) |
| accent-ts | `#3178c6` | TypeScript/blue accent (secondary CTA, links) |
| border | `rgba(255,255,255,0.08)` | Borders, dividers |

## Pages

### Home (`/`)

- **Header** (shared): Logo `vercel-rpc`, nav (Home, Docs), GitHub icon
- **Hero**: Title, tagline, description
- **Features**: Grid of 5 feature cards with dark card backgrounds
- **CTA**: "Get Started" (orange) + "GitHub" (blue outline)
- **Footer** (shared): Minimal, GitHub link

### Docs (`/docs`)

- **Sidebar** (240px, left): Section navigation
- **Content** (right): Text + embedded interactive examples

**Sidebar sections:**
1. Getting Started
2. Queries (`#[rpc_query]`)
3. Mutations (`#[rpc_mutation]`)
4. Type Mappings
5. Error Handling
6. Streaming

Each section: description text + live RPC example (migrated from current `/examples` page).

## Structure Changes

- Delete: `about/+page.svelte`, `about/+page.ts`, `examples/+page.svelte`
- Create: `docs/+page.svelte` (or `docs/+layout.svelte` + sections)
- Rewrite: `Header.svelte`, `+layout.svelte`, `+page.svelte`
- Replace: `layout.css` with Tailwind `app.css`

## Design Principles

- Dark theme only (no toggle)
- Dual accent: orange (Rust) + blue (TypeScript)
- Layout inspired by vite.dev/guide
- Responsive: sidebar collapses on mobile
- Interactive examples preserved from current site
