import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }> = {
	newtypeRust: {
		lang: 'rust',
		code: `pub struct UserId(pub String);
pub struct OrderId(pub String);
pub struct Pair(pub String, pub i32);`
	},
	newtypeDefaultTs: {
		lang: 'typescript',
		code: `// Default: plain type aliases
type UserId = string;
type OrderId = string;
type Pair = [string, number];

// Problem: UserId and OrderId are interchangeable
const userId: UserId = "u-123";
const orderId: OrderId = userId; // no error!`
	},
	newtypeBrandedTs: {
		lang: 'typescript',
		code: `// With branded_newtypes = true
type UserId = string & { readonly __brand: "UserId" };
type OrderId = string & { readonly __brand: "OrderId" };
type Pair = [string, number]; // tuples unchanged

// Now they're distinct at compile time
const userId: UserId = "u-123" as UserId;
const orderId: OrderId = userId; // TS error!`
	},
	configToml: {
		lang: 'toml',
		code: `# rpc.config.toml
[codegen]
branded_newtypes = true`
	},
	configCli: {
		lang: 'shellscript',
		code: `rpc generate --branded-newtypes`
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
