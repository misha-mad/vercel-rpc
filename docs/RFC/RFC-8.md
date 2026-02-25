# RFC-8: React Reactive Wrappers

- **Status:** Implemented
- **Topic:** Generated React hooks for queries and mutations
- **Date:** February 2026

## 1. Summary

Generate an optional React hook file (`rpc.react.ts`) that provides `useQuery` and `useMutation` hooks. These wrap the plain-promise `RpcClient` with reactive state (`useState`, `useEffect`), giving components a TanStack Query–style API with zero external dependencies.

## 2. Motivation

The generated `RpcClient` returns raw promises — great for libraries and SSR, but inconvenient in reactive UI components:

```tsx
import { useState, useEffect } from "react";
import { rpc } from "./rpc";
import type { UserProfile, RpcError } from "./rpc-types";

function CurrentUser() {
  // Manual state management for every query
  const [data, setData] = useState<UserProfile | undefined>();
  const [error, setError] = useState<RpcError | undefined>();
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    rpc.query("current_user")
      .then(d => { setData(d); })
      .catch(e => { setError(e); })
      .finally(() => { setIsLoading(false); });
  }, []);

  if (isLoading) return <Spinner />;
  if (error) return <ErrorBanner error={error} />;
  return <p>Hello, {data.name}</p>;
}
```

This boilerplate is repeated in every component. A reactive hook eliminates it:

```tsx
import { rpc } from "./rpc";
import { useQuery } from "./rpc.react";

function CurrentUser() {
  const user = useQuery(rpc, "current_user");

  if (user.isLoading) return <Spinner />;
  if (user.isError) return <ErrorBanner error={user.error} />;
  return <p>Hello, {user.data.name}</p>;
}
```

| Without hooks                                | With hooks                          |
|----------------------------------------------|-------------------------------------|
| ~12 lines of manual state + effect per query | 1 line                              |
| Error/loading state easy to forget           | Always tracked                      |
| No auto-refetch, no reactive input           | Built-in                            |
| No lifecycle callbacks                       | `onSuccess`, `onError`, `onSettled` |

## 3. Design Principles

1. **React 18+ first** — uses hooks (`useState`, `useEffect`, `useCallback`, `useRef`) natively. No class component compatibility.
2. **Zero dependencies** — everything is generated; no npm packages to install or version-match. `react` is a peer dependency only.
3. **Same types, same client** — hooks import from the generated `rpc-client.ts` and `rpc-types.ts`. No parallel type system.
4. **Opt-in** — generated only when `output.react` is configured. Projects that don't need reactive hooks pay nothing.
5. **Value-based input** — input is passed as a plain value, not a getter. React re-renders the component on state change, so the hook receives the latest input on every render.
6. **Composable** — hooks return plain objects with plain properties. They compose with other hooks and React patterns naturally.

## 4. Configuration

### 4.1 Config File

New optional `react` field in `[output]`:

```toml
[output]
types  = "src/rpc-types.ts"
client = "src/rpc-client.ts"
react  = "src/rpc.react.ts"     # NEW — opt-in
```

When `react` is omitted (default), no hook file is generated.

### 4.2 CLI Flag

```
metaxy generate --react-output src/rpc.react.ts
```

### 4.3 Import Path

The generated hook file imports from the client file. The import specifier is derived from the relative path between `output.react` and `output.client`, using `output.imports.extension` for the suffix:

```typescript
// If react = "src/rpc.react.ts" and client = "src/rpc-client.ts"
import { type RpcClient, RpcError, type CallOptions } from "./rpc-client";
```

### 4.4 Config Struct Changes

```rust
pub struct OutputConfig {
    pub types: PathBuf,
    pub client: PathBuf,
    pub svelte: Option<PathBuf>,
    pub react: Option<PathBuf>,   // NEW
    pub imports: ImportsConfig,
}
```

```rust
pub struct CliOverrides {
    // ... existing fields ...
    pub react_output: Option<PathBuf>,  // NEW
}
```

