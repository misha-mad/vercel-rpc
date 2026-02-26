import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' }> = {
	basic: {
		lang: 'rust',
		code: `// Cache for 1 hour on Vercel CDN
#[rpc_query(cache = "1h")]
async fn get_products() -> Vec<Product> { ... }`
	},
	durations: {
		lang: 'rust',
		code: `#[rpc_query(cache = "30s")]   // 30 seconds
#[rpc_query(cache = "5m")]    // 5 minutes
#[rpc_query(cache = "1h")]    // 1 hour
#[rpc_query(cache = "1d")]    // 1 day`
	},
	private: {
		lang: 'rust',
		code: `// Browser-only cache (no CDN) — user-specific data
#[rpc_query(cache = "private, 10m")]
async fn get_profile() -> Profile { ... }

// Public (default) — shared across all users on CDN
#[rpc_query(cache = "1h")]
async fn get_products() -> Vec<Product> { ... }`
	},
	withStale: {
		lang: 'rust',
		code: `// Cache 5 min + serve stale for up to 1 hour while revalidating
#[rpc_query(cache = "5m", stale = "1h")]
async fn get_feed() -> Vec<Post> { ... }`
	},
	combined: {
		lang: 'rust',
		code: `// Combine with other attributes
#[rpc_query(cache = "1h", timeout = "5s", init = "setup")]
async fn fast_cached(db: &PgPool) -> Stats { ... }

// ❌ Compile error — mutations cannot be cached
// #[rpc_mutation(cache = "1h")]
// async fn create_order(input: OrderInput) -> Order { ... }`
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
