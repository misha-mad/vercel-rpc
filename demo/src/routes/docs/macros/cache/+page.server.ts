import { highlightBlocks } from '$lib/highlight.server';
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
	combined: {
		lang: 'rust',
		code: `// Combine with other attributes
#[rpc_query(cache = "1h", timeout = "5s", init = "setup")]
async fn fast_cached(db: &PgPool) -> Stats { ... }

// Compile error — mutations cannot be cached
// #[rpc_mutation(cache = "1h")]
// async fn create_order(input: OrderInput) -> Order { ... }`
	},
	cachedTimeRust: {
		lang: 'rust',
		code: `#[rpc_query(cache = "30s")]
async fn cached_time() -> CachedTimeResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap().as_secs();

    CachedTimeResponse {
        timestamp: now,
        generated_at: format!("{}s since epoch", now),
    }
}`
	},
	cachedTimePrivateRust: {
		lang: 'rust',
		code: `#[rpc_query(cache = "private, 1m")]
async fn cached_time_private() -> CachedTimePrivateResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap().as_secs();

    CachedTimePrivateResponse {
        timestamp: now,
        generated_at: format!("{}s since epoch", now),
    }
}`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
