# Contributing to vercel-rpc

Thank you for your interest in contributing! This guide will help you get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js](https://nodejs.org/) (v18+)
- [npm](https://www.npmjs.com/)

## Getting Started

### 1. Clone the repository

```bash
git clone https://github.com/misha-mad/vercel-rpc.git
cd vercel-rpc
```

### 2. Install dependencies

```bash
# Install Node dependencies for the demo app
cd demo
npm install
cd ..
```

### 3. Build the Rust workspace

```bash
cargo build --workspace
```

## Development Workflow

### Running the demo app

```bash
cd demo
npm run dev
```

This starts the RPC watcher (auto-regenerates TypeScript types on `.rs` changes) and the Vite dev server in parallel.

### Generating TypeScript bindings manually

```bash
cd demo
npm run generate
```

### Code formatting & linting

```bash
# Rust
cargo fmt --all
cargo clippy --workspace

# TypeScript / Svelte (from demo/)
cd demo
npm run format
npm run lint
```

## Project Structure

The project is a Cargo workspace with two main crates and a SvelteKit demo:

| Path                | Description                                          |
|---------------------|------------------------------------------------------|
| `crates/rpc-macro/` | Proc-macro crate (`#[rpc_query]`, `#[rpc_mutation]`) |
| `crates/rpc-cli/`   | CLI binary — `scan`, `generate`, `watch` commands    |
| `demo/`             | SvelteKit demo app with Rust lambdas in `demo/api/`  |

See the [README](README.md) for a detailed file tree.

## Testing

### Run all tests

```bash
cd demo
npm run test:all
```

This runs Rust tests, Vitest integration tests, and Playwright e2e tests sequentially.

### Run tests individually

```bash
# Rust unit & integration tests (from project root)
cargo test --workspace

# Vitest integration tests (from demo/)
cd demo
npm run test:integration

# Playwright e2e tests (from demo/)
cd demo
npm run test:e2e
```

### Test structure

| Suite       | Location                                    | Runner       | What it covers                                   |
|-------------|---------------------------------------------|--------------|--------------------------------------------------|
| Rust        | `crates/*/src/` and `crates/rpc-cli/tests/` | `cargo test` | Parser, codegen, macro expansion, type mapping   |
| Integration | `demo/tests/integration/`                   | Vitest       | End-to-end codegen pipeline (Rust → TypeScript)  |
| E2E         | `demo/tests/e2e/`                           | Playwright   | UI interactions and API responses in the browser |

## npm Scripts Reference

All scripts are run from the `demo/` directory:

| Script                     | Description                            |
|----------------------------|----------------------------------------|
| `npm run dev`              | Start watcher + Vite dev server        |
| `npm run build`            | Generate types + production build      |
| `npm run generate`         | One-time TypeScript codegen            |
| `npm run preview`          | Preview production build               |
| `npm run check`            | Svelte type checking                   |
| `npm run lint`             | Prettier + ESLint check                |
| `npm run format`           | Auto-format with Prettier              |
| `npm run test`             | Rust tests + Vitest                    |
| `npm run test:rust`        | Rust tests only                        |
| `npm run test:integration` | Vitest only                            |
| `npm run test:e2e`         | Playwright only                        |
| `npm run test:all`         | All tests (Rust + Vitest + Playwright) |

## Making Changes

1. **Create a branch** from `main`:
   ```bash
   git checkout -b feat/my-feature
   ```

2. **Make your changes** — follow the existing code style in each language (Rust / TypeScript / Svelte).

3. **Add or update tests** for any new functionality.

4. **Run the full test suite** to make sure nothing is broken:
   ```bash
   cargo test --workspace
   cd demo && npm run test:integration
   ```

5. **Open a Pull Request** against `main` with a clear description of what you changed and why.

## Code Style

- **Rust**: Follow standard Rust conventions. Run `cargo fmt` and `cargo clippy` before committing.
- **TypeScript / Svelte**: Follow the project's Prettier and ESLint configuration. Run `npm run format` and `npm run lint` from `demo/`.
- Auto-generated files (`rpc-types.ts`, `rpc-client.ts`) should not be edited manually.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project: **MIT OR Apache-2.0**.
