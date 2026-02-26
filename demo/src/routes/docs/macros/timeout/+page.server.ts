import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' }> = {
	basic: {
		lang: 'rust',
		code: `// 30 second server-side timeout
#[rpc_query(timeout = "30s")]
async fn slow_query() -> Report { ... }

// 5 second timeout on a mutation
#[rpc_mutation(timeout = "5s")]
async fn process_payment(input: PaymentInput) -> Receipt { ... }`
	},
	durations: {
		lang: 'rust',
		code: `#[rpc_query(timeout = "500ms")]  // 500 milliseconds
#[rpc_query(timeout = "5s")]     // 5 seconds
#[rpc_query(timeout = "1m")]     // 1 minute`
	},
	behavior: {
		lang: 'rust',
		code: `// If the handler exceeds the timeout:
// 1. The future is cancelled via tokio::time::timeout
// 2. The server returns HTTP 504 Gateway Timeout
// 3. The TypeScript client receives an RpcError with status 504

#[rpc_query(timeout = "5s")]
async fn get_report() -> Report {
    // If this takes > 5s, the client gets:
    // RpcError { status: 504, message: "Gateway Timeout" }
    expensive_computation().await
}`
	},
	combined: {
		lang: 'rust',
		code: `// Combine with other attributes
#[rpc_query(cache = "1h", timeout = "5s", init = "setup")]
async fn fast_cached(db: &PgPool) -> Stats { ... }

#[rpc_mutation(timeout = "10s", idempotent)]
async fn upsert_record(input: RecordInput) -> Record { ... }`
	}
};

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(entries.map(([, { code, lang }]) => highlightCode(code, lang)));
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
