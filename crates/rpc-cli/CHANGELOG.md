# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/misha-mad/vercel-rpc/compare/vercel-rpc-cli-v0.1.4...vercel-rpc-cli-v0.2.0) - 2026-02-19

### Added

- *(rpc-cli)* support serde attributes in codegen (RFC-3)

### Other

- *(rpc-cli)* address PR review findings

## [0.1.4](https://github.com/misha-mad/vercel-rpc/compare/vercel-rpc-cli-v0.1.3...vercel-rpc-cli-v0.1.4) - 2026-02-18

### Fixed

- resolve all clippy warnings across workspace
- *(rpc-cli)* remove flaky config tests using set_current_dir

### Other

- *(rpc-cli)* apply best practices from code audit
- *(rpc-cli)* reformat and align code for improved readability
- *(rpc-cli)* move inline tests to integration tests
- *(rpc-cli)* replace mod.rs with named module files (Rust 2018 style)
- center sponsor line, remove codecov badges from crate READMEs
- *(rpc-cli)* remove dead-code branches, exclude rpc-macro from coverage
- *(rpc-cli)* cover remaining uncovered lines across all modules
- *(rpc-cli)* exclude untestable code from tarpaulin coverage
- *(rpc-cli)* add unit tests for main.rs functions

## [0.1.3](https://github.com/misha-mad/vercel-rpc/compare/vercel-rpc-cli-v0.1.2...vercel-rpc-cli-v0.1.3) - 2026-02-16

### Added

- *(rpc-cli)* add CLI flags for all config-only options
- *(rpc-cli)* add `clear_screen` option to watch mode
- *(rpc-cli)* add [codegen.naming] config with fields option
- *(rpc-cli)* add preserve_docs option to [codegen] config
- *(rpc-cli)* add extension field to [output.imports] config
- *(rpc-cli)* add include/exclude glob patterns to [input] config
- *(rpc-cli)* add rpc.config.toml support (RFC-2 Phase 1)

### Other

- add test coverage with cargo-tarpaulin and Codecov
- *(rpc-cli)* update README with `--clear-screen` flag details
- *(rpc-cli)* document rpc.config.toml support in READMEs

## [0.1.2](https://github.com/misha-mad/vercel-rpc/compare/vercel-rpc-cli-v0.1.1...vercel-rpc-cli-v0.1.2) - 2026-02-15

### Other

- *(rpc-cli)* fix alignment in architecture ASCII diagram
- *(rpc-cli)* add crate-level documentation and README

## [0.1.1](https://github.com/misha-mad/vercel-rpc/compare/vercel-rpc-cli-v0.1.0...vercel-rpc-cli-v0.1.1) - 2026-02-15

### Other

- release v0.1.0

## [0.1.0](https://github.com/misha-mad/vercel-rpc/releases/tag/vercel-rpc-cli-v0.1.0) - 2026-02-15

### Other

- add release-plz setup, improve metadata in Cargo.toml files
- Rename project references from "svelte-rust" to "vercel-rpc" across files.
- Add contributing guidelines and update README to reference them
- Update README: Fix incorrect div alignment attribute
- Remove "Tech Stack" section from README.
- Update README formatting and links for `demo` directory
- Add dual licensing (MIT OR Apache-2.0) to crates and update README
- Update README diagram alignment for improved formatting
- Add live demo link and sponsorship section to README
- Add CI workflow and badge to README
- Fix inconsistent box widths in README diagrams
- Normalize table formatting in README.md
- Move demo app to `demo` dir
- Add TypeScript code generation and parsing support for Rust enums
- Replace `rspc` with `vercel-rpc`, introducing a new TypeScript client, query macro, and CLI framework.
- Init
