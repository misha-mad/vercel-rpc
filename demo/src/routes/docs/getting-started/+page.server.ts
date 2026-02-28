import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' | 'shellscript' }> = {
	installCli: {
		lang: 'shellscript',
		code: `cargo install metaxy-cli`
	},
	installCrate: {
		lang: 'shellscript',
		code: `cargo add metaxy`
	},
	writeLambda: {
		lang: 'rust',
		code: `// api/hello.rs
use metaxy::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}`
	},
	installGenerate: {
		lang: 'shellscript',
		code: `metaxy generate --dir api --output src/lib/rpc-types.ts --client-output src/lib/rpc-client.ts`
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

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
