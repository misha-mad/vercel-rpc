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

## Branch Protection Rules

The `main` branch is protected with the following rules:

### Pull Request Requirements

- All changes must go through a **Pull Request** — direct pushes to `main` are not allowed.
- Each PR requires **at least 1 approving review**.
- If a designated [Code Owner](https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners) is affected, their approval is required.
- Approvals are **dismissed automatically** when new commits are pushed — re-review is needed.
- The **most recent push** must be approved by someone other than the author.
- All **review conversations must be resolved** before merging.

### CI Checks

Every PR must pass these status checks before merging:

| Check                  | What it runs                              |
|------------------------|-------------------------------------------|
| **Rust Tests**         | `cargo test --workspace`                  |
| **Vitest Integration** | `npm run test:integration` (from `demo/`) |
| **Playwright E2E**     | `npm run test:e2e` (from `demo/`)         |

The PR branch must be **up to date with `main`** before merging. If `main` has new commits, you need to merge them into your branch.

### Merge Policy

- Only **merge commits** are allowed — squash and rebase are disabled.
- **Force pushes** and **branch deletion** of `main` are blocked.

### Quick Checklist

Before requesting a review, make sure:

- [ ] All three CI checks pass (Rust, Vitest, Playwright)
- [ ] Branch is up to date with `main`
- [ ] Code follows the project's style conventions
- [ ] New functionality has tests
- [ ] Auto-generated files are not edited manually

## Code Style

- **Rust**: Follow standard Rust conventions. Run `cargo fmt` and `cargo clippy` before committing.
- **TypeScript / Svelte**: Follow the project's Prettier and ESLint configuration. Run `npm run format` and `npm run lint` from `demo/`.
- Auto-generated files (`rpc-types.ts`, `rpc-client.ts`) should not be edited manually.

## Conventional Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/) to drive automated versioning and changelog generation via [release-plz](https://release-plz.ieni.dev/).

### Git Hooks Setup

The repository includes a `commit-msg` hook that validates your commit messages locally. To activate it, run once after cloning:

```bash
git config core.hooksPath .githooks
```

### Commit Format

```
<type>(<scope>): <description>
```

### Types

| Type       | Version Bump | Description                          |
|------------|--------------|--------------------------------------|
| `feat`     | Minor        | A new feature                        |
| `fix`      | Patch        | A bug fix                            |
| `docs`     | None         | Documentation only                   |
| `style`    | None         | Formatting, missing semicolons, etc. |
| `refactor` | None         | Code change that neither fixes a bug nor adds a feature |
| `test`     | None         | Adding or updating tests             |
| `chore`    | None         | Build process, dependencies, etc.    |
| `ci`       | None         | CI configuration changes             |

### Scopes

| Scope       | Crate              |
|-------------|---------------------|
| `rpc-macro` | `vercel-rpc-macro`  |
| `rpc-cli`   | `vercel-rpc-cli`    |

### Breaking Changes

A breaking change triggers a **Major** version bump. Use either:

- `feat!:` or `fix!:` prefix (e.g., `feat!: remove deprecated API`)
- `BREAKING CHANGE:` footer in the commit body

### Examples

```bash
# Patch bump for rpc-macro
git commit -m "fix(rpc-macro): handle empty function bodies"

# Minor bump for rpc-cli
git commit -m "feat(rpc-cli): add --output flag for custom output directory"

# No version bump
git commit -m "docs: update installation instructions"

# Major bump (breaking change)
git commit -m "feat(rpc-macro)!: rename #[rpc_query] to #[query]"
```

## License

By contributing, you agree that your contributions will be licensed under the same license as the project: **MIT OR Apache-2.0**.
