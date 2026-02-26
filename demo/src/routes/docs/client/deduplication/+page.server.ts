import { highlightCode } from '$lib/highlight.server';
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
