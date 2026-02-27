import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'typescript' | 'rust' }> = {
	defaultSerialization: {
		lang: 'typescript',
		code: `// Default: JSON.stringify / JSON.parse
const rpc = createRpcClient({
  baseUrl: '/api',
});`
	},
	customSerialization: {
		lang: 'typescript',
		code: `import superjson from 'superjson';

// Custom serialization — e.g. superjson for Date, BigInt, Map, Set
const rpc = createRpcClient({
  baseUrl: '/api',
  serialize: (data) => superjson.stringify(data),
  deserialize: (text) => superjson.parse(text),
});`
	},
	signature: {
		lang: 'typescript',
		code: `interface RpcClientConfig {
  // ...
  serialize?: (data: unknown) => string;   // default: JSON.stringify
  deserialize?: (text: string) => unknown; // default: JSON.parse
}`
	},
	bigintDemoRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct BigIntDemoValue {
    pub label: String,
    pub exact: String,
    pub as_number: u64,
}

#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let cases: &[(&str, u64)] = &[
        ("small (42)", 42),
        ("MAX_SAFE_INTEGER", 9_007_199_254_740_991),
        ("MAX_SAFE + 2", 9_007_199_254_740_993),
        ("u64::MAX", u64::MAX),
    ];
    // ...
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
