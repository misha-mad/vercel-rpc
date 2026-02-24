import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	fullConfig: {
		lang: 'rust',
		code: `[input]
dir = "api"
include = ["**/*.rs"]
exclude = []

[output]
types = "src/lib/rpc-types.ts"
client = "src/lib/rpc-client.ts"
svelte = "src/lib/rpc.svelte.ts"     # opt-in
# react = "src/lib/rpc.react.ts"
# vue = "src/lib/rpc.vue.ts"
# solid = "src/lib/rpc.solid.ts"

[output.imports]
types_path = "./rpc-types"
extension = ""                        # e.g. ".js" for ESM

[codegen]
preserve_docs = false
branded_newtypes = false
# bigint_types = ["i64", "u64"]

[codegen.naming]
fields = "preserve"                   # or "camelCase"

[codegen.type_overrides]
# "chrono::DateTime" = "string"
# "uuid::Uuid" = "string"

[watch]
debounce_ms = 200
clear_screen = false`
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
