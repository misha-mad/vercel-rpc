import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	retryConfig: {
		lang: 'typescript',
		code: `interface RetryPolicy {
  attempts: number;                              // max retries (excluding initial request)
  delay: number | ((attempt: number) => number); // fixed ms or backoff function
  retryOn?: number[];                            // HTTP status codes (default: [408, 429, 500, 502, 503, 504])
}`
	},
	retryBasic: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: "/api",
  retry: { attempts: 3, delay: 1000 },
});`
	},
	retryExponential: {
		lang: 'typescript',
		code: `// Exponential backoff: 1s, 2s, 4s
const rpc = createRpcClient({
  baseUrl: "/api",
  retry: { attempts: 3, delay: (n) => 1000 * 2 ** (n - 1) },
});`
	},
	retryCustom: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: "/api",
  retry: {
    attempts: 3,
    delay: 1000,
    retryOn: [429, 503],  // only retry rate-limited or unavailable
  },
});`
	},
	retryIdempotent: {
		lang: 'rust',
		code: `// By default mutations are never retried.
// Mark a mutation as idempotent to opt in to retry:
#[rpc_mutation(idempotent)]
async fn upsert_user(input: UserInput) -> User {
    // safe to retry — repeated calls produce the same result
}`
	},
	retryDemoRust: {
		lang: 'rust',
		code: `static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

/// Returns an error for the first \`fail_count\` calls, then 200.
/// Each Vercel cold start resets the counter automatically.
#[rpc_query]
async fn retry_demo(input: RetryDemoInput) -> Result<RetryDemoResponse, String> {
    let call_number = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    if call_number <= input.fail_count {
        return Err(format!("Simulated failure (call {})", call_number));
    }
    Ok(RetryDemoResponse {
        call_number,
        total_calls: CALL_COUNT.load(Ordering::Relaxed),
        message: format!("Success on call {}", call_number),
    })
}`
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
