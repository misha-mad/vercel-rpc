# RFC-10: SolidJS Reactive Wrappers

- **Status:** Draft
- **Topic:** Generated SolidJS primitives for queries and mutations
- **Date:** February 2026

## 1. Summary

Generate an optional SolidJS file (`rpc.solid.ts`) that provides `createQuery` and `createMutation` primitives. These wrap the plain-promise `RpcClient` with reactive signals (`createSignal`, `createEffect`), giving components a TanStack Query-style API with zero external dependencies.

## 2. Motivation

The generated `RpcClient` returns raw promises — great for libraries and SSR, but inconvenient in reactive Solid components:

```tsx
import { createSignal, createEffect, onCleanup } from "solid-js";
import { rpc } from "./rpc";
import type { UserProfile, RpcError } from "./rpc-types";

function CurrentUser() {
  // Manual state management for every query
  const [data, setData] = createSignal<UserProfile | undefined>();
  const [error, setError] = createSignal<RpcError | undefined>();
  const [isLoading, setIsLoading] = createSignal(true);

  createEffect(() => {
    rpc.query("current_user")
      .then(d => { setData(d); })
      .catch(e => { setError(e); })
      .finally(() => { setIsLoading(false); });
  });

  return (
    <Show when={!isLoading()} fallback={<Spinner />}>
      <Show when={!error()} fallback={<ErrorBanner error={error()!} />}>
        <p>Hello, {data()!.name}</p>
      </Show>
    </Show>
  );
}
```

This boilerplate is repeated in every component. A reactive primitive eliminates it:

```tsx
import { rpc } from "./rpc";
import { createQuery } from "./rpc.solid";

function CurrentUser() {
  const user = createQuery(rpc, "current_user");

  return (
    <Show when={!user.isLoading()} fallback={<Spinner />}>
      <Show when={!user.isError()} fallback={<ErrorBanner error={user.error()!} />}>
        <p>Hello, {user.data()!.name}</p>
      </Show>
    </Show>
  );
}
```

| Without primitives                          | With primitives                     |
|----------------------------------------------|-------------------------------------|
| ~15 lines of manual signals + effects        | 1 line                              |
| Error/loading state easy to forget           | Always tracked                      |
| No auto-refetch, no reactive input           | Built-in                            |
| No lifecycle callbacks                       | `onSuccess`, `onError`, `onSettled` |

## 3. Design Principles

1. **SolidJS first** — uses `createSignal`, `createEffect`, `onCleanup`. No React compatibility layer.
2. **Zero dependencies** — everything is generated; no npm packages to install. `solid-js` is a peer dependency only.
3. **Same types, same client** — primitives import from the generated `rpc-client.ts` and `rpc-types.ts`. No parallel type system.
4. **Opt-in** — generated only when `output.solid` is configured.
5. **Getter-based input** — input is passed as a getter `() => value`, idiomatic for Solid's fine-grained reactivity. Solid tracks dependencies through function calls, so getters integrate naturally with `createEffect`.
6. **Accessor-based output** — results expose signal getters (`data()`, `error()`, `isLoading()`) following Solid's convention that reactive values are accessed via function calls.

## 4. Configuration

### 4.1 Config File

New optional `solid` field in `[output]`:

```toml
[output]
types  = "src/rpc-types.ts"
client = "src/rpc-client.ts"
solid  = "src/rpc.solid.ts"     # NEW — opt-in
```

When `solid` is omitted (default), no file is generated.

### 4.2 CLI Flag

```
rpc generate --solid-output src/rpc.solid.ts
```

### 4.3 Config Struct Changes

```rust
pub struct OutputConfig {
    pub types: PathBuf,
    pub client: PathBuf,
    pub svelte: Option<PathBuf>,
    pub react: Option<PathBuf>,
    pub vue: Option<PathBuf>,
    pub solid: Option<PathBuf>,   // NEW
    pub imports: ImportsConfig,
}
```

```rust
pub struct CliOverrides {
    // ... existing fields ...
    pub solid_output: Option<PathBuf>,  // NEW
}
```

