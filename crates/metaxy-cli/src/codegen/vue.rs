use crate::model::Manifest;

use super::common::{self, FrameworkConfig};

const QUERY_OPTIONS_INTERFACE: &str = r#"export interface QueryOptions<K extends QueryKey> {
  /**
   * Whether to execute the query. @default true
   *
   * Pass a getter `() => bool` for reactive updates — a plain `boolean` is
   * read once when `useQuery` is called and will not trigger re-fetches.
   */
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
}"#;

const QUERY_RESULT_INTERFACE: &str = r#"export interface QueryResult<K extends QueryKey> {
  /** The latest successfully resolved data, or placeholderData. */
  readonly data: Ref<QueryOutput<K> | undefined>;

  /** The error from the most recent failed fetch, cleared on next attempt. */
  readonly error: Ref<RpcError | undefined>;

  /** True while a fetch is in-flight (including the initial fetch). */
  readonly isLoading: Ref<boolean>;

  /** True after the first successful fetch. Stays true even if a later refetch fails. */
  readonly isSuccess: ComputedRef<boolean>;

  /** True when the most recent fetch failed. */
  readonly isError: ComputedRef<boolean>;

  /** Manually trigger a refetch. No-op when `enabled` is false. Resets the polling interval. */
  refetch: () => Promise<void>;
}"#;

const MUTATION_OPTIONS_INTERFACE: &str = r#"export interface MutationOptions<K extends MutationKey> {
  /** Per-call options forwarded to client.mutate(). */
  callOptions?: CallOptions;

  /** Called when the mutation succeeds. */
  onSuccess?: (data: MutationOutput<K>) => void;

  /** Called when the mutation fails. */
  onError?: (error: RpcError) => void;

  /** Called when the mutation settles (success or failure). */
  onSettled?: () => void;
}"#;

const MUTATION_RESULT_INTERFACE: &str = r#"export interface MutationResult<K extends MutationKey> {
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
  readonly isSuccess: ComputedRef<boolean>;

  /** True when the most recent mutation failed. */
  readonly isError: ComputedRef<boolean>;

  /** Reset state back to idle (clear data, error, status). */
  reset: () => void;
}"#;

