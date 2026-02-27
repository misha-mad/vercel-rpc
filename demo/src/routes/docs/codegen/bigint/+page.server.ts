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
pub struct BigIntDemoResponse {
    pub small: u64,          // fits in JS number
    pub small_str: String,
    pub max_safe: u64,       // 2^53 - 1 (last safe value)
    pub max_safe_str: String,
    pub above_safe: u64,     // 2^53 + 1 (precision loss!)
    pub above_safe_str: String,
    pub u64_max: u64,        // 2^64 - 1 (massive loss)
    pub u64_max_str: String,
}

#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let small: u64 = 42;
    let max_safe: u64 = 9_007_199_254_740_991;
    let above_safe: u64 = 9_007_199_254_740_993;
    let u64_max: u64 = u64::MAX;

    BigIntDemoResponse {
        small, small_str: small.to_string(),
        max_safe, max_safe_str: max_safe.to_string(),
        above_safe, above_safe_str: above_safe.to_string(),
        u64_max, u64_max_str: u64_max.to_string(),
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