## 5. API Surface

### 5.1 `useQuery`

For queries with input (non-void):

```typescript
function useQuery<K extends QueryKey>(
  client: RpcClient,
  key: K,
  input: QueryInput<K>,
  options?: QueryOptions<K>,
): QueryResult<K>;
```

For queries without input (void):

```typescript
function useQuery<K extends VoidQueryKey>(
  client: RpcClient,
  key: K,
  options?: QueryOptions<K>,
): QueryResult<K>;
```

#### `QueryOptions<K>`

```typescript
interface QueryOptions<K extends QueryKey> {
  /** Whether to execute the query. @default true */
  enabled?: boolean;

  /** Auto-refetch interval in milliseconds. Set to 0 or omit to disable. */
  refetchInterval?: number;

  /** Initial data shown before the first fetch completes. */
  placeholderData?: QueryOutput<K>;

  /** Per-call options forwarded to client.query(). */
  callOptions?: CallOptions;

  /** Called when the query succeeds. */
  onSuccess?: (data: QueryOutput<K>) => void;

  /** Called when the query fails. */
  onError?: (error: RpcError) => void;

  /** Called when the query settles (success or failure). */
  onSettled?: () => void;
}
```

#### `QueryResult<K>`

```typescript
type QueryStatus = "idle" | "loading" | "success" | "error";

interface QueryResult<K extends QueryKey> {
  /** The latest successfully resolved data, or placeholderData. */
  readonly data: QueryOutput<K> | undefined;

  /** The error from the most recent failed fetch, cleared on success. */
  readonly error: RpcError | undefined;

  /** Current status of the query state machine. */
  readonly status: QueryStatus;

  /** True while a fetch is in-flight (including the initial fetch). */
  readonly isLoading: boolean;

  /** True after the first successful fetch. Stays true across refetches. */
  readonly isSuccess: boolean;

  /** True when the most recent fetch failed. */
  readonly isError: boolean;

  /** True when placeholderData is being shown and no real fetch has completed yet. */
  readonly isPlaceholderData: boolean;

  /** Manually trigger a refetch. */
  refetch: () => Promise<void>;
}
```

### 5.2 `useMutation`

```typescript
function useMutation<K extends MutationKey>(
  client: RpcClient,
  key: K,
  options?: MutationOptions<K>,
): MutationResult<K>;
```

#### `MutationOptions<K>`

```typescript
interface MutationOptions<K extends MutationKey> {
  /** Per-call options forwarded to client.mutate(). */
  callOptions?: CallOptions;

  /** Called when the mutation succeeds. */
  onSuccess?: (data: MutationOutput<K>) => void;

  /** Called when the mutation fails. */
  onError?: (error: RpcError) => void;

  /** Called when the mutation settles (success or failure). */
  onSettled?: () => void;
}
```

#### `MutationResult<K>`

For mutations with input (non-void):

```typescript
type MutationStatus = "idle" | "loading" | "success" | "error";

interface MutationResult<K extends MutationKey> {
  /** Execute the mutation with the given input. Rejects on error. */
  mutate: (input: MutationInput<K>) => Promise<void>;

  /** Execute the mutation and return the result. Rejects on error. */
  mutateAsync: (input: MutationInput<K>) => Promise<MutationOutput<K>>;

  /** The latest successfully resolved data. */
  readonly data: MutationOutput<K> | undefined;

  /** The error from the most recent failed mutation, cleared on next attempt. */
  readonly error: RpcError | undefined;

  /** Current status of the mutation state machine. */
  readonly status: MutationStatus;

  /** True while a mutation is in-flight. */
  readonly isLoading: boolean;

  /** True after the most recent mutation succeeded. */
  readonly isSuccess: boolean;

  /** True when the most recent mutation failed. */
  readonly isError: boolean;

  /** Reset state back to idle (clear data, error, status). */
  reset: () => void;
}
```

For void-input mutations, `mutate` and `mutateAsync` take no arguments:

