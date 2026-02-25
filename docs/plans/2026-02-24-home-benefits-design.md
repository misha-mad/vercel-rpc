# Home Page Benefits Section — Design

## Goal

Replace the current 5 benefit cards on the home page with a comprehensive 10-item feature overview targeting both Rust and TypeScript developers. Only ship features that work today (no streaming/coming soon).

## Benefits List

### Core (what & why)

1. **End-to-end type safety** — Rust types -> TypeScript types, zero manual sync
2. **Auto-generated client** — typed `rpc.query()` / `rpc.mutate()` with autocomplete
3. **Watch mode** — types regenerate on every `.rs` save

### Server-side (Rust)

4. **Macro-driven** — `#[rpc_query]` / `#[rpc_mutation]` — CORS, parsing, error handling out of the box
5. **Serde support** — `rename_all`, `rename`, `skip`, `flatten`, all 4 enum tagging strategies
6. **Init & state injection** — cold-start setup for DB pools, HTTP clients via `init = "setup"`
7. **Edge caching** — `cache = "1h"` generates `Cache-Control` headers for Vercel CDN

### Client-side (TypeScript)

8. **4 framework wrappers** — Svelte 5, React, Vue 3, SolidJS (opt-in reactive hooks)
9. **Rich client** — retry, timeout, dedup, lifecycle hooks, custom headers, AbortSignal

### Deploy

10. **Vercel-native** — each `.rs` file deploys as a serverless lambda, zero config

## Layout

Current: 2-column grid (2x2 + 1 half-width). New: 2-column grid accommodating 10 items (5 rows), same card style as current.

## File

`demo/src/routes/+page.svelte` — replace the existing grid of 5 cards with 10 cards.