## 5. API Surface

### 5.1 `createQuery`

For queries with input (non-void):

```typescript
function createQuery<K extends QueryKey>(
  client: RpcClient,
  key: K,
  input: () => QueryInput<K>,
  options?: QueryOptions<K>,
): QueryResult<K>;
```

For queries without input (void):

```typescript
function createQuery<K extends VoidQueryKey>(
  client: RpcClient,
  key: K,
  options?: QueryOptions<K>,
): QueryResult<K>;
```

#### `QueryOptions<K>`

```typescript
interface QueryOptions<K extends QueryKey> {
  /** Whether to execute the query. @default true */
  enabled?: boolean | (() => boolean);

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
interface QueryResult<K extends QueryKey> {
  /** The latest successfully resolved data, or placeholderData. */
  data: () => QueryOutput<K> | undefined;

  /** The error from the most recent failed fetch, cleared on success. */
  error: () => RpcError | undefined;

  /** True while a fetch is in-flight (including the initial fetch). */
  isLoading: () => boolean;

  /** True after the first successful fetch. Stays true across refetches. */
  isSuccess: () => boolean;

  /** True when the most recent fetch failed. */
  isError: () => boolean;

  /** Manually trigger a refetch. */
  refetch: () => Promise<void>;
}
```

### 5.2 `createMutation`

```typescript
function createMutation<K extends MutationKey>(
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

```typescript
interface MutationResult<K extends MutationKey> {
  /** Execute the mutation. Rejects on error. */
  mutate: (...args: MutationArgs<K>) => Promise<void>;

  /** Execute the mutation and return the result. Rejects on error. */
  mutateAsync: (...args: MutationArgs<K>) => Promise<MutationOutput<K>>;

  /** The latest successfully resolved data. */
  data: () => MutationOutput<K> | undefined;

  /** The error from the most recent failed mutation, cleared on next attempt. */
  error: () => RpcError | undefined;

  /** True while a mutation is in-flight. */
  isLoading: () => boolean;

  /** True after the most recent mutation succeeded. */
  isSuccess: () => boolean;

  /** True when the most recent mutation failed. */
  isError: () => boolean;

  /** Reset state back to idle (clear data, error, status). */
  reset: () => void;
}
```

## 6. Usage Examples

### 6.1 Basic Query

```tsx
import { rpc } from "./rpc";
import { createQuery } from "./rpc.solid";

function HealthStatus() {
  const health = createQuery(rpc, "health_check");

  return <p>Status: {health.data()?.status ?? "loading..."}</p>;
}
```

### 6.2 Query with Reactive Input

```tsx
import { createSignal } from "solid-js";
import { createQuery } from "./rpc.solid";

function UserProfile() {
  const [userId, setUserId] = createSignal(1);

  // Automatically refetches when userId() changes
  const user = createQuery(rpc, "get_user", () => ({ id: userId() }));

  return (
    <>
      <input
        type="number"
        value={userId()}
        onInput={(e) => setUserId(Number(e.currentTarget.value))}
      />
      <p>{user.isLoading() ? "Loading..." : user.data()?.name}</p>
    </>
  );
}
```

### 6.3 Conditional Query

```tsx
function Profile(props: { token: string | null }) {
  const profile = createQuery(rpc, "get_profile", {
    enabled: () => props.token !== null,
  });

  return <div>{profile.data()?.name}</div>;
}
```

### 6.4 Polling

```tsx
function ServerStats() {
  const stats = createQuery(rpc, "server_stats", {
    refetchInterval: 5000,
  });

  return <pre>{JSON.stringify(stats.data(), null, 2)}</pre>;
}
```

### 6.5 Mutation

```tsx
import { createMutation } from "./rpc.solid";

function UpdateProfile() {
  const updateName = createMutation(rpc, "update_profile", {
    onSuccess: () => alert("Saved!"),
  });

  return (
    <button
      onClick={() => updateName.mutate({ name: "Alice" })}
      disabled={updateName.isLoading()}
    >
      {updateName.isLoading() ? "Saving..." : "Save"}
    </button>
  );
}
```

### 6.6 Mutation with Async Result

```tsx
import { useNavigate } from "@solidjs/router";