```typescript
mutate: () => Promise<void>;
mutateAsync: () => Promise<MutationOutput<K>>;
```

## 6. Usage Examples

### 6.1 Basic Query

```tsx
import { rpc } from "./rpc";
import { useQuery } from "./rpc.react";

function HealthStatus() {
  const health = useQuery(rpc, "health_check");

  return <p>Status: {health.data?.status ?? "loading..."}</p>;
}
```

### 6.2 Query with Reactive Input

```tsx
import { useState } from "react";
import { useQuery } from "./rpc.react";

function UserProfile() {
  const [userId, setUserId] = useState(1);

  // Automatically refetches when userId changes
  const user = useQuery(rpc, "get_user", { id: userId });

  return (
    <>
      <input
        type="number"
        value={userId}
        onChange={(e) => setUserId(Number(e.target.value))}
      />
      <p>{user.isLoading ? "Loading..." : user.data?.name}</p>
    </>
  );
}
```

### 6.3 Conditional Query

```tsx
function Profile({ token }: { token: string | null }) {
  const profile = useQuery(rpc, "get_profile", {
    enabled: token !== null,
  });

  return <div>{profile.data?.name}</div>;
}
```

### 6.4 Polling

```tsx
function ServerStats() {
  const stats = useQuery(rpc, "server_stats", {
    refetchInterval: 5000, // every 5 seconds
  });

  return <pre>{JSON.stringify(stats.data, null, 2)}</pre>;
}
```

### 6.5 Mutation

```tsx
import { useMutation } from "./rpc.react";

function UpdateProfile() {
  const updateName = useMutation(rpc, "update_profile", {
    onSuccess: () => alert("Saved!"),
  });

  return (
    <button
      onClick={() => updateName.mutate({ name: "Alice" })}
      disabled={updateName.isLoading}
    >
      {updateName.isLoading ? "Saving..." : "Save"}
    </button>
  );
}
```

### 6.6 Mutation with Async Result

```tsx
import { useNavigate } from "react-router-dom";

function Checkout({ cart }: { cart: CartItem[] }) {
  const navigate = useNavigate();
  const order = useMutation(rpc, "create_order");

  async function handleSubmit() {
    try {
      const result = await order.mutateAsync({ items: cart });
      navigate(`/orders/${result.id}`);
    } catch (e) {
      // error is also available as order.error
    }
  }

  return <button onClick={handleSubmit}>Place Order</button>;
}
```

### 6.7 Dependent Queries

```tsx
function UserPosts() {
  const user = useQuery(rpc, "current_user");

  // Only fires after user.data is available
  const posts = useQuery(rpc, "user_posts", { userId: user.data?.id ?? 0 }, {
    enabled: user.isSuccess,
  });

  if (user.isLoading) return <Spinner />;
  return <PostList posts={posts.data ?? []} />;
}
```

## 7. React Implementation

### 7.1 File Extension

The generated file uses the standard `.ts` extension. Unlike Svelte 5 runes which require `.svelte.ts`, React hooks work in any `.ts` or `.tsx` file.

### 7.2 `useQuery` Implementation Sketch

