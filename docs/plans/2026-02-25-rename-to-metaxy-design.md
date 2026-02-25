# Design: Rename Project to Metaxy

## Overview

Rename the entire project and all crates from `vercel-rpc` to `metaxy`.

## Renaming Table

### Crate Names

| Before | After |
|--------|-------|
| `vercel-rpc` | `metaxy` |
| `vercel-rpc-macro` | `metaxy-macro` |
| `vercel-rpc-cli` | `metaxy-cli` |
| `vercel-rpc-demo` | `metaxy-demo` |

### Rust Module Names

| Before | After |
|--------|-------|
| `vercel_rpc` | `metaxy` |
| `vercel_rpc_macro` | `metaxy_macro` |
| `vercel_rpc_cli` | `metaxy_cli` |

### Directories

| Before | After |
|--------|-------|
| `crates/rpc` | `crates/metaxy` |
| `crates/rpc-macro` | `crates/metaxy-macro` |
| `crates/rpc-cli` | `crates/metaxy-cli` |
| `svelte-rust` (root) | `metaxy` |

### Other

| Before | After |
|--------|-------|
| Binary: `rpc` | `metaxy` |
| Config: `rpc.config.toml` | `metaxy.config.toml` |
| Repo: `github.com/misha-mad/vercel-rpc` | `github.com/misha-mad/metaxy` |
| CLAUDE.md scopes: `rpc`, `rpc-macro`, `rpc-cli` | `metaxy`, `metaxy-macro`, `metaxy-cli` |

## What Does NOT Change

- Crate versions (keep current)
- Internal directory structure within crates (src/, tests/)
- Overall architecture and functionality

## Approach

**Big Bang** - single commit with all renaming changes.

Order of operations:
1. Rename directories (`crates/rpc*` -> `crates/metaxy*`)
2. Update all `Cargo.toml` files (workspace members, package names, dependencies)
3. Update Rust source code (`use` statements, `extern crate`, module references)
4. Update TypeScript/JavaScript files (auto-generated headers, package.json, code examples)
5. Rename `rpc.config.toml` -> `metaxy.config.toml` and update CLI code that reads it
6. Update documentation (README.md, CONTRIBUTING.md, crate READMEs)
7. Update CI/CD workflows
8. Update CLAUDE.md scopes
9. Update CHANGELOG files (URLs and crate names)
10. Regenerate Cargo.lock
11. Verify: `cargo build`, `cargo test`
12. Rename root directory: `svelte-rust` -> `metaxy`

## Affected Files (~80+ files)

- **Cargo.toml**: 5 files (workspace root + 4 crates)
- **Rust source**: 60+ files (lib.rs, main.rs, tests, codegen, parser modules)
- **Markdown docs**: 16+ files (READMEs, CHANGELOGs, RFCs, CONTRIBUTING)
- **TypeScript/JS**: 5+ files (generated types, client, package.json, route examples)
- **CI/CD**: 3 workflow files
- **Config**: CLAUDE.md, release-plz.toml, rpc.config.toml
