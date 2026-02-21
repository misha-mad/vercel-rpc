# RFC-7: Framework Reactive Wrappers

- **Status:** Draft
- **Topic:** Generated Svelte 5 reactive wrappers for queries and mutations
- **Date:** February 2026

## 1. Summary

Generate an optional Svelte 5 rune-based wrapper file (`rpc.svelte.ts`) that provides `createQuery` and `createMutation` helpers. These wrap the plain-promise `RpcClient` with reactive state (`$state`, `$derived`, `$effect`), giving components a TanStack Query–style API with zero external dependencies.

## 2. Motivation

The generated `RpcClient` returns raw promises — great for libraries and SSR, but inconvenient in reactive UI components:

```svelte
<script lang="ts">
  import { rpc } from '$lib/rpc';

  // Manual state management for every query
  let data = $state<UserProfile | undefined>();
  let error = $state<RpcError | undefined>();
  let isLoading = $state(true);

  $effect(() => {
    rpc.query("current_user")
      .then(d => { data = d; })
      .catch(e => { error = e; })
      .finally(() => { isLoading = false; });
  });
</script>

{#if isLoading}
  <Spinner />
{:else if error}
  <ErrorBanner {error} />
{:else}
  <p>Hello, {data.name}</p>
{/if}
```

This boilerplate is repeated in every component. A reactive wrapper eliminates it:

```svelte
<script lang="ts">
  import { rpc } from '$lib/rpc';
  import { createQuery } from '$lib/rpc.svelte';

  const user = createQuery(rpc, "current_user");
</script>

{#if user.isLoading}
  <Spinner />
{:else if user.isError}
  <ErrorBanner error={user.error} />
{:else}
  <p>Hello, {user.data.name}</p>
{/if}
```

| Without wrappers                             | With wrappers                       |
|----------------------------------------------|-------------------------------------|
| ~12 lines of manual state + effect per query | 1 line                              |
| Error/loading state easy to forget           | Always tracked                      |
| No auto-refetch, no reactive input           | Built-in                            |
| No lifecycle callbacks                       | `onSuccess`, `onError`, `onSettled` |

## 3. Design Principles

1. **Svelte 5 first** — uses runes (`$state`, `$derived`, `$effect`) natively. No Svelte 4 stores compatibility.
2. **Zero dependencies** — everything is generated; no npm packages to install or version-match.
3. **Same types, same client** — wrappers import from the generated `rpc-client.ts` and `rpc-types.ts`. No parallel type system.
4. **Opt-in** — generated only when `output.svelte` is configured. Projects that don't need reactive wrappers pay nothing.
5. **Getter-based reactivity** — non-void input is passed as `() => Input`, enabling Svelte's fine-grained dependency tracking via `$effect`.
6. **Composable** — wrappers return plain objects with reactive getters. They compose with Svelte's `$derived` naturally.

## 4. Configuration

### 4.1 Config File

New optional `svelte` field in `[output]`:

```toml
[output]
types = "src/lib/rpc-types.ts"
client = "src/lib/rpc-client.ts"
svelte = "src/lib/rpc.svelte.ts"     # NEW — opt-in
```

When `svelte` is omitted (default), no wrapper file is generated.

### 4.2 CLI Flag

```
rpc generate --svelte-output src/lib/rpc.svelte.ts
```

### 4.3 Import Path

The generated wrapper imports from the client file. The import specifier is derived from the relative path between `output.svelte` and `output.client`, using `output.imports.extension` for the suffix:

```typescript
// If svelte = "src/lib/rpc.svelte.ts" and client = "src/lib/rpc-client.ts"
import { type RpcClient, RpcError } from "./rpc-client";
```

### 4.4 Config Struct Changes

```rust
pub struct OutputConfig {
    pub types: PathBuf,
    pub client: PathBuf,
    pub svelte: Option<PathBuf>,  // NEW
    pub imports: ImportsConfig,
}
```

```rust
pub struct CliOverrides {
    // ... existing fields ...
    pub svelte_output: Option<PathBuf>,  // NEW
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
  /** Whether to execute the query. Reactive when passed as a getter. @default true */
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
  readonly data: QueryOutput<K> | undefined;

  /** The error from the most recent failed fetch, cleared on success. */
  readonly error: RpcError | undefined;

  /** True while a fetch is in-flight (including the initial fetch). */
  readonly isLoading: boolean;

  /** True after the first successful fetch. Stays true across refetches. */
  readonly isSuccess: boolean;

  /** True when error is set. */
  readonly isError: boolean;

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

For mutations with input (non-void):

```typescript
interface MutationResult<K extends MutationKey> {
  /** Execute the mutation with the given input. Rejects on error. */
  mutate: (input: MutationInput<K>) => Promise<void>;

