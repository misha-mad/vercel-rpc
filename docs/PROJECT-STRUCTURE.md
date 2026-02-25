# Project Structure

```
metaxy/
├── crates/
│   ├── metaxy/                   # Facade crate (re-exports macros + runtime deps)
│   │   └── src/lib.rs            #   pub use metaxy_macro::{rpc_query, rpc_mutation}
│   ├── metaxy-macro/             # Proc-macro crate
│   │   └── src/
│   │       ├── lib.rs            #   #[rpc_query] / #[rpc_mutation]
│   │       └── tests.rs          #   Macro expansion tests
│   └── metaxy-cli/               # CLI crate (library + binary: `metaxy`)
│       ├── src/
│       │   ├── lib.rs            #   Library root — public module declarations
│       │   ├── main.rs           #   CLI entry (clap arg parsing)
│       │   ├── commands.rs       #   scan / generate command implementations
│       │   ├── config.rs         #   metaxy.config.toml loading & merging
│       │   ├── model.rs          #   Manifest, Procedure, RustType, StructDef, EnumDef, FieldDef
│       │   ├── parser.rs         #   Module declarations for parser/
│       │   ├── parser/           #   Rust source → Manifest (via syn)
│       │   │   ├── extract.rs    #     File scanning & procedure extraction
│       │   │   ├── serde.rs      #     #[serde(...)] attribute parsing
│       │   │   └── types.rs      #     syn::Type → RustType conversion
│       │   ├── codegen.rs        #   Module declarations for codegen/
│       │   ├── codegen/          #   Manifest → TypeScript
│       │   │   ├── common.rs     #     Shared codegen logic (header, framework helpers)
│       │   │   ├── typescript.rs #     RustType → TS type mapping + rpc-types.ts
│       │   │   ├── client.rs     #     RpcClient interface + rpc-client.ts
│       │   │   ├── overrides.rs  #     Type override resolution
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
│           ├── overrides.rs      #   Type override tests
│           ├── svelte.rs         #   Svelte codegen (createQuery, createMutation)
│           ├── react.rs          #   React codegen (useQuery, useMutation)
│           ├── vue.rs            #   Vue codegen (useQuery, useMutation)
│           ├── solid.rs          #   SolidJS codegen (createQuery, createMutation)
│           └── snapshots/        #   insta snapshot files (auto-generated)
├── demo/                         # Demo application (SvelteKit) + Rust lambdas
│   ├── api/                      # Rust lambdas (each file = one endpoint)
│   │   ├── hello.rs              #   GET  /api/hello?input="name"
│   │   ├── time.rs               #   GET  /api/time
│   │   ├── status.rs             #   GET  /api/status
│   │   ├── math.rs               #   GET  /api/math?input={a,b,op}
│   │   ├── stats.rs              #   GET  /api/stats?input=[numbers]
│   │   ├── types.rs              #   GET  /api/types (expanded type mappings demo)
│   │   ├── secret.rs             #   GET  /api/secret (Headers demo)
│   │   ├── echo.rs               #   POST /api/echo (mutation)
│   │   └── profile.rs            #   GET  /api/profile?input=id (serde attrs demo)
│   ├── Cargo.toml                # Rust package for demo lambdas
│   ├── metaxy.config.toml        # CLI config file
│   ├── src/
│   │   ├── app.d.ts              # SvelteKit type declarations
│   │   ├── lib/
│   │   │   ├── rpc-types.ts      # ← auto-generated types
│   │   │   ├── rpc-client.ts     # ← auto-generated client
│   │   │   ├── rpc.svelte.ts     # ← auto-generated Svelte 5 wrappers
│   │   │   ├── client.ts         #   RPC client instance (manual)
│   │   │   ├── highlight.server.ts  # Shiki syntax highlighting (SSR)
│   │   │   └── components/
│   │   │       └── CodeBlock.svelte  # Code block component
│   │   └── routes/               # SvelteKit pages (docs site)
│   ├── tests/
│   │   ├── integration/
│   │   │   └── codegen.test.ts   # Vitest: codegen pipeline tests
│   │   └── e2e/
│   │       └── rpc.test.ts       # Playwright: UI + API tests
│   ├── package.json              # Node scripts
│   ├── svelte.config.js          # SvelteKit config
│   ├── vite.config.ts            # Vite config
│   ├── vitest.config.ts          # Vitest config
│   ├── playwright.config.ts      # Playwright config
│   ├── eslint.config.js          # ESLint config
│   ├── tsconfig.json             # TypeScript config
│   └── vercel.json               # Vercel deployment config
├── docs/                         # Design documents & references
│   ├── ROADMAP.md                #   Feature roadmap by phase
│   ├── PROJECT-STRUCTURE.md      #   This file
│   └── RFC/                      #   Individual RFC design documents
│       └── RFC-*.md
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                # CI: build, test, lint
│   │   ├── coverage.yml          # Code coverage (tarpaulin + codecov)
│   │   └── release-plz.yml      # Automated releases (release-plz)
│   ├── ISSUE_TEMPLATE/           # Bug report & feature request templates
│   └── DISCUSSION_TEMPLATE/      # Discussion category templates
├── Cargo.toml                    # Rust workspace (crates + demo)
├── Cargo.lock                    # Dependency lock file
├── README.md
├── CONTRIBUTING.md               # Development setup & contributing guide
├── CLAUDE.md                     # AI assistant instructions
├── release-plz.toml              # release-plz configuration
├── codecov.yml                   # Codecov configuration
└── rustfmt.toml                  # Rust formatter configuration
```