```typescript
// rpc.react.ts (generated)

import { useState, useEffect, useCallback, useRef } from "react";
import type { RpcClient, CallOptions, RpcError } from "./rpc-client";
import type { Procedures } from "./rpc-types";

// ... type helpers (QueryKey, MutationKey, etc.) ...

export function useQuery<K extends QueryKey>(
  client: RpcClient,
  key: K,
  inputOrOptions?: QueryInput<K> | QueryOptions<K>,
  maybeOptions?: QueryOptions<K>,
): QueryResult<K> {
  // Resolve overloaded arguments
  const hasInput = maybeOptions !== undefined
    || (inputOrOptions !== undefined && !isQueryOptions(inputOrOptions));
  const input = hasInput ? (inputOrOptions as QueryInput<K>) : undefined;
  const options = (hasInput ? maybeOptions : inputOrOptions) as
    | QueryOptions<K>
    | undefined;

  const [data, setData] = useState<QueryOutput<K> | undefined>(
    options?.placeholderData,
  );
  const [error, setError] = useState<RpcError | undefined>();
  const [status, setStatus] = useState<QueryStatus>("idle");

  // Stable refs for callbacks to avoid re-triggering effects
  const optionsRef = useRef(options);
  optionsRef.current = options;

  const fetch = useCallback(async () => {
    const opts = optionsRef.current;
    const enabled = opts?.enabled ?? true;
    if (!enabled) return;

    setStatus("loading");
    setError(undefined);
    try {
      const result = await client.query(key, ...(input !== undefined ? [input] : []),
        ...(opts?.callOptions ? [opts.callOptions] : [])) as QueryOutput<K>;
      setData(result);
      setStatus("success");
      opts?.onSuccess?.(result);
    } catch (e) {
      const err = e as RpcError;
      setError(err);
      setStatus("error");
      opts?.onError?.(err);
    } finally {
      opts?.onSettled?.();
    }
  }, [client, key, JSON.stringify(input)]);

  useEffect(() => {
    const enabled = options?.enabled ?? true;
    if (!enabled) return;

    fetch();

    if (options?.refetchInterval) {
      const interval = setInterval(fetch, options.refetchInterval);
      return () => clearInterval(interval);
    }
  }, [fetch, options?.enabled, options?.refetchInterval]);

  return {
    data,
    error,
    status,
    isLoading: status === "loading",
    isSuccess: status === "success",
    isError: status === "error",
    isPlaceholderData: status !== "success" && data !== undefined,
    refetch: fetch,
  };
}
```

### 7.3 `useMutation` Implementation Sketch

```typescript
export function useMutation<K extends MutationKey>(
  client: RpcClient,
  key: K,
  options?: MutationOptions<K>,
): MutationResult<K> {
  const [data, setData] = useState<MutationOutput<K> | undefined>();
  const [error, setError] = useState<RpcError | undefined>();
  const [status, setStatus] = useState<MutationStatus>("idle");

  // Stable ref for callbacks
  const optionsRef = useRef(options);
  optionsRef.current = options;

  const execute = useCallback(
    async (input?: MutationInput<K>): Promise<MutationOutput<K>> => {
      const opts = optionsRef.current;
      setStatus("loading");
      setError(undefined);
      try {
        const result = await client.mutate(key, ...(input !== undefined ? [input] : []),
          ...(opts?.callOptions ? [opts.callOptions] : [])) as MutationOutput<K>;
        setData(result);
        setStatus("success");
        opts?.onSuccess?.(result);
        return result;
      } catch (e) {
        const err = e as RpcError;
        setError(err);
        setStatus("error");
        opts?.onError?.(err);
        throw e;
      } finally {
        opts?.onSettled?.();
      }
    },
    [client, key],
  );

  const reset = useCallback(() => {
    setData(undefined);
    setError(undefined);
    setStatus("idle");
  }, []);

  return {
    mutate: async (input?: MutationInput<K>) => { await execute(input); },
    mutateAsync: (input?: MutationInput<K>) => execute(input),
    data,
    error,
    status,
    isLoading: status === "loading",
    isSuccess: status === "success",
    isError: status === "error",
    reset,
  };
}
```

### 7.4 Reactivity Model

| Feature                      | Mechanism                                                              |
|------------------------------|------------------------------------------------------------------------|
| `data`, `error`              | `useState` — triggers re-render on change                              |
| `status`                     | `useState<Status>` — single enum for the state machine                 |
| `isLoading`, `isSuccess`, …  | Derived inline: `status === "loading"`, `status === "success"`, etc.   |
| `isPlaceholderData`          | `status !== "success" && data !== undefined` — true only before fetch  |
| Auto-refetch on input change | `useEffect` with `JSON.stringify(input)` in `useCallback` deps         |
| `enabled` flag               | Checked in `useEffect` deps — plain boolean, no getter needed          |
| Polling cleanup              | `useEffect` returns cleanup function for `clearInterval`               |
| Stable callbacks             | `useCallback` for `fetch`/`execute`/`reset`; `useRef` for options      |