  /** Execute the mutation and return the result. Rejects on error. */
  mutateAsync: (input: MutationInput<K>) => Promise<MutationOutput<K>>;

  /** The latest successfully resolved data. */
  readonly data: MutationOutput<K> | undefined;

  /** The error from the most recent failed mutation, cleared on next attempt. */
  readonly error: RpcError | undefined;

  /** True while a mutation is in-flight. */
  readonly isLoading: boolean;

  /** True after the most recent mutation succeeded. */
  readonly isSuccess: boolean;

  /** True when error is set. */
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

```svelte
<script lang="ts">
  import { rpc } from '$lib/rpc';
  import { createQuery } from '$lib/rpc.svelte';

  const health = createQuery(rpc, "health_check");
</script>

<p>Status: {health.data?.status ?? "loading..."}</p>
```

### 6.2 Query with Reactive Input

```svelte
<script lang="ts">
  import { createQuery } from '$lib/rpc.svelte';

  let userId = $state(1);

  // Automatically refetches when userId changes
  const user = createQuery(rpc, "get_user", () => ({ id: userId }));
</script>

<input type="number" bind:value={userId} />
<p>{user.isLoading ? "Loading..." : user.data?.name}</p>
```

### 6.3 Conditional Query

```svelte
<script lang="ts">
  let token = $state<string | null>(null);

  const profile = createQuery(rpc, "get_profile", {
    enabled: () => token !== null,
  });
</script>
```

### 6.4 Polling

```svelte
<script lang="ts">
  const stats = createQuery(rpc, "server_stats", {
    refetchInterval: 5000, // every 5 seconds
  });
</script>
```

### 6.5 Mutation

```svelte
<script lang="ts">
  import { createMutation } from '$lib/rpc.svelte';

  const updateName = createMutation(rpc, "update_profile", {
    onSuccess: () => alert("Saved!"),
  });
</script>

<button
  onclick={() => updateName.mutate({ name: "Alice" })}
  disabled={updateName.isLoading}
>
  {updateName.isLoading ? "Saving..." : "Save"}
</button>
```

### 6.6 Mutation with Async Result

```svelte
<script lang="ts">
  async function handleSubmit() {
    try {
      const result = await order.mutateAsync({ items: cart });
      goto(`/orders/${result.id}`);
    } catch (e) {
      // error is also available as order.error
    }
  }
</script>
```

### 6.7 Dependent Queries

```svelte
<script lang="ts">
  const user = createQuery(rpc, "current_user");

  // Only fires after user.data is available
  const posts = createQuery(rpc, "user_posts", () => ({ userId: user.data!.id }), {
    enabled: () => user.isSuccess,
  });
</script>
```

## 7. Svelte 5 Implementation

### 7.1 File Extension

The generated file **must** use the `.svelte.ts` extension for Svelte 5 rune syntax (`$state`, `$derived`, `$effect`) to work. Regular `.ts` files do not support runes.

### 7.2 `createQuery` Implementation Sketch

```typescript
// rpc.svelte.ts (generated)

import type { RpcClient, CallOptions, RpcError } from "./rpc-client";
import type { Procedures } from "./rpc-types";

// ... type helpers (QueryKey, MutationKey, etc.) ...

export function createQuery<K extends QueryKey>(
  client: RpcClient,
  key: K,
  inputOrOptions?: (() => QueryInput<K>) | QueryOptions<K>,
  maybeOptions?: QueryOptions<K>,
): QueryResult<K> {
  // Resolve overloaded arguments
  const inputFn = typeof inputOrOptions === "function"
    ? inputOrOptions as () => QueryInput<K>
    : undefined;
  const options = (typeof inputOrOptions === "object" ? inputOrOptions : maybeOptions) as
    | QueryOptions<K>
    | undefined;

  let data = $state<QueryOutput<K> | undefined>(options?.placeholderData);
  let error = $state<RpcError | undefined>();
  let isLoading = $state(true);

  async function fetch() {
    const enabled = typeof options?.enabled === "function"
      ? options.enabled()
      : (options?.enabled ?? true);
    if (!enabled) return;

    isLoading = true;
    error = undefined;
    try {
      const input = inputFn?.();
      data = await client.query(key, ...(input !== undefined ? [input] : []),
        ...(options?.callOptions ? [options.callOptions] : [])) as QueryOutput<K>;
      options?.onSuccess?.(data!);
    } catch (e) {
      error = e as RpcError;
      options?.onError?.(error);
    } finally {
      isLoading = false;
      options?.onSettled?.();
    }
  }

  $effect(() => {
    // Track reactive dependencies: inputFn() and enabled()
    const enabled = typeof options?.enabled === "function"
      ? options.enabled()
      : (options?.enabled ?? true);
    if (inputFn) inputFn(); // subscribe to input changes
    if (!enabled) return;

    fetch();

    if (options?.refetchInterval) {
      const interval = setInterval(fetch, options.refetchInterval);
      return () => clearInterval(interval);
    }
  });

  return {
    get data() { return data; },
    get error() { return error; },
    get isLoading() { return isLoading; },
    get isSuccess() { return data !== undefined; },
    get isError() { return error !== undefined; },
    refetch: fetch,
  };
}
```

### 7.3 `createMutation` Implementation Sketch

```typescript
export function createMutation<K extends MutationKey>(
  client: RpcClient,
  key: K,
  options?: MutationOptions<K>,
): MutationResult<K> {
  let data = $state<MutationOutput<K> | undefined>();
  let error = $state<RpcError | undefined>();
  let isLoading = $state(false);

  async function execute(input?: MutationInput<K>): Promise<MutationOutput<K>> {
    isLoading = true;
    error = undefined;
    try {
      const result = await client.mutate(key, ...(input !== undefined ? [input] : []),
        ...(options?.callOptions ? [options.callOptions] : [])) as MutationOutput<K>;
      data = result;
      options?.onSuccess?.(result);
      return result;
    } catch (e) {
      error = e as RpcError;
      options?.onError?.(error);
      throw e;
    } finally {
      isLoading = false;
      options?.onSettled?.();
    }
  }

  return {
    mutate: async (input?: MutationInput<K>) => { await execute(input); },
    mutateAsync: (input?: MutationInput<K>) => execute(input),
    get data() { return data; },
    get error() { return error; },
    get isLoading() { return isLoading; },
    get isSuccess() { return data !== undefined && error === undefined; },
    get isError() { return error !== undefined; },
    reset: () => { data = undefined; error = undefined; isLoading = false; },
  };
}
```

### 7.4 Reactivity Model

| Feature                      | Mechanism                                 |
|------------------------------|-------------------------------------------|
| `data`, `error`, `isLoading` | `$state` — triggers re-render on change   |
| `isSuccess`, `isError`       | Getter over `$state` — derived implicitly |
| Auto-refetch on input change | `$effect` tracks `inputFn()` call         |
| `enabled` as getter          | `$effect` tracks `options.enabled()` call |
| Polling cleanup              | `$effect` returns cleanup function        |

### 7.5 No `$effect.pre` or `$derived`

The implementation uses `$state` + getters rather than `$derived` for computed fields. This keeps the generated code simpler and avoids edge cases with `$derived` and mutable state. The getter pattern (`get isSuccess() { return data !== undefined; }`) is re-evaluated on each access, which is the correct behavior for these flags.

## 8. Codegen Changes

### 8.1 New Module

A new codegen module `codegen/svelte.rs` generates the wrapper file:

```rust
pub fn generate_svelte_file(
    manifest: &Manifest,
    client_import_path: &str,
    types_import_path: &str,
    preserve_docs: bool,
) -> String;
```

### 8.2 Generated File Structure

```
// Header
// Import { type RpcClient, RpcError, type CallOptions } from "./rpc-client"
// Import { type Procedures, ...types } from "./rpc-types"
// Re-export types
//
// Type helpers (QueryKey, MutationKey, etc.)
// VoidQueryKey / VoidMutationKey union types (from manifest)
// QueryOptions, QueryResult interfaces
// MutationOptions, MutationResult interfaces
//
// createQuery() function
// createMutation() function
```

### 8.3 Void vs Non-Void Handling

The codegen emits type unions from the manifest to distinguish void-input procedures:

```typescript
type VoidQueryKey = "health_check" | "server_stats";
type NonVoidQueryKey = Exclude<QueryKey, VoidQueryKey>;
```

The `createQuery` function uses TypeScript overloads so void-input queries don't require an `input` argument:

```typescript
// Void input — no input parameter
export function createQuery<K extends VoidQueryKey>(
  client: RpcClient, key: K, options?: QueryOptions<K>
): QueryResult<K>;

// Non-void input — requires input getter
export function createQuery<K extends NonVoidQueryKey>(
  client: RpcClient, key: K, input: () => QueryInput<K>, options?: QueryOptions<K>
): QueryResult<K>;
```

### 8.4 Conditional Emission

| Manifest state          | What's emitted                                         |
|-------------------------|--------------------------------------------------------|
| Has queries + mutations | Full file with both `createQuery` and `createMutation` |
| Queries only            | `createQuery` only, no mutation types                  |
| Mutations only          | `createMutation` only, no query types                  |
| Empty manifest          | Not generated (skip file entirely)                     |

### 8.5 `commands.rs` Changes

`cmd_generate` writes the Svelte file when `config.output.svelte` is `Some`:

```rust
if let Some(svelte_path) = &config.output.svelte {
    let svelte_content = codegen::svelte::generate_svelte_file(
        &manifest,
        &client_import_specifier,
        &types_import_specifier,
        config.codegen.preserve_docs,
    );
    write_file(svelte_path, &svelte_content)?;
}
```

## 9. Test Plan

### Unit Tests

| Test                                      | Description                                                             |
|-------------------------------------------|-------------------------------------------------------------------------|
| `svelte_file_imports_client_and_types`    | Output imports `RpcClient`, `RpcError`, `CallOptions`, and `Procedures` |
| `svelte_file_contains_create_query`       | Output contains `createQuery` function                                  |
| `svelte_file_contains_create_mutation`    | Output contains `createMutation` function                               |
| `svelte_void_query_no_input_overload`     | Void-input queries have overload without `input` parameter              |
| `svelte_non_void_query_input_overload`    | Non-void queries require `input: () => ...` parameter                   |
| `svelte_void_mutation_no_input`           | Void-input mutation `mutate()` takes no arguments                       |
| `svelte_queries_only_no_mutation`         | Queries-only manifest omits `createMutation`                            |
| `svelte_mutations_only_no_query`          | Mutations-only manifest omits `createQuery`                             |
| `svelte_empty_manifest_not_generated`     | Empty manifest produces empty string                                    |
| `svelte_uses_state_rune`                  | Output contains `$state` calls                                          |
| `svelte_uses_effect_rune`                 | Output contains `$effect` call                                          |
| `svelte_refetch_interval_cleanup`         | `$effect` returns cleanup for `clearInterval`                           |
| `svelte_query_result_has_refetch`         | Return object includes `refetch` method                                 |
| `svelte_mutation_result_has_reset`        | Return object includes `reset` method                                   |
| `svelte_mutation_result_has_mutate_async` | Return object includes `mutateAsync` method                             |
| `snapshot_svelte_full`                    | Insta snapshot with mixed queries and mutations                         |

### Config Tests

| Test                                            | Description                                           |
|-------------------------------------------------|-------------------------------------------------------|
| `config_svelte_output_default_none`             | Default config has `svelte: None`                     |
| `config_svelte_output_parsed`                   | Parses `output.svelte` from TOML                      |
| `cli_svelte_output_override`                    | CLI `--svelte-output` overrides config                |
| `cmd_generate_writes_svelte_file`               | `cmd_generate` writes the Svelte file when configured |
| `cmd_generate_skips_svelte_when_not_configured` | No file written when `output.svelte` is None          |

## 10. Backward Compatibility

- **No breaking changes.** The Svelte wrapper is entirely opt-in via a new config field.
- Existing projects without `output.svelte` configured see no change.
- The generated `rpc-client.ts` and `rpc-types.ts` are unchanged.

## 11. Interactions with Existing Features

### 11.1 Request Deduplication (RFC-6)

`createQuery` calls `client.query()` which already deduplicates. Multiple components calling `createQuery` with the same key and input share a single HTTP request automatically.

### 11.2 Per-Call Options (RFC-5)

`QueryOptions.callOptions` and `MutationOptions.callOptions` are forwarded directly to `client.query()` / `client.mutate()`. This allows per-wrapper `headers`, `timeout`, and `signal` overrides.

### 11.3 Abort / Cleanup

When a component unmounts, Svelte 5 runs `$effect` cleanup functions. The generated `$effect` cleans up polling intervals. For abort-on-unmount, callers can pass an `AbortSignal` via `callOptions.signal`:

```svelte
<script lang="ts">
  const controller = new AbortController();
  const user = createQuery(rpc, "current_user", {
    callOptions: { signal: controller.signal },
  });

  // Svelte 5: cleanup on unmount is handled by $effect return
</script>
```

### 11.4 JSDoc / Preserve Docs

When `codegen.preserve_docs = true`, doc comments from Rust source are forwarded to the type helpers and option interfaces in the Svelte file, matching the behavior in the client and types files.

## 12. Future Extensions

These are out of scope for the initial implementation but the design accommodates them:

- **Other frameworks** — React (`rpc-react.ts` with `useState`/`useEffect`), Vue (`rpc-vue.ts` with `ref`/`watchEffect`). Each would be a separate codegen module and config field (`output.react`, `output.vue`).
- **`createInfiniteQuery`** — for cursor/offset-based pagination. Extends the pattern with a `getNextPageParam` option and accumulated `pages` array.
- **Optimistic updates** — `MutationOptions.onMutate` callback that returns rollback data, with automatic rollback on error.
- **Query invalidation** — `queryClient.invalidate("key")` pattern for cache-busting after mutations. Would require a shared query registry.
