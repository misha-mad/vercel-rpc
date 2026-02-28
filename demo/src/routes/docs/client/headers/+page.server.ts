import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' | 'rust' }> = {
	staticHeaders: {
		lang: 'typescript',
		code: `const rpc = createRpcClient({
  baseUrl: '/api',
  headers: {
    Authorization: 'Bearer my-token',
    'X-App-Version': '1.0.0',
  },
});`
	},
	asyncHeaders: {
		lang: 'typescript',
		code: `// Async headers — called before every request
const rpc = createRpcClient({
  baseUrl: '/api',
  headers: async () => ({
    Authorization: \`Bearer \${await getToken()}\`,
  }),
});`
	},
	perCallHeaders: {
		lang: 'typescript',
		code: `// Override or add headers for a single call
const result = await rpc.query('secret', input, {
  headers: { Authorization: 'Bearer one-time-token' },
});`
	},
	mergeOrder: {
		lang: 'typescript',
		code: `// Merge order: client headers → per-call headers
// Per-call headers override client headers with the same key.
const rpc = createRpcClient({
  baseUrl: '/api',
  headers: { Authorization: 'Bearer default', 'X-App': 'myapp' },
});

// This call sends: Authorization: "Bearer override", X-App: "myapp"
await rpc.query('secret', input, {
  headers: { Authorization: 'Bearer override' },
});`
	},
	secretRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn secret(headers: Headers) -> Result<String, String> {
    let auth = headers.get("authorization")
        .and_then(|v| v.to_str().ok());
    if auth != Some("Bearer secret-token-123") {
        return Err("Unauthorized: invalid or missing token".into());
    }
    Ok("Top secret: the cake is a lie.".to_string())
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