### 7.5 `JSON.stringify` for Input Dependencies

React's `useEffect` and `useCallback` use referential equality for dependency checks. Since `input` is typically a new object literal on each render (e.g. `{ id: userId }`), a naive `[input]` dep would re-trigger every render. `JSON.stringify(input)` produces a stable string for structural comparison:

```typescript
// Without JSON.stringify — refetches every render (broken)
const fetch = useCallback(() => { ... }, [client, key, input]);

// With JSON.stringify — refetches only when values change (correct)
const fetch = useCallback(() => { ... }, [client, key, JSON.stringify(input)]);
```

This is a well-known React pattern for object deps. It works correctly for all JSON-serializable input types, which covers all RPC input types generated from Rust structs.

### 7.6 `useRef` for Callback Stability

Lifecycle callbacks (`onSuccess`, `onError`, `onSettled`) are stored in a `useRef` rather than included in hook dependencies. This prevents the effect from re-running when the parent component passes new inline callback functions:

```tsx
// Without useRef — re-fetches every render because onSuccess is a new function
const user = useQuery(rpc, "current_user", {
  onSuccess: (data) => console.log(data), // new reference each render
});

// With useRef — callbacks update without triggering a refetch
```

## 8. Codegen Changes

### 8.1 New Module

A new codegen module `codegen/react.rs` generates the hook file:

```rust
pub fn generate_react_file(
    manifest: &Manifest,
    client_import_path: &str,
    types_import_path: &str,
    preserve_docs: bool,
) -> String;
```

### 8.2 Generated File Structure

```
// Header
// Import { useState, useEffect, useCallback, useRef } from "react"
// Import { type RpcClient, RpcError, type CallOptions } from "./rpc-client"
// Import { type Procedures, ...types } from "./rpc-types"
// Re-export types
//
// Type helpers (QueryKey, MutationKey, etc.)
// VoidQueryKey / VoidMutationKey union types (from manifest)
// QueryOptions, QueryResult interfaces
// MutationOptions, MutationResult interfaces
//
// isQueryOptions() type guard (for overload resolution)
// useQuery() hook
// useMutation() hook
```

### 8.3 Void vs Non-Void Handling

The codegen emits type unions from the manifest to distinguish void-input procedures:

```typescript
type VoidQueryKey = "health_check" | "server_stats";
type NonVoidQueryKey = Exclude<QueryKey, VoidQueryKey>;
```

The `useQuery` hook uses TypeScript overloads so void-input queries don't require an `input` argument:

```typescript
// Void input — no input parameter
export function useQuery<K extends VoidQueryKey>(
  client: RpcClient, key: K, options?: QueryOptions<K>
): QueryResult<K>;

// Non-void input — requires input value
export function useQuery<K extends NonVoidQueryKey>(
  client: RpcClient, key: K, input: QueryInput<K>, options?: QueryOptions<K>
): QueryResult<K>;
```

### 8.4 Conditional Emission

| Manifest state          | What's emitted                                     |
|-------------------------|----------------------------------------------------|
| Has queries + mutations | Full file with both `useQuery` and `useMutation`   |
| Queries only            | `useQuery` only, no mutation types                 |
| Mutations only          | `useMutation` only, no query types                 |
| Empty manifest          | Not generated (skip file entirely)                 |

### 8.5 `commands.rs` Changes

`cmd_generate` writes the React file when `config.output.react` is `Some`:

```rust
if let Some(react_path) = &config.output.react {
    let react_content = codegen::react::generate_react_file(
        &manifest,
        &client_import_specifier,
        &types_import_specifier,
        config.codegen.preserve_docs,
    );
    write_file(react_path, &react_content)?;
}
```

### 8.6 `watch.rs` Changes

The file watcher includes the React output path in regeneration triggers:

