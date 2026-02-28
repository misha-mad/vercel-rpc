# RFC-009: Vue 3 Reactive Wrappers

- **Status:** Implemented
- **Topic:** Generated Vue 3 composables for queries and mutations
- **Date:** February 2026

## 1. Summary

Generate an optional Vue 3 Composition API file (`rpc.vue.ts`) that provides `useQuery` and `useMutation` composables. These wrap the plain-promise `RpcClient` with reactive state (`ref`, `watch`), giving components a TanStack Query-style API with zero external dependencies.

## 2. Motivation

The generated `RpcClient` returns raw promises — great for libraries and SSR, but inconvenient in reactive Vue components:

```vue
<script setup lang="ts">
import { ref, onMounted } from "vue";
import { rpc } from "./rpc";
import type { UserProfile, RpcError } from "./rpc-types";

// Manual state management for every query
const data = ref<UserProfile | undefined>();
const error = ref<RpcError | undefined>();
const isLoading = ref(true);

onMounted(async () => {
  try {
    data.value = await rpc.query("current_user");
  } catch (e) {
    error.value = e as RpcError;
  } finally {
    isLoading.value = false;
  }
});
</script>

<template>
  <Spinner v-if="isLoading" />
  <ErrorBanner v-else-if="error" :error="error" />
  <p v-else>Hello, {{ data.name }}</p>
</template>
```

This boilerplate is repeated in every component. A reactive composable eliminates it:

```vue
<script setup lang="ts">
import { rpc } from "./rpc";
import { useQuery } from "./rpc.vue";

const user = useQuery(rpc, "current_user");
</script>

<template>
  <Spinner v-if="user.isLoading" />
  <ErrorBanner v-else-if="user.isError" :error="user.error" />
  <p v-else>Hello, {{ user.data.name }}</p>
</template>
```

| Without composables                   | With composables                    |
|---------------------------------------|-------------------------------------|
| ~15 lines of manual state + lifecycle | 1 line                              |
| Error/loading state easy to forget    | Always tracked                      |
| No auto-refetch, no reactive input    | Built-in                            |
| No lifecycle callbacks                | `onSuccess`, `onError`, `onSettled` |

## 3. Design Principles

1. **Vue 3 Composition API** — uses `ref`, `watch`, `computed`, `onUnmounted`. No Options API compatibility.
2. **Zero dependencies** — everything is generated; no npm packages to install. `vue` is a peer dependency only.
3. **Same types, same client** — composables import from the generated `rpc-client.ts` and `rpc-types.ts`. No parallel type system.
4. **Opt-in** — generated only when `output.vue` is configured. Projects that don't need reactive composables pay nothing.
5. **Getter-based input** — input is passed as a getter `() => value` for automatic reactive dependency tracking via `watch`. This is idiomatic Vue — composables commonly accept `MaybeRefOrGetter` parameters.
6. **Ref-based output** — results are returned as an object with `Ref` properties, usable directly in templates.

## 4. Configuration

### 4.1 Config File

New optional `vue` field in `[output]`:

```toml
[output]
types  = "src/rpc-types.ts"
client = "src/rpc-client.ts"
vue    = "src/rpc.vue.ts"     # NEW — opt-in
```

When `vue` is omitted (default), no composable file is generated.

### 4.2 CLI Flag

```
metaxy generate --vue-output src/rpc.vue.ts
```

### 4.3 Import Path

The generated composable file imports from the client file. The import specifier is derived from the relative path between `output.vue` and `output.client`, using `output.imports.extension` for the suffix.

### 4.4 Config Struct Changes

```rust
pub struct OutputConfig {
    pub types: PathBuf,
    pub client: PathBuf,
    pub svelte: Option<PathBuf>,
    pub react: Option<PathBuf>,
    pub vue: Option<PathBuf>,   // NEW
    pub imports: ImportsConfig,
}
```

```rust
pub struct CliOverrides {
    // ... existing fields ...
    pub vue_output: Option<PathBuf>,  // NEW
}
```

## 5. API Surface

### 5.1 `useQuery`

For queries with input (non-void):

```typescript
function useQuery<K extends QueryKey>(
  client: RpcClient,
  key: K,
  input: () => QueryInput<K>,
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
  readonly data: Ref<QueryOutput<K> | undefined>;

  /** The error from the most recent failed fetch, cleared on success. */
  readonly error: Ref<RpcError | undefined>;

  /** True while a fetch is in-flight (including the initial fetch). */
  readonly isLoading: Ref<boolean>;

  /** True after the first successful fetch. Stays true across refetches. */
  readonly isSuccess: Ref<boolean>;

  /** True when the most recent fetch failed. */
  readonly isError: Ref<boolean>;

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

```typescript
interface MutationResult<K extends MutationKey> {
  /** Execute the mutation. Rejects on error. */
  mutate: (...args: MutationArgs<K>) => Promise<void>;

