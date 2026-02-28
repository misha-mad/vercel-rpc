import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	configToml: {
		lang: 'toml',
		code: `# metaxy.config.toml
[codegen]
bigint_types = ["i64", "u64", "i128", "u128"]`
	},
	configCli: {
		lang: 'shellscript',
		code: `metaxy generate --bigint-type i64 --bigint-type u64`
	},
	defaultTs: {
		lang: 'typescript',
		code: `// Default: all integer types map to number
export interface Stats {
  total_users: number;   // u64 → number
  total_bytes: number;   // i64 → number
}`
	},
	bigintTs: {
		lang: 'typescript',
		code: `// With bigint_types = ["i64", "u64"]
export interface Stats {
  total_users: bigint;   // u64 → bigint
  total_bytes: bigint;   // i64 → bigint
}`
	},
	bigintDemoRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct BigIntDemoValue {
    pub label: String,
    pub exact: String,  // exact decimal string
    pub as_number: u64, // → number (may lose precision)
}

#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let cases: &[(&str, u64)] = &[
        ("small (42)", 42),
        ("MAX_SAFE_INTEGER", 9_007_199_254_740_991), // 2^53 - 1
        ("MAX_SAFE + 2", 9_007_199_254_740_993),     // 2^53 + 1
        ("u64::MAX", u64::MAX),                      // 2^64 - 1
    ];

    BigIntDemoResponse {
        values: cases.iter().map(|(label, val)| BigIntDemoValue {
            label: label.to_string(),
            exact: val.to_string(),
            as_number: *val,
        }).collect(),
    }
}`
	},
	losslessClient: {
		lang: 'typescript',
		code: `import { parse, parseNumberAndBigInt } from 'lossless-json';

const client = createRpcClient({
  baseUrl: '/api',
  deserialize: (text) =>
    parse(text, undefined, parseNumberAndBigInt),
});

// Safe integers stay as number, large ones become BigInt
const res = await client.query('bigint_demo');
typeof res.values[0].as_number; // "number"  (42)
typeof res.values[3].as_number; // "bigint"  (u64::MAX)`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