function Checkout(props: { cart: CartItem[] }) {
  const navigate = useNavigate();
  const order = createMutation(rpc, "create_order");

  async function handleSubmit() {
    try {
      const result = await order.mutateAsync({ items: props.cart });
      navigate(`/orders/${result.id}`);
    } catch {
      // error is also available as order.error()
    }
  }

  return <button onClick={handleSubmit}>Place Order</button>;
}
```

### 6.7 Dependent Queries

```tsx
function UserPosts() {
  const user = createQuery(rpc, "current_user");

  // Only fires after user data is available
  const posts = createQuery(rpc, "user_posts", () => ({ userId: user.data()?.id ?? 0 }), {
    enabled: () => user.isSuccess(),
  });

  return (
    <Show when={!user.isLoading()} fallback={<Spinner />}>
      <PostList posts={posts.data() ?? []} />
    </Show>
  );
}
```

## 7. Solid Implementation

### 7.1 File Extension

The generated file uses `.ts` extension. Solid components and primitives work in any `.ts` or `.tsx` file.

### 7.2 `createQuery` Implementation Sketch

```typescript
// rpc.solid.ts (generated)

import { createSignal, createEffect, onCleanup } from "solid-js";
import { type RpcClient, RpcError, type CallOptions } from "./rpc-client";
import type { Procedures } from "./rpc-types";

// ... type helpers (QueryKey, MutationKey, etc.) ...

