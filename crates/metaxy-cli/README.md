# metaxy-cli

[![Crates.io](https://img.shields.io/crates/v/metaxy-cli.svg)](https://crates.io/crates/metaxy-cli)
[![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/metaxy-cli.svg)](https://github.com/misha-mad/metaxy/blob/main/LICENSE-MIT)

CLI that scans Rust source files annotated with `#[rpc_query]` /
`#[rpc_mutation]` and generates TypeScript type definitions and a fully typed
RPC client.

Part of the [metaxy](https://github.com/misha-mad/metaxy) project.

## Installation

```bash
cargo install metaxy-cli
```

## Quick Start

```bash
metaxy generate --dir api --output src/lib/rpc-types.ts --client-output src/lib/rpc-client.ts
```

## Documentation

Full documentation: **[metaxy-demo.vercel.app/docs](https://metaxy-demo.vercel.app/docs/getting-started)**

- [CLI Commands](https://metaxy-demo.vercel.app/docs/cli/commands) — generate, scan, watch
- [Config File](https://metaxy-demo.vercel.app/docs/config-file) — metaxy.config.toml reference
- [Codegen](https://metaxy-demo.vercel.app/docs/codegen/type-mappings) — type mappings, serde, generics, field naming
- [Client](https://metaxy-demo.vercel.app/docs/client/config) — config, headers, timeout, hooks, retry, dedup
- [Frameworks](https://metaxy-demo.vercel.app/docs/frameworks/svelte) — Svelte 5, React, Vue 3, SolidJS

## License

MIT OR Apache-2.0
