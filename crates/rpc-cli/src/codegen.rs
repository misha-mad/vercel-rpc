//! TypeScript code generation from a [`Manifest`](crate::model::Manifest).
//!
//! - [`typescript`] — generates `rpc-types.ts` (interfaces, enum types,
//!   `Procedures` map).
//! - [`client`] — generates `rpc-client.ts` (`RpcClient` interface,
//!   `createRpcClient` factory, `RpcError` class, `rpcFetch` helper).
//! - [`svelte`] — generates `rpc.svelte.ts` (Svelte 5 reactive wrappers:
//!   `createQuery`, `createMutation`).
//! - [`react`] — generates `rpc.react.ts` (React hooks wrappers:
//!   `useQuery`, `useMutation`).
//! - [`vue`] — generates `rpc.vue.ts` (Vue 3 Composition API wrappers:
//!   `useQuery`, `useMutation`).
//! - [`solid`] — generates `rpc.solid.ts` (SolidJS reactive primitives:
//!   `createQuery`, `createMutation`).

/// Shorthand for `let _ = writeln!(...)` when writing to a `String` buffer.
///
/// Writing to `String` is infallible, so the result is always safe to discard.
macro_rules! emit {
    ($dst:expr, $($arg:tt)*) => {
        {
            use ::std::fmt::Write as _;
            let _ = writeln!($dst, $($arg)*);
        }
    };
}

pub mod client;
pub mod react;
pub mod svelte;
pub mod typescript;
pub mod solid;
pub mod vue;