export function createQuery<K extends QueryKey>(
  client: RpcClient,
  ...args: unknown[]
): QueryResult<K> {
  const key = args[0] as K;

  const inputFn = typeof args[1] === "function"
    ? args[1] as () => QueryInput<K>
    : undefined;
  const options = (typeof args[1] === "object" ? args[1] : args[2]) as
    | QueryOptions<K>
    | undefined;

  const [data, setData] = createSignal<QueryOutput<K> | undefined>(options?.placeholderData);
  const [error, setError] = createSignal<RpcError | undefined>();
  const [isLoading, setIsLoading] = createSignal(false);
  const [hasFetched, setHasFetched] = createSignal(false);

  async function fetchData(input?: QueryInput<K>) {
    setIsLoading(true);
    setError(undefined);
    try {
      const callArgs: unknown[] = [key];
      if (input !== undefined) callArgs.push(input);
      if (options?.callOptions) callArgs.push(options.callOptions);
      const result = await (client.query as (...a: unknown[]) => Promise<unknown>)(
        ...callArgs
      ) as QueryOutput<K>;
      setData(() => result);
      setHasFetched(true);
      options?.onSuccess?.(result);
    } catch (e) {
      const err = e as RpcError;
      setError(() => err);
      options?.onError?.(err);
    } finally {
      setIsLoading(false);
      options?.onSettled?.();
    }
  }

  createEffect(() => {
    const enabled = typeof options?.enabled === "function"
      ? options.enabled()
      : (options?.enabled ?? true);
    const input = inputFn?.();
    if (!enabled) return;

    void fetchData(input);

    if (options?.refetchInterval) {
      const interval = setInterval(() => fetchData(inputFn?.()), options.refetchInterval);
      onCleanup(() => clearInterval(interval));
    }
  });

  return {
    data,
    error,
    isLoading,
    isSuccess: () => hasFetched() && error() === undefined,
    isError: () => error() !== undefined,
    refetch: () => fetchData(inputFn?.()),
  };
}
```

### 7.3 `useMutation` Implementation Sketch

```typescript
export function createMutation<K extends MutationKey>(
  client: RpcClient,
  key: K,
  options?: MutationOptions<K>,
): MutationResult<K> {
  const [data, setData] = createSignal<MutationOutput<K> | undefined>();
  const [error, setError] = createSignal<RpcError | undefined>();
  const [isLoading, setIsLoading] = createSignal(false);
  const [hasSucceeded, setHasSucceeded] = createSignal(false);

  async function execute(...input: MutationArgs<K>): Promise<MutationOutput<K>> {
    setIsLoading(true);
    setError(undefined);
    setHasSucceeded(false);
    try {
      const callArgs: unknown[] = [key];
      if (input.length > 0) callArgs.push(input[0]);
      if (options?.callOptions) callArgs.push(options.callOptions);
      const result = await (client.mutate as (...a: unknown[]) => Promise<unknown>)(
        ...callArgs
      ) as MutationOutput<K>;
      setData(() => result);
      setHasSucceeded(true);
      options?.onSuccess?.(result);
      return result;
    } catch (e) {
      const err = e as RpcError;
      setError(() => err);
      options?.onError?.(err);
      throw e;
    } finally {
      setIsLoading(false);
      options?.onSettled?.();
    }
  }

  function reset() {
    setData(undefined);
    setError(undefined);
    setIsLoading(false);
    setHasSucceeded(false);
  }

  return {
    mutate: async (...args: MutationArgs<K>) => { await execute(...args); },
    mutateAsync: (...args: MutationArgs<K>) => execute(...args),
    data,
    error,
    isLoading,
    isSuccess: () => hasSucceeded() && error() === undefined,
    isError: () => error() !== undefined,
    reset,
  };
}
```

### 7.4 Reactivity Model

| Feature                      | Mechanism                                                                |
|------------------------------|--------------------------------------------------------------------------|
| `data`, `error`, `isLoading` | `createSignal()` — accessed via getter function, triggers fine-grained updates |
| `isSuccess`, `isError`       | Derived getters composing signal reads                                   |
| Auto-refetch on input change | `createEffect()` reads `inputFn()` — Solid tracks the dependency         |
| `enabled` flag               | Read inside `createEffect` — reactive when passed as a getter            |
| Polling cleanup              | `onCleanup()` inside `createEffect` clears interval on re-run           |
| Component unmount cleanup    | `onCleanup()` — Solid calls it when owner scope is disposed             |

### 7.5 Solid vs Svelte Comparison

SolidJS and Svelte 5 share a similar reactivity model, making the generated code structurally very close:

| Aspect              | SolidJS                    | Svelte 5                   |
|----------------------|----------------------------|----------------------------|
| State primitive      | `createSignal()`           | `$state()`                 |
| Side effects         | `createEffect()`           | `$effect()`                |
| Cleanup              | `onCleanup()`              | Return from `$effect`      |
| Value access         | `signal()` (getter call)   | `variable` (direct)        |
| Value mutation       | `setSignal(value)`         | `variable = value`         |
| Dependency tracking  | Automatic (getter calls)   | Automatic (compiler)       |
| File extension       | `.ts`                      | `.svelte.ts`               |
| Primitive name       | `createQuery`              | `createQuery`              |

Key difference: Solid signals are accessed via function calls (`data()`), while Svelte 5 runes use direct variable access. The generated `QueryResult`/`MutationResult` interfaces reflect this — Solid exposes `data: () => T` accessors, Svelte uses `get data()` property getters.

## 8. Codegen Changes

### 8.1 New Module

A new codegen module `codegen/solid.rs` generates the primitives file:

```rust
pub fn generate_solid_file(
    manifest: &Manifest,
    client_import_path: &str,
    types_import_path: &str,
    preserve_docs: bool,
) -> String;
```

### 8.2 Generated File Structure

```
// Header
// Import { createSignal, createEffect, onCleanup } from "solid-js"
// Import { type RpcClient, RpcError, type CallOptions } from "./rpc-client"
// Import { type Procedures, ...types } from "./rpc-types"
// Re-export types
//
// Type helpers (QueryKey, MutationKey, etc.)
// VoidQueryKey / VoidMutationKey union types (from manifest)
// QueryOptions, QueryResult interfaces
// MutationOptions, MutationResult interfaces
//
// createQuery() primitive with overloads
// createMutation() primitive
```

### 8.3 Conditional Emission

| Manifest state          | What's emitted                                      |
|-------------------------|-----------------------------------------------------|
| Has queries + mutations | Full file with both `createQuery` and `createMutation` |
| Queries only            | `createQuery` only, no mutation types               |
| Mutations only          | `createMutation` only, no query types               |
| Empty manifest          | Not generated (skip file entirely)                  |

## 9. Test Plan

### Unit Tests

| Test                                     | Description                                                           |
|------------------------------------------|-----------------------------------------------------------------------|
| `solid_imports_client_and_types`         | Output imports `RpcClient`, `RpcError`, `CallOptions`, `Procedures`   |
| `solid_imports_solid`                    | Output imports `createSignal`, `createEffect`, `onCleanup` from `"solid-js"` |
| `solid_contains_create_query`           | Output contains `createQuery` function                                |
| `solid_contains_create_mutation`        | Output contains `createMutation` function                             |
| `solid_void_query_no_input_overload`    | Void-input queries have overload without `input` parameter            |
| `solid_non_void_query_input_getter`     | Non-void queries take `input: () => QueryInput<K>`                    |
| `solid_queries_only_no_mutation`        | Queries-only manifest omits `createMutation`                          |
| `solid_mutations_only_no_query`         | Mutations-only manifest omits `createQuery`                           |
| `solid_empty_manifest_not_generated`    | Empty manifest produces empty string                                  |
| `solid_uses_create_signal`              | Output contains `createSignal(` calls                                 |
| `solid_uses_create_effect`              | Output contains `createEffect(` call                                  |
| `solid_uses_on_cleanup`                | Output contains `onCleanup(` call                                     |
| `solid_accessor_return_type`            | `QueryResult` uses `() => T` accessor pattern, not plain `T`         |
| `solid_enabled_accepts_getter`          | `enabled` option typed as `boolean \| (() => boolean)`                |
| `solid_refetch_interval_cleanup`        | `onCleanup` clears interval inside `createEffect`                     |
| `solid_mutation_has_reset`              | `MutationResult` includes `reset` method                              |
| `solid_mutation_has_mutate_async`       | `MutationResult` includes `mutateAsync` method                        |
| `solid_custom_import_paths`             | Custom import paths are used in generated imports                     |
| `snapshot_solid_full`                   | Insta snapshot with mixed queries and mutations                       |
| `snapshot_solid_queries_only`           | Insta snapshot with queries only                                      |
| `snapshot_solid_mutations_only`         | Insta snapshot with mutations only                                    |

### Config Tests

| Test                                | Description                                  |
|-------------------------------------|----------------------------------------------|
| `config_solid_output_default_none`  | Default config has `solid: None`             |
| `config_solid_output_parsed`        | Parses `output.solid` from TOML              |
| `cli_solid_output_override`         | CLI `--solid-output` overrides config        |

## 10. Backward Compatibility

- **No breaking changes.** The Solid file is entirely opt-in via a new config field.
- Existing projects without `output.solid` configured see no change.
- The generated `rpc-client.ts` and `rpc-types.ts` are unchanged.
- The existing Svelte, React, and Vue wrappers are unaffected.

## 11. Interactions with Existing Features

### 11.1 `createResource` Comparison

Solid has a built-in `createResource` primitive for async data fetching. Our `createQuery` differs in:

| Feature              | `createResource`           | `createQuery`                        |
|----------------------|----------------------------|--------------------------------------|
| Polling              | Not built-in               | `refetchInterval` option             |
| Lifecycle callbacks  | Not built-in               | `onSuccess`, `onError`, `onSettled`  |
| `enabled` flag       | Not built-in               | Conditional execution                |
| Type integration     | Generic                    | Typed to `Procedures` map            |
| `callOptions`        | N/A                        | Forwarded to `RpcClient`             |

`createResource` is a general-purpose primitive; `createQuery` is purpose-built for the RPC client with zero-config type safety.

## 12. Future Extensions

- **`createInfiniteQuery`** — cursor/offset-based pagination with accumulated pages signal.
- **Suspense integration** — Solid supports Suspense natively via `createResource`. A future version could integrate with Solid's Suspense boundary by throwing a promise.
- **SSR streaming** — Solid's streaming SSR could hydrate query results from the server.
