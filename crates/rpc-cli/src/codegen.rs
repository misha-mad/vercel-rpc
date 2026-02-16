//! TypeScript code generation from a [`Manifest`](crate::model::Manifest).
//!
//! - [`typescript`] — generates `rpc-types.ts` (interfaces, enum types,
//!   `Procedures` map).
//! - [`client`] — generates `rpc-client.ts` (`RpcClient` interface,
//!   `createRpcClient` factory, `RpcError` class, `rpcFetch` helper).

pub mod client;
pub mod typescript;
