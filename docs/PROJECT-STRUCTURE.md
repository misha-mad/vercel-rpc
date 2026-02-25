# Project Structure

```
metaxy/
├── crates/
│   ├── metaxy/                   # Facade crate (re-exports macros + runtime deps)
│   │   └── src/lib.rs            #   pub use metaxy_macro::{rpc_query, rpc_mutation}
│   ├── metaxy-macro/             # Proc-macro crate
│   │   └── src/lib.rs            #   #[rpc_query] / #[rpc_mutation]
│   └── metaxy-cli/               # CLI crate (library + binary: `metaxy`)
│       ├── src/
│       │   ├── lib.rs            #   Library root — public module declarations
│       │   ├── main.rs           #   CLI entry (clap arg parsing)
│       │   ├── commands.rs       #   scan / generate command implementations
│       │   ├── config.rs         #   metaxy.config.toml loading & merging
│       │   ├── model.rs          #   Manifest, Procedure, RustType, StructDef, EnumDef, FieldDef
│       │   ├── parser/           #   Rust source → Manifest (via syn)
│       │   │   ├── extract.rs    #     File scanning & procedure extraction
│       │   │   ├── serde.rs      #     #[serde(...)] attribute parsing
│       │   │   └── types.rs      #     syn::Type → RustType conversion
│       │   ├── codegen/          #   Manifest → TypeScript
│       │   │   ├── typescript.rs #     RustType → TS type mapping + rpc-types.ts
│       │   │   ├── client.rs     #     RpcClient interface + rpc-client.ts
│       │   │   ├── svelte.rs     #     Svelte 5 reactive wrappers (opt-in)
│       │   │   ├── react.rs      #     React hooks wrappers (opt-in)
│       │   │   ├── vue.rs        #     Vue 3 composable wrappers (opt-in)
│       │   │   └── solid.rs      #     SolidJS reactive primitives (opt-in)
│       │   └── watch.rs          #   File watcher with debounce
│       └── tests/                # Integration tests
│           ├── common/mod.rs     #   Shared test helpers
│           ├── commands.rs       #   scan / generate / write_file / bytecount
│           ├── config.rs         #   Config parsing, discovery, CLI overrides
│           ├── extract.rs        #   Parser extraction from Rust source
│           ├── types.rs          #   syn::Type → RustType + RenameRule conversion
│           ├── typescript.rs     #   TypeScript codegen (type mapping, JSDoc, serde)
│           ├── client.rs         #   Client codegen (RpcClient, overloads)
│           ├── svelte.rs         #   Svelte codegen (createQuery, createMutation)
│           ├── react.rs          #   React codegen (useQuery, useMutation)
│           ├── vue.rs            #   Vue codegen (useQuery, useMutation)
│           └── solid.rs          #   SolidJS codegen (createQuery, createMutation)
├── demo/                         # Demo application (SvelteKit) + Rust lambdas
│   ├── api/                      # Rust lambdas (each file = one endpoint)
│   │   ├── hello.rs              #   GET  /api/hello?input="name"
│   │   ├── time.rs               #   GET  /api/time
│   │   ├── status.rs             #   GET  /api/status
│   │   ├── math.rs               #   GET  /api/math?input={a,b,op}
│   │   ├── stats.rs              #   GET  /api/stats?input=[numbers]
│   │   ├── echo.rs               #   POST /api/echo (mutation)
│   │   └── profile.rs            #   GET  /api/profile?input=id (serde attrs demo)
│   ├── Cargo.toml                # Rust package for demo lambdas
│   ├── src/
│   │   ├── lib/
│   │   │   ├── rpc-types.ts      # ← auto-generated types
│   │   │   ├── rpc-client.ts     # ← auto-generated client
│   │   │   ├── rpc.svelte.ts     # ← auto-generated Svelte 5 wrappers
│   │   │   └── client.ts         #   RPC client instance (manual)
│   │   └── routes/               # SvelteKit pages
│   ├── tests/
│   │   ├── integration/          # Vitest: codegen pipeline tests
│   │   └── e2e/                  # Playwright: UI + API tests
│   ├── package.json              # Node scripts
│   ├── svelte.config.js          # SvelteKit config
│   ├── vite.config.ts            # Vite config
│   └── tsconfig.json             # TypeScript config
├── docs/                         # Design documents & references
│   ├── ROADMAP.md                #   Feature roadmap by phase
│   ├── RFC/                      #   Individual RFC design documents
│   │   └── RFC-*.md
│   └── PROJECT-STRUCTURE.md      #   This file
├── Cargo.toml                    # Rust workspace (crates + demo)
├── vercel.json                   # Vercel config
└── README.md
```