  /** Execute the mutation and return the result. Rejects on error. */
  mutateAsync: (...args: MutationArgs<K>) => Promise<MutationOutput<K>>;

  /** The latest successfully resolved data. */
  readonly data: Ref<MutationOutput<K> | undefined>;

  /** The error from the most recent failed mutation, cleared on next attempt. */
  readonly error: Ref<RpcError | undefined>;

  /** True while a mutation is in-flight. */
  readonly isLoading: Ref<boolean>;

  /** True after the most recent mutation succeeded. */
  readonly isSuccess: Ref<boolean>;

  /** True when the most recent mutation failed. */
  readonly isError: Ref<boolean>;

  /** Reset state back to idle (clear data, error, status). */
  reset: () => void;
}
```

## 6. Usage Examples

### 6.1 Basic Query

```vue
<script setup lang="ts">
import { rpc } from "./rpc";
import { useQuery } from "./rpc.vue";

const health = useQuery(rpc, "health_check");
</script>

<template>
  <p>Status: {{ health.data?.status ?? "loading..." }}</p>
</template>
```

### 6.2 Query with Reactive Input

```vue
<script setup lang="ts">
import { ref } from "vue";
import { useQuery } from "./rpc.vue";

const userId = ref(1);

// Automatically refetches when userId changes
const user = useQuery(rpc, "get_user", () => ({ id: userId.value }));
</script>

<template>
  <input v-model.number="userId" type="number" />
  <p v-if="user.isLoading">Loading...</p>
  <p v-else>{{ user.data?.name }}</p>
</template>
```

### 6.3 Conditional Query

```vue
<script setup lang="ts">
const props = defineProps<{ token: string | null }>();

const profile = useQuery(rpc, "get_profile", {
  enabled: () => props.token !== null,
});
</script>
```

### 6.4 Polling

```vue
<script setup lang="ts">
const stats = useQuery(rpc, "server_stats", {
  refetchInterval: 5000,
});
</script>

<template>
  <pre>{{ JSON.stringify(stats.data, null, 2) }}</pre>
</template>
```

### 6.5 Mutation

```vue
<script setup lang="ts">
import { useMutation } from "./rpc.vue";

const updateName = useMutation(rpc, "update_profile", {
  onSuccess: () => alert("Saved!"),
});
</script>

<template>
  <button
    @click="updateName.mutate({ name: 'Alice' })"
    :disabled="updateName.isLoading"
  >
    {{ updateName.isLoading ? "Saving..." : "Save" }}
  </button>
</template>
```

### 6.6 Mutation with Async Result

```vue
<script setup lang="ts">
import { useRouter } from "vue-router";

const router = useRouter();
const order = useMutation(rpc, "create_order");

async function handleSubmit() {
  try {
    const result = await order.mutateAsync({ items: cart });
    router.push(`/orders/${result.id}`);
  } catch {
    // error is also available as order.error.value
  }
}
</script>

<template>
  <button @click="handleSubmit">Place Order</button>
</template>
```

### 6.7 Dependent Queries

```vue
<script setup lang="ts">
const user = useQuery(rpc, "current_user");

// Only fires after user.data is available
const posts = useQuery(rpc, "user_posts", () => ({ userId: user.data.value?.id ?? 0 }), {
  enabled: () => user.isSuccess.value,
});
</script>

<template>
  <Spinner v-if="user.isLoading" />
  <PostList v-else :posts="posts.data ?? []" />
</template>
```

## 7. Vue Implementation

### 7.1 File Extension

The generated file uses `.ts` extension. Vue composables work in any `.ts` or `.vue` file.

### 7.2 `useQuery` Implementation Sketch

```typescript
// rpc.vue.ts (generated)

import { ref, watch, onUnmounted, type Ref } from "vue";
import { type RpcClient, RpcError, type CallOptions } from "./rpc-client";
import type { Procedures } from "./rpc-types";

// ... type helpers (QueryKey, MutationKey, etc.) ...

