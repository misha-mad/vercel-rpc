import { highlightCode } from '$lib/highlight.server';
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
    pub exact: String,      // always-correct string
    pub as_number: u64,     // → number (may lose precision)
}

#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let cases: &[(&str, u64)] = &[
        ("small (42)", 42),
        ("MAX_SAFE_INTEGER", 9_007_199_254_740_991),
        ("MAX_SAFE + 2", 9_007_199_254_740_993),
        ("u64::MAX", u64::MAX),
    ];
    // exact: val.to_string(), as_number: *val
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
