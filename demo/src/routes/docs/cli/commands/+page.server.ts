import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'shellscript' }> = {
	install: {
		lang: 'shellscript',
		code: `cargo install metaxy-cli`
	},
	usage: {
		lang: 'shellscript',
		code: `# One-shot code generation
metaxy generate

# Debug — print discovered procedures and types
metaxy scan

# Watch mode — regenerate on .rs file changes
metaxy watch`
	},
	configFlags: {
		lang: 'shellscript',
		code: `# Explicit config file
metaxy generate --config ./custom-metaxy.toml

# Ignore config file entirely
metaxy generate --no-config --dir src/api`
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
