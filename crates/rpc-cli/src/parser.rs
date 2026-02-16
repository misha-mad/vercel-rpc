//! Rust source code parser.
//!
//! Walks a directory of `.rs` files, parses each one with [`syn`], and extracts:
//!
//! - RPC procedures (functions annotated with `#[rpc_query]` / `#[rpc_mutation]`)
//! - Struct definitions with `#[derive(Serialize)]`
//! - Enum definitions with `#[derive(Serialize)]`
//!
//! The public entry point is [`scan_directory`], which returns a
//! [`Manifest`](crate::model::Manifest) containing all discovered metadata.

pub mod extract;
pub mod types;

pub use extract::scan_directory;
