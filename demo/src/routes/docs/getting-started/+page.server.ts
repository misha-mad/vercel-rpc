import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' | 'shellscript' }> = {
	installCli: {
		lang: 'shellscript',
		code: `cargo install vercel-rpc-cli`
	},
	installCrate: {
		lang: 'shellscript',
		code: `cargo add vercel-rpc`
	},
	installGenerate: {
		lang: 'shellscript',
		code: `rpc generate --dir api --output src/lib/rpc-types.ts --client-output src/lib/rpc-client.ts`
	},
	gettingStartedRust: {
		lang: 'rust',
		code: `#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}`
	},
	gettingStartedTs: {
		lang: 'typescript',
		code: `import { createRpcClient } from './rpc-client';

const rpc = createRpcClient({ baseUrl: '/api' });
const greeting = await rpc.query('hello', 'World');
// greeting: string — "Hello, World from Rust on Vercel!"`
	},
	gettingStartedSvelte: {
		lang: 'typescript',
		code: `import { createQuery } from './rpc.svelte';

let name = $state('World');
const hello = createQuery(rpc, 'hello', () => name);
// hello.data reactively updates when 'name' changes`
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