```rust
if let Some(react_path) = &config.output.react {
    watched_outputs.push(react_path.clone());
}
```

### 8.7 `main.rs` Changes

Register the new CLI flag:

```rust
.arg(
    Arg::new("react-output")
        .long("react-output")
        .value_name("PATH")
        .help("Output path for React hooks file"),
)
```

## 9. Test Plan

### Unit Tests

| Test                                     | Description                                                           |
|------------------------------------------|-----------------------------------------------------------------------|
| `react_file_imports_client_and_types`    | Output imports `RpcClient`, `RpcError`, `CallOptions`, `Procedures`   |
| `react_file_imports_react_hooks`         | Output imports `useState`, `useEffect`, `useCallback`, `useRef`       |
| `react_file_contains_use_query`          | Output contains `useQuery` function                                   |
| `react_file_contains_use_mutation`       | Output contains `useMutation` function                                |
| `react_void_query_no_input_overload`     | Void-input queries have overload without `input` parameter            |
| `react_non_void_query_input_overload`    | Non-void queries require `input: QueryInput<K>` parameter             |
| `react_void_mutation_no_input`           | Void-input mutation `mutate()` takes no arguments                     |
| `react_queries_only_no_mutation`         | Queries-only manifest omits `useMutation`                             |
| `react_mutations_only_no_query`          | Mutations-only manifest omits `useQuery`                              |
| `react_empty_manifest_not_generated`     | Empty manifest produces empty string                                  |
| `react_query_status_enum`                | Output contains `QueryStatus` type with idle/loading/success/error    |
| `react_mutation_status_enum`             | Output contains `MutationStatus` type with idle/loading/success/error |
| `react_query_result_has_is_placeholder`  | `QueryResult` includes `isPlaceholderData` boolean field              |
| `react_status_derives_booleans`          | `isLoading`/`isSuccess`/`isError` derived from `status` comparisons   |
| `react_uses_use_state`                   | Output contains `useState` calls                                      |
| `react_uses_use_effect`                  | Output contains `useEffect` call                                      |
| `react_uses_use_callback`                | Output contains `useCallback` calls                                   |
| `react_uses_use_ref`                     | Output contains `useRef` calls                                        |
| `react_refetch_interval_cleanup`         | `useEffect` returns cleanup for `clearInterval`                       |
| `react_query_result_has_refetch`         | Return object includes `refetch` method                               |
| `react_mutation_result_has_reset`        | Return object includes `reset` method                                 |
| `react_mutation_result_has_mutate_async` | Return object includes `mutateAsync` method                           |
| `react_json_stringify_input_deps`        | `useCallback` deps include `JSON.stringify(input)`                    |
| `react_options_ref_pattern`              | Options stored in `useRef` for callback stability                     |
| `react_is_query_options_guard`           | Output contains `isQueryOptions` type guard function                  |
| `react_input_is_value_not_getter`        | Non-void input parameter is a value, not a getter function            |
| `react_enabled_is_boolean_not_getter`    | `enabled` option is `boolean`, not `boolean \| (() => boolean)`       |
| `react_file_extension_is_ts`             | Output file has `.ts` extension, not `.svelte.ts`                     |
| `snapshot_react_full`                    | Insta snapshot with mixed queries and mutations                       |

### Config Tests

| Test                                            | Description                                          |
|-------------------------------------------------|------------------------------------------------------|
| `config_react_output_default_none`              | Default config has `react: None`                     |
| `config_react_output_parsed`                    | Parses `output.react` from TOML                      |
| `cli_react_output_override`                     | CLI `--react-output` overrides config                |
| `cmd_generate_writes_react_file`                | `cmd_generate` writes the React file when configured |
| `cmd_generate_skips_react_when_not_configured`  | No file written when `output.react` is None          |

## 10. Backward Compatibility

- **No breaking changes.** The React hook file is entirely opt-in via a new config field.
- Existing projects without `output.react` configured see no change.
- The generated `rpc-client.ts` and `rpc-types.ts` are unchanged.
- The existing Svelte wrapper (`output.svelte`) is unaffected.

