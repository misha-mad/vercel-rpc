import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	dedupExample: {
		lang: 'typescript',
		code: `// Both calls result in a single HTTP request
const [a, b] = await Promise.all([
  rpc.query("get_user", { id: 1 }),
  rpc.query("get_user", { id: 1 }),
]);
// a === b (same reference)`
	},
	dedupDisableGlobal: {
		lang: 'typescript',
		code: `// Disable deduplication globally
const rpc = createRpcClient({
  baseUrl: "/api",
  dedupe: false,
});`
	},
	dedupDisablePerCall: {
		lang: 'typescript',
		code: `// Disable for a single call
const fresh = await rpc.query("get_user", id, { dedupe: false });`
	},
	dedupDemoRust: {
		lang: 'rust',
		code: `static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

/// Slow endpoint (500ms) with a request counter.
#[rpc_query]
async fn dedup_demo() -> DedupDemoResponse {
    let num = REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
    tokio::time::sleep(Duration::from_millis(500)).await;
    DedupDemoResponse { request_number: num, timestamp: format!("{}ms", now) }
}`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
