import { highlightCode } from '$lib/highlight.server';
import { createRpcClient } from '$lib/rpc-client';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' | 'rust' }> = {
	defaultFetch: {
		lang: 'typescript',
		code: `// Default: uses globalThis.fetch
const rpc = createRpcClient({ baseUrl: '/api' });`
	},
	ssrFetch: {
		lang: 'typescript',
		code: `// SvelteKit server load — forward cookies & resolve relative URLs
export const load: PageServerLoad = async ({ fetch }) => {
  const rpc = createRpcClient({ baseUrl: '/api', fetch });
  const data = await rpc.query('cookie_demo');
  return { ssrResult: data };
};`
	},
	testFetch: {
		lang: 'typescript',
		code: `// Mock fetch for unit tests
const mockFetch = async (url: string) =>
  new Response(JSON.stringify({ ok: true }), { status: 200 });

const rpc = createRpcClient({ baseUrl: '/api', fetch: mockFetch });`
	},
	cookieDemoRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn cookie_demo(headers: Headers) -> CookieDemoResponse {
    let session = headers.get("cookie")
        .and_then(|v| v.to_str().ok())
        .and_then(|c| c.split(';').find(|s| s.trim().starts_with("session=")))
        .map(|s| s.trim().trim_start_matches("session=").to_string());

    match session {
        Some(val) => CookieDemoResponse {
            authenticated: true,
            message: "Authenticated via session cookie".into(),
            cookie_value: Some(val),
        },
        None => CookieDemoResponse {
            authenticated: false,
            message: "No session cookie — client fetch doesn't forward server cookies".into(),
            cookie_value: None,
        },
    }
}`
	}
};

export const load: PageServerLoad = async ({ fetch }) => {
	const entries = Object.entries(codeBlocks);
	const results = await Promise.all(entries.map(([, { code, lang }]) => highlightCode(code, lang)));
	const highlighted: Record<string, string> = {};
	entries.forEach(([key], i) => {
		highlighted[key] = results[i];
	});

	// SSR call with platform fetch (forwards cookies)
	let ssrResult: unknown = null;
	try {
		const ssrRpc = createRpcClient({ baseUrl: '/api', fetch });
		ssrResult = await ssrRpc.query('cookie_demo');
	} catch (e) {
		ssrResult = { authenticated: false, message: `SSR error: ${e}`, cookie_value: null };
	}

	return { highlighted, ssrResult };
};