const USE_QUERY_IMPL: &str = r#"export function useQuery<K extends QueryKey>(
  client: RpcClient,
  ...args: unknown[]
): QueryResult<K> {
  const key = args[0] as K;

  let inputFn: (() => QueryInput<K>) | undefined;
  let optionsArg: QueryOptions<K> | (() => QueryOptions<K>) | undefined;

  if (typeof args[1] === "function" && args[2] !== undefined) {
    inputFn = args[1] as () => QueryInput<K>;
    optionsArg = args[2] as QueryOptions<K> | (() => QueryOptions<K>) | undefined;
  } else if (typeof args[1] === "function") {
    if (VOID_QUERY_KEYS.has(key)) {
      optionsArg = args[1] as () => QueryOptions<K>;
    } else {
      inputFn = args[1] as () => QueryInput<K>;
    }
  } else if (typeof args[1] === "object") {
    optionsArg = args[1] as QueryOptions<K>;
  }

  function resolveOptions(): QueryOptions<K> | undefined {
    return typeof optionsArg === "function" ? optionsArg() : optionsArg;
  }

  function resolveEnabled(): boolean {
    const opts = resolveOptions();
    return typeof opts?.enabled === "function"
      ? opts.enabled()
      : (opts?.enabled ?? true);
  }

  const data = ref<QueryOutput<K> | undefined>(resolveOptions()?.placeholderData) as Ref<QueryOutput<K> | undefined>;
  const error = ref<RpcError | undefined>();
  const hasFetched = ref(false);
  const isLoading = ref(false);
  const isSuccess = computed(() => hasFetched.value);
  const isError = computed(() => error.value !== undefined);

  let generation = 0;
  let controller: AbortController | undefined;
  let intervalId: ReturnType<typeof setInterval> | undefined;

  async function fetchData(input: QueryInput<K> | undefined, signal: AbortSignal, gen: number) {
    const opts = resolveOptions();
    isLoading.value = true;
    error.value = undefined;
    try {
      const callArgs: unknown[] = [key];
      if (input !== undefined) callArgs.push(input);
      const mergedCallOptions = { ...opts?.callOptions, signal: opts?.callOptions?.signal
          ? AbortSignal.any([signal, opts.callOptions.signal])
          : signal };
      callArgs.push(mergedCallOptions);
      const result = await (client.query as (...a: unknown[]) => Promise<unknown>)(
        ...callArgs
      ) as QueryOutput<K>;
      if (gen !== generation) return;
      data.value = result;
      hasFetched.value = true;
      opts?.onSuccess?.(data.value!);
    } catch (e) {
      if (gen !== generation) return;
      error.value = e as RpcError;
      opts?.onError?.(error.value);
    } finally {
      if (gen === generation) {
        isLoading.value = false;
        opts?.onSettled?.();
      }
    }
  }

  function setupInterval(enabled: boolean, refetchInterval: number | undefined) {
    if (intervalId) { clearInterval(intervalId); intervalId = undefined; }
    if (enabled && refetchInterval) {
      intervalId = setInterval(() => {
        if (controller && !controller.signal.aborted) {
          void fetchData(inputFn?.(), controller.signal, generation);
        }
      }, refetchInterval);
    }
  }

  const stopWatch = watch(
    () => {
      const enabled = resolveEnabled();
      const input = inputFn?.();
      return { enabled, input, serialized: JSON.stringify(input), refetchInterval: resolveOptions()?.refetchInterval };
    },
    (curr, prev) => {
      const inputChanged = !prev || curr.enabled !== prev.enabled || curr.serialized !== prev.serialized;

      if (inputChanged) {
        if (controller) { controller.abort(); controller = undefined; }
        if (curr.enabled) {
          generation++;
          const gen = generation;
          controller = new AbortController();
          void fetchData(curr.input, controller.signal, gen);
        } else {
          isLoading.value = false;
        }
      }

      setupInterval(curr.enabled, curr.refetchInterval);
    },
    { immediate: true },
  );

  onScopeDispose(() => {
    stopWatch();
    generation++;
    if (controller) controller.abort();
    if (intervalId) clearInterval(intervalId);
  });

  return {
    data,
    error,
    isLoading,
    isSuccess,
    isError,
    refetch: async () => {
      const enabled = resolveEnabled();
      if (!enabled) return;
      generation++;
      const gen = generation;
      const localController = new AbortController();
      if (controller) controller.abort();
      controller = localController;
      setupInterval(enabled, resolveOptions()?.refetchInterval);
      await fetchData(inputFn?.(), localController.signal, gen);
    },
  };
}"#;

const USE_MUTATION_IMPL: &str = r#"export function useMutation<K extends MutationKey>(
  client: RpcClient,
  key: K,
  options?: MutationOptions<K>,
): MutationResult<K> {
  const data = ref<MutationOutput<K> | undefined>() as Ref<MutationOutput<K> | undefined>;
  const error = ref<RpcError | undefined>();
  const isLoading = ref(false);
  const hasSucceeded = ref(false);
  const isSuccess = computed(() => hasSucceeded.value);
  const isError = computed(() => error.value !== undefined);

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

  return {
    mutate: async (...args: MutationArgs<K>) => { await execute(...args); },
    mutateAsync: (...args: MutationArgs<K>) => execute(...args),
    data,
    error,
    isLoading,
    isSuccess,
    isError,
    reset: () => { data.value = undefined; error.value = undefined; isLoading.value = false; hasSucceeded.value = false; },
  };
}"#;

const FRAMEWORK_IMPORT: &str =
    "import { ref, computed, watch, onScopeDispose, type Ref, type ComputedRef } from \"vue\";";

/// Generates the complete Vue 3 Composition API wrapper file content from a manifest.
///
/// Returns an empty string when the manifest contains no procedures (the caller
/// should skip writing the file in that case).
pub fn generate_vue_file(
    manifest: &Manifest,
    client_import_path: &str,
    types_import_path: &str,
    preserve_docs: bool,
) -> String {
    common::generate_framework_file(
        manifest,
        client_import_path,
        types_import_path,
        preserve_docs,
        &FrameworkConfig {
            framework_import: Some(FRAMEWORK_IMPORT),
            query_fn_name: "useQuery",
            input_as_getter: true,
            query_interfaces: &[QUERY_OPTIONS_INTERFACE, QUERY_RESULT_INTERFACE],
            mutation_interfaces: &[MUTATION_OPTIONS_INTERFACE, MUTATION_RESULT_INTERFACE],
            query_impl: USE_QUERY_IMPL,
            mutation_impl: USE_MUTATION_IMPL,
        },
    )
}
