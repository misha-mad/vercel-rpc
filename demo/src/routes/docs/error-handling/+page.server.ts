import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	secretRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn secret(req: Request) -> impl IntoResponse {
    let auth = req.headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    match auth {
        Some("Bearer secret-token-123") =>
            (StatusCode::OK, "Access granted!"),
        _ =>
            (StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}`
	},
	secretTs: {
		lang: 'typescript',
		code: `import { RpcError } from './rpc.svelte';

try {
  const result = await client.query('secret');
} catch (e) {
  if (e instanceof RpcError) {
    e.status   // HTTP status code
    e.message  // error message
    e.data     // parsed error body
  }
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