## 11. Interactions with Existing Features

### 11.1 Request Deduplication (RFC-6)

`useQuery` calls `client.query()` which already deduplicates. Multiple components calling `useQuery` with the same key and input share a single HTTP request automatically.

### 11.2 Per-Call Options (RFC-5)

`QueryOptions.callOptions` and `MutationOptions.callOptions` are forwarded directly to `client.query()` / `client.mutate()`. This allows per-hook `headers`, `timeout`, and `signal` overrides.

### 11.3 Abort / Cleanup

When a component unmounts, React runs `useEffect` cleanup functions. The generated `useEffect` cleans up polling intervals. For abort-on-unmount, callers can pass an `AbortSignal` via `callOptions.signal`:

```tsx
function UserProfile() {
  const controller = useMemo(() => new AbortController(), []);
  const user = useQuery(rpc, "current_user", {
    callOptions: { signal: controller.signal },
  });

  useEffect(() => {
    return () => controller.abort();
  }, [controller]);

  return <p>{user.data?.name}</p>;
}
```

### 11.4 JSDoc / Preserve Docs

When `codegen.preserve_docs = true`, doc comments from Rust source are forwarded to the type helpers and option interfaces in the React file, matching the behavior in the client and types files.

## 12. Future Extensions

These are out of scope for the initial implementation but the design accommodates them:

- **Other frameworks** — Vue (`rpc.vue.ts` with `ref`/`watchEffect`), Solid (`rpc.solid.ts` with `createSignal`/`createEffect`). Each would be a separate codegen module and config field (`output.vue`, `output.solid`).
- **`useInfiniteQuery`** — for cursor/offset-based pagination. Extends the pattern with a `getNextPageParam` option and accumulated `pages` array.
- **Optimistic updates** — `MutationOptions.onMutate` callback that returns rollback data, with automatic rollback on error.
- **Query invalidation** — `queryClient.invalidate("key")` pattern for cache-busting after mutations. Would require a shared query registry.
- **Suspense integration** — React Suspense support via throwing a promise from `useQuery` when data is not yet available, enabling `<Suspense>` boundaries for loading states.

## 13. Implementation Notes

The implementation deviates from the initial design sketches in sections 7.2–7.3 in the following ways:

### Simplified state model

Instead of a `status: QueryStatus` enum with derived booleans, the implementation uses individual `useState` hooks for `data`, `error`, `isLoading`, plus a `hasFetched` flag. This avoids the overhead of a status enum while keeping `isSuccess` semantics correct — `isSuccess` is `hasFetched && error === undefined`, not `data !== undefined` (which would be true with `placeholderData`).

Similarly, `useMutation` uses a `hasSucceeded` flag instead of a status enum, so `isSuccess` remains accurate even when a mutation returns `undefined` as a valid result.

### `isQueryOptions` type guard

The overloaded `useQuery` signature requires disambiguating `input` from `options` at runtime. The implementation uses a key-based guard (`isQueryOptions`) that checks whether all keys of the argument are known option keys (`enabled`, `refetchInterval`, `placeholderData`, `callOptions`, `onSuccess`, `onError`, `onSettled`). This is more robust than the `args.length` heuristic in the original sketch.

### No `status` / `isPlaceholderData` fields

The React implementation omits the `status` and `isPlaceholderData` fields from `QueryResult` and `MutationResult` to keep the generated code minimal. These can be added in a future iteration if needed.

### Svelte fixes shipped alongside

During code review, several issues were identified and fixed in both the React and Svelte codegen:
- **Double `inputFn()` call** — Svelte's `$effect` was calling `inputFn()` for reactive tracking and again inside `fetchData`. Fixed by computing input once and passing it as a parameter.
- **Type safety** — Both hooks replaced `as Function` casts with `(...a: unknown[]) => Promise<unknown>` and used `MutationArgs<K>` instead of `unknown[]` in mutation signatures.
