# Docs Restructure Design

## Goal

Replace the current docs sidebar hierarchy with a flatter, topic-focused structure. The existing **Configuration** group gets split into **CLI**, **Macros**, and **Client** top-level sections.

## Current → New Mapping

| New Section | Source Pages |
|---|---|
| **Start** | Getting Started |
| **Procedures/** | Queries, Mutations, Streaming |
| **CLI/** | Config File, CLI |
| **Macros/** | Macro Attributes |
| **Client/** | RpcClientConfig, Retry, Deduplication |
| **Type System/** | Type Mappings, Serde, Generics, Branded Newtypes, Type Overrides, BigInt |
| **Codegen/** | Doc Comments, Field Naming |
| **Frameworks/** | Svelte 5, React, Vue 3, SolidJS |
| **Error** | Error Handling |

## File System Changes

Move/rename routes in `demo/src/routes/docs/`:

```
getting-started/  → start/
procedures/       → procedures/          (no change)
configuration/
  config-file/    → cli/config-file/
  cli/            → cli/commands/
  macro-attributes/ → macros/attributes/
  client-config/  → client/config/
  retry/          → client/retry/
  deduplication/  → client/deduplication/
type-system/      → type-system/         (no change)
codegen/          → codegen/             (no change)
frameworks/       → frameworks/          (no change)
error-handling/   → error/
```

## Sidebar Update

Update the sidebar navigation data to reflect the new groups and routes.

## Scope

- Route restructuring only (move files, update sidebar config, fix internal links)
- No content changes to the pages themselves beyond path updates
