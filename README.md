<div align="center">

# metaxy

/mɛˈtæk.si/ · Greek: μεταξύ · *the in-between*

**End-to-end typesafe RPC between Rust lambdas on Vercel and any TypeScript frontend**

[Documentation](https://metaxy-demo.vercel.app/docs/getting-started) · [Live Demo](https://metaxy-demo.vercel.app)

[![CI](https://github.com/misha-mad/metaxy/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/misha-mad/metaxy/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/misha-mad/metaxy/graph/badge.svg)](https://codecov.io/gh/misha-mad/metaxy)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-yellow.svg)](#license)

</div>

---

> **Note:** This project was previously published as `vercel-rpc` / `vercel-rpc-macro` / `vercel-rpc-cli` on crates.io. Those crates are deprecated — use `metaxy`, `metaxy-macro`, and `metaxy-cli` instead.

---

## Why?

Building serverless APIs with Rust on Vercel is fast — but keeping TypeScript types in sync is painful. **metaxy** solves this:

- Write plain Rust functions with `#[rpc_query]` / `#[rpc_mutation]`
- Auto-generate TypeScript types & a fully typed client
- Framework integrations for Svelte 5, React, Vue 3, and SolidJS
- Watch mode — types regenerate on every save
- Each function deploys as a serverless lambda on Vercel

## Quick Start

```bash
cargo install metaxy-cli
cargo add metaxy-macro
```

```rust
// api/hello.rs
use metaxy::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}
```

```bash
metaxy generate --dir api --output src/lib/rpc-types.ts --client-output src/lib/rpc-client.ts
```

```typescript
import { createRpcClient } from "./rpc-client";

const rpc = createRpcClient({ baseUrl: "/api" });
const greeting = await rpc.query("hello", "World");
//                          ^ autocomplete    ^ typed as string
```

See the [Getting Started guide](https://metaxy-demo.vercel.app/docs/getting-started) for the full walkthrough.

## Documentation

Full documentation is available at **[metaxy-demo.vercel.app/docs](https://metaxy-demo.vercel.app/docs/getting-started)**:

- [Procedures](https://metaxy-demo.vercel.app/docs/procedures/queries) — queries, mutations, streaming
- [Macro Attributes](https://metaxy-demo.vercel.app/docs/macros/attributes) — cache, stale, init, timeout, idempotent
- [Codegen](https://metaxy-demo.vercel.app/docs/codegen/type-mappings) — type mappings, serde, generics, field naming, branded newtypes, bigint, type overrides
- [Client](https://metaxy-demo.vercel.app/docs/client/config) — config, headers, timeout, hooks, retry, dedup, custom fetch, serialization
- [Frameworks](https://metaxy-demo.vercel.app/docs/frameworks/svelte) — Svelte 5, React, Vue 3, SolidJS
- [CLI](https://metaxy-demo.vercel.app/docs/cli/commands) — generate, scan, watch
- [Config File](https://metaxy-demo.vercel.app/docs/config-file) — metaxy.config.toml reference
- [Error Handling](https://metaxy-demo.vercel.app/docs/error-handling)

## Project Structure

See [docs/PROJECT-STRUCTURE.md](./docs/PROJECT-STRUCTURE.md) for the full annotated file tree.

## Deploy to Vercel

Each `.rs` file in `api/` becomes a serverless function at `/api/<name>`.

```bash
npm i -g vercel
vercel
```

## Sponsors

<div align="center">
  <em>You could be the first sponsor!</em>
</div>

<p align="center">If you find this project useful, consider <a href="https://github.com/sponsors/misha-mad">sponsoring</a> to support its development.</p>

## License

MIT OR Apache-2.0

---

<sub>This project is not affiliated with or endorsed by Vercel Inc. "Vercel" is a trademark of Vercel Inc.</sub>
