import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	cacheRust: {
		lang: 'rust',
		code: `// Cache for 1 hour on Vercel CDN
#[rpc_query(cache = "1h")]
async fn get_products() -> Vec<Product> { ... }

// Cache 5 min + stale-while-revalidate 1 hour
#[rpc_query(cache = "5m", stale = "1h")]
async fn get_feed() -> Vec<Post> { ... }

// Browser-only cache (no CDN)
#[rpc_query(cache = "private, 10m")]
async fn get_profile() -> Profile { ... }`
	},
	initRust: {
		lang: 'rust',
		code: `// Side-effects only (logger, dotenv)
async fn setup_logger() {
    env_logger::init();
}

#[rpc_query(init = "setup_logger")]
async fn get_data() -> Data { ... }

// With shared state (DB pool, HTTP client)
async fn setup_db() -> PgPool {
    PgPool::connect(&env::var("DATABASE_URL").unwrap()).await.unwrap()
}

#[rpc_query(init = "setup_db")]
async fn get_users(pool: &PgPool) -> Vec<User> {
    sqlx::query_as("SELECT * FROM users")
        .fetch_all(pool).await.unwrap()
}`
	},
	timeoutRust: {
		lang: 'rust',
		code: `// 30 second server-side timeout
#[rpc_query(timeout = "30s")]
async fn slow_query() -> Report { ... }

// Combine with other attributes
#[rpc_query(cache = "1h", timeout = "5s", init = "setup")]
async fn fast_cached(db: &PgPool) -> Stats { ... }`
	},
	idempotentRust: {
		lang: 'rust',
		code: `// Safe to retry on failure
#[rpc_mutation(idempotent)]
async fn upsert_user(input: UpsertInput) -> User { ... }

// NOT safe to retry (default)
#[rpc_mutation]
async fn create_order(input: OrderInput) -> Order { ... }`
	}
};

export const load: PageServerLoad = async () => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(
		entries.map(([, { code, lang }]) => highlightCode(code, lang))
	);
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});
	return { highlighted };
};