export function useQuery<K extends QueryKey>(
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

  const data = ref<QueryOutput<K> | undefined>(options?.placeholderData) as Ref<QueryOutput<K> | undefined>;
  const error = ref<RpcError | undefined>();
  const hasFetched = ref(false);
  const isLoading = ref(false);

  async function fetchData(input?: QueryInput<K>) {
    isLoading.value = true;
    error.value = undefined;
    try {
      const callArgs: unknown[] = [key];
      if (input !== undefined) callArgs.push(input);
      if (options?.callOptions) callArgs.push(options.callOptions);
      data.value = await (client.query as (...a: unknown[]) => Promise<unknown>)(
        ...callArgs
      ) as QueryOutput<K>;
      hasFetched.value = true;
      options?.onSuccess?.(data.value!);
    } catch (e) {
      error.value = e as RpcError;
      options?.onError?.(error.value);
    } finally {
      isLoading.value = false;
      options?.onSettled?.();
    }
  }

  let intervalId: ReturnType<typeof setInterval> | undefined;

  const stopWatch = watch(
    () => {
      const enabled = typeof options?.enabled === "function"
        ? options.enabled()
        : (options?.enabled ?? true);
      const input = inputFn?.();
      return { enabled, input };
    },
    ({ enabled, input }) => {
      if (intervalId) { clearInterval(intervalId); intervalId = undefined; }
      if (!enabled) return;

      void fetchData(input);

      if (options?.refetchInterval) {
        intervalId = setInterval(() => fetchData(inputFn?.()), options.refetchInterval);
      }
    },
    { immediate: true, deep: true },
  );

  onUnmounted(() => {
    stopWatch();
    if (intervalId) clearInterval(intervalId);
  });

  return {
    data,
    error,
    isLoading,
    get isSuccess() { return hasFetched.value && error.value === undefined; },
    get isError() { return error.value !== undefined; },
    refetch: () => fetchData(inputFn?.()),
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
  const data = ref<MutationOutput<K> | undefined>() as Ref<MutationOutput<K> | undefined>;
  const error = ref<RpcError | undefined>();
  const isLoading = ref(false);
  const hasSucceeded = ref(false);

  async function execute(...input: MutationArgs<K>): Promise<MutationOutput<K>> {
    isLoading.value = true;
    error.value = undefined;
    hasSucceeded.value = false;
    try {
      const callArgs: unknown[] = [key];
      if (input.length > 0) callArgs.push(input[0]);
      if (options?.callOptions) callArgs.push(options.callOptions);
      const result = await (client.mutate as (...a: unknown[]) => Promise<unknown>)(
        ...callArgs
      ) as MutationOutput<K>;
      data.value = result;
      hasSucceeded.value = true;
      options?.onSuccess?.(result);
      return result;
    } catch (e) {
      error.value = e as RpcError;
      options?.onError?.(error.value);
      throw e;
    } finally {
      isLoading.value = false;
      options?.onSettled?.();
    }
  }

  function reset() {
    data.value = undefined;
    error.value = undefined;
    isLoading.value = false;
    hasSucceeded.value = false;
  }

  return {
    mutate: async (...args: MutationArgs<K>) => { await execute(...args); },
    mutateAsync: (...args: MutationArgs<K>) => execute(...args),
    data,
    error,
    isLoading,
    get isSuccess() { return hasSucceeded.value && error.value === undefined; },
    get isError() { return error.value !== undefined; },
    reset,
  };
}
```

### 7.4 Reactivity Model

| Feature                      | Mechanism                                                              |
|------------------------------|------------------------------------------------------------------------|
| `data`, `error`, `isLoading` | `ref()` — triggers template re-render on `.value` change               |
| `isSuccess`, `isError`       | Getters derived from refs                                              |
| Auto-refetch on input change | `watch()` with getter that reads `inputFn()` — Vue tracks dependencies |
| `enabled` flag               | Read inside `watch` getter — reactive when passed as a getter          |
| Polling cleanup              | `clearInterval` in watch callback + `onUnmounted`                      |
| Component unmount cleanup    | `onUnmounted()` stops watcher and clears interval                      |

### 7.5 Vue vs React vs Svelte Comparison

| Aspect               | Vue 3                      | React               | Svelte 5                   |
|----------------------|----------------------------|---------------------|----------------------------|
| State primitive      | `ref()`                    | `useState()`        | `$state()`                 |
| Side effects         | `watch()`                  | `useEffect()`       | `$effect()`                |
| Input parameter      | `() => value` (getter)     | `value` (plain)     | `() => value` (getter)     |
| `enabled`            | `boolean \| () => boolean` | `boolean`           | `boolean \| () => boolean` |
| Cleanup              | `onUnmounted()`            | `useEffect` return  | `$effect` return           |
| Dependency tracking  | Automatic (proxy-based)    | Manual (deps array) | Automatic (compiler)       |
| File extension       | `.ts`                      | `.ts`               | `.svelte.ts`               |
| Hook/composable name | `useQuery`                 | `useQuery`          | `createQuery`              |

## 8. Codegen Changes

### 8.1 New Module

A new codegen module `codegen/vue.rs` generates the composable file:

```rust
pub fn generate_vue_file(
    manifest: &Manifest,
    client_import_path: &str,
    types_import_path: &str,
    preserve_docs: bool,
) -> String;
```

### 8.2 Generated File Structure

```
// Header
// Import { ref, watch, onUnmounted, type Ref } from "vue"
// Import { type RpcClient, RpcError, type CallOptions } from "./rpc-client"
// Import { type Procedures, ...types } from "./rpc-types"
// Re-export types
//
// Type helpers (QueryKey, MutationKey, etc.)
// VoidQueryKey / VoidMutationKey union types (from manifest)
// QueryOptions, QueryResult interfaces
// MutationOptions, MutationResult interfaces
//
// useQuery() composable with overloads
// useMutation() composable
```

### 8.3 Conditional Emission

| Manifest state          | What's emitted                                    |
|-------------------------|---------------------------------------------------|
| Has queries + mutations | Full file with both `useQuery` and `useMutation`  |
| Queries only            | `useQuery` only, no mutation types                |
| Mutations only          | `useMutation` only, no query types                |
| Empty manifest          | Not generated (skip file entirely)                |

## 9. Test Plan

### Unit Tests

| Test                               | Description                                                         |
|------------------------------------|---------------------------------------------------------------------|
| `vue_imports_client_and_types`     | Output imports `RpcClient`, `RpcError`, `CallOptions`, `Procedures` |
| `vue_imports_vue`                  | Output imports `ref`, `watch`, `onUnmounted` from `"vue"`           |
| `vue_contains_use_query`           | Output contains `useQuery` function                                 |
| `vue_contains_use_mutation`        | Output contains `useMutation` function                              |
| `vue_void_query_no_input_overload` | Void-input queries have overload without `input` parameter          |
| `vue_non_void_query_input_getter`  | Non-void queries take `input: () => QueryInput<K>`                  |
| `vue_queries_only_no_mutation`     | Queries-only manifest omits `useMutation`                           |
| `vue_mutations_only_no_query`      | Mutations-only manifest omits `useQuery`                            |
| `vue_empty_manifest_not_generated` | Empty manifest produces empty string                                |
| `vue_uses_ref`                     | Output contains `ref(` calls                                        |
| `vue_uses_watch`                   | Output contains `watch(` call                                       |
| `vue_uses_on_unmounted`            | Output contains `onUnmounted` call                                  |
| `vue_enabled_accepts_getter`       | `enabled` option typed as `boolean \| (() => boolean)`              |
| `vue_refetch_interval_cleanup`     | Watch callback clears interval on re-run                            |
| `vue_mutation_has_reset`           | `MutationResult` includes `reset` method                            |
| `vue_mutation_has_mutate_async`    | `MutationResult` includes `mutateAsync` method                      |
| `vue_custom_import_paths`          | Custom import paths are used in generated imports                   |
| `snapshot_vue_full`                | Insta snapshot with mixed queries and mutations                     |
| `snapshot_vue_queries_only`        | Insta snapshot with queries only                                    |
| `snapshot_vue_mutations_only`      | Insta snapshot with mutations only                                  |

### Config Tests

| Test                              | Description                                |
|-----------------------------------|--------------------------------------------|
| `config_vue_output_default_none`  | Default config has `vue: None`             |
| `config_vue_output_parsed`        | Parses `output.vue` from TOML              |
| `cli_vue_output_override`         | CLI `--vue-output` overrides config        |

## 10. Backward Compatibility

- **No breaking changes.** The Vue composable file is entirely opt-in via a new config field.
- Existing projects without `output.vue` configured see no change.
- The generated `rpc-client.ts` and `rpc-types.ts` are unchanged.
- The existing Svelte and React wrappers are unaffected.

## 11. Implementation Notes

The following deviations from the original draft were made during implementation based on code review:

### 11.1 `computed()` for `isSuccess` / `isError`

The draft specified `isSuccess` and `isError` as plain object getters (`get isSuccess() { ... }`). In the implementation, these are `computed()` refs returning `ComputedRef<boolean>`. Plain getters lose reactivity in Vue templates — only `Ref` and `ComputedRef` values trigger re-renders when used in `<template>`.

### 11.2 `onScopeDispose` instead of `onUnmounted`

The draft used `onUnmounted()` for cleanup. The implementation uses `onScopeDispose()` instead, which works with any Vue effect scope (not just component lifecycles). This is more flexible — composables can be used in `effectScope()` contexts outside of components.

### 11.3 `JSON.stringify` instead of `deep: true`

The draft used `watch(..., { immediate: true, deep: true })` for reactive input tracking. The implementation replaces `deep: true` with a `serialized: JSON.stringify(input)` field in the watch source. This avoids expensive deep object comparison and uses string equality instead.

### 11.4 Typed watch callback

The watch callback destructures `{ enabled, input }` with an explicit type annotation `{ enabled: boolean; input: QueryInput<K> | undefined; serialized: string }` to avoid TypeScript implicit-any errors (TS7031).

## 12. Future Extensions

- **`useInfiniteQuery`** — cursor/offset-based pagination with accumulated `pages` ref.
- **Suspense integration** — Vue 3 `<Suspense>` support via async setup.
- **SSR hydration** — initial data passed from server to avoid client-side refetch.
