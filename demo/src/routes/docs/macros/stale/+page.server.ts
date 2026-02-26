import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' }> = {
	basic: {
		lang: 'rust',
		code: `// Cache 5 min, serve stale for up to 1 hour while revalidating
#[rpc_query(cache = "5m", stale = "1h")]
async fn get_feed() -> Vec<Post> { ... }`
	},
	private: {
		lang: 'rust',
		code: `// Private cache with stale-while-revalidate
#[rpc_query(cache = "private, 10m", stale = "30m")]
async fn get_dashboard() -> Dashboard { ... }`
	},
	error: {
		lang: 'rust',
		code: `// ❌ Compile error — stale requires cache
// #[rpc_query(stale = "1h")]
// async fn bad() -> Data { ... }`
	},
	headers: {
		lang: 'rust',
		code: `// cache = "5m", stale = "1h" produces:
// Cache-Control: public, max-age=300, stale-while-revalidate=3600

// cache = "private, 10m", stale = "30m" produces:
// Cache-Control: private, max-age=600, stale-while-revalidate=1800`
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
