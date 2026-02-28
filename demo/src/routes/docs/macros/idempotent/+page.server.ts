import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	basic: {
		lang: 'rust',
		code: `// Safe to retry on failure — repeated calls produce the same result
#[rpc_mutation(idempotent)]
async fn upsert_user(input: UpsertInput) -> User { ... }

// NOT safe to retry (default) — each call creates a new resource
#[rpc_mutation]
async fn create_order(input: OrderInput) -> Order { ... }`
	},
	examples: {
		lang: 'rust',
		code: `// Good candidates for idempotent:
#[rpc_mutation(idempotent)]
async fn set_preference(input: PrefInput) -> Pref { ... }  // upsert

#[rpc_mutation(idempotent)]
async fn update_status(input: StatusInput) -> Status { ... }  // overwrite

#[rpc_mutation(idempotent)]
async fn delete_item(id: u64) -> bool { ... }  // delete is idempotent

// BAD — do NOT mark as idempotent:
// #[rpc_mutation(idempotent)]
// async fn transfer_money(input: TransferInput) -> Receipt { ... }`
	},
	retryBehavior: {
		lang: 'typescript',
		code: `// Without idempotent: mutation is NEVER retried, even with retry policy
// With idempotent: mutation follows the same retry policy as queries

const rpc = createRpcClient({
  baseUrl: '/api',
  retry: { attempts: 3, delay: 1000 },
});

// upsert_user (idempotent) — will retry up to 3 times on failure
await rpc.mutate('upsert_user', input);

// create_order (not idempotent) — will NOT retry, even though retry is configured
await rpc.mutate('create_order', input);`
	},
	combined: {
		lang: 'rust',
		code: `// Combine with other attributes
#[rpc_mutation(idempotent, timeout = "10s")]
async fn upsert_record(input: RecordInput) -> Record { ... }

#[rpc_mutation(idempotent, init = "setup_db")]
async fn sync_profile(pool: &PgPool, input: ProfileInput) -> Profile { ... }

// ❌ Compile error — idempotent is mutations only
// #[rpc_query(idempotent)]
// async fn get_data() -> Data { ... }`
	},
	idempotentDemoRust: {
		lang: 'rust',
		code: `static STORED_VALUE: AtomicU64 = AtomicU64::new(0);
static CALL_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Serialize)]
pub struct IdempotentDemoResponse {
    pub previous: u64,
    pub current: u64,
    pub total_calls: u64,
}

/// Repeated calls with the same input produce the same result.
#[rpc_mutation(idempotent)]
async fn idempotent_demo(input: IdempotentDemoInput) -> IdempotentDemoResponse {
    let previous = STORED_VALUE.swap(input.value, Ordering::Relaxed);
    let total_calls = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    IdempotentDemoResponse { previous, current: input.value, total_calls }
}`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
