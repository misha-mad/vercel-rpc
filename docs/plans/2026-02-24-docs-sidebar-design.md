# Docs Sidebar Redesign

## Goal

Replace the current flat anchor-based sidebar with a multi-page, hierarchical navigation system. Each sidebar item opens its own documentation page. Groups have expandable children. Active page is highlighted.

## Navigation Structure

```
Getting Started                         /docs/getting-started
Procedures ▾
  ├─ Queries                            /docs/procedures/queries
  ├─ Mutations                          /docs/procedures/mutations
  └─ Streaming (coming soon)            /docs/procedures/streaming
Configuration ▾
  ├─ rpc.config.toml                    /docs/configuration/config-file
  ├─ CLI                                /docs/configuration/cli
  ├─ Macro Attributes                   /docs/configuration/macro-attributes
  ├─ RpcClientConfig                    /docs/configuration/client-config
  └─ Per-Call Options                   /docs/configuration/per-call-options
Type System ▾
  ├─ Type Mappings                      /docs/type-system/type-mappings
  ├─ Serde Support                      /docs/type-system/serde
  ├─ Generics                           /docs/type-system/generics
  └─ Branded Newtypes                   /docs/type-system/branded-newtypes
Frameworks ▾
  ├─ Svelte 5                           /docs/frameworks/svelte
  ├─ React                              /docs/frameworks/react
  ├─ Vue 3                              /docs/frameworks/vue
  └─ SolidJS                            /docs/frameworks/solid
Error Handling                          /docs/error-handling
```

## Routing Approach

File-based SvelteKit routing. Each section is a folder with `+page.svelte` (and `+page.server.ts` when code highlighting is needed).

```
demo/src/routes/docs/
├─ +layout.svelte              (sidebar + content slot)
├─ +page.svelte                (redirect → /docs/getting-started)
├─ getting-started/+page.svelte
├─ procedures/
│   ├─ queries/+page.svelte
│   ├─ mutations/+page.svelte
│   └─ streaming/+page.svelte
├─ configuration/
│   ├─ config-file/+page.svelte
│   ├─ cli/+page.svelte
│   ├─ macro-attributes/+page.svelte
│   ├─ client-config/+page.svelte
│   └─ per-call-options/+page.svelte
├─ type-system/
│   ├─ type-mappings/+page.svelte
│   ├─ serde/+page.svelte
│   ├─ generics/+page.svelte
│   └─ branded-newtypes/+page.svelte
├─ frameworks/
│   ├─ svelte/+page.svelte
│   ├─ react/+page.svelte
│   ├─ vue/+page.svelte
│   └─ solid/+page.svelte
└─ error-handling/+page.svelte
```

## Sidebar Component Design

### Data structure

```ts
type NavItem = { label: string; href: string; badge?: string }
type NavGroup = { label: string; children: NavItem[] }
type NavEntry = NavItem | NavGroup

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
```

### Active state

Compare `page.url.pathname` with `href`. Active link gets:
- `bg-bg-soft text-text-primary` background
- `border-l-2 border-accent-rust` left accent

### Group expand/collapse

- A group is **auto-expanded** if any child `href` matches current pathname
- Groups can also be **manually toggled** by clicking the group header
- Expand/collapse animated with `svelte/transition` slide
- Chevron rotates: `▸` collapsed → `▾` expanded

### Badge

Items with `badge: 'soon'` get a small `rounded-full bg-accent-rust/20 text-accent-rust text-[10px] px-1.5` tag.

### Mobile

Keep existing pattern: fixed hamburger button (bottom-right), sidebar slides in from left with backdrop overlay. Close on link click.

## Content Migration

Current single-page content from `docs/+page.svelte` will be split across the new page files. Each page gets its own `+page.server.ts` for code highlighting where needed. The existing interactive demos (hello, time, math, etc.) will be placed within the relevant feature pages as examples.
