import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<
	string,
	{ code: string; lang: 'rust' | 'typescript' | 'toml' | 'shellscript' }
> = {
	configToml: {
		lang: 'toml',
		code: `# rpc.config.toml
[codegen.naming]
fields = "camelCase"`
	},
	configCli: {
		lang: 'shellscript',
		code: `rpc generate --field-naming camelCase`
	},
	exampleRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
struct ServiceStatus {
    uptime_secs: u64,
    version: String,
}`
	},
	preserveTs: {
		lang: 'typescript',
		code: `// fields = "preserve" (default)
export interface ServiceStatus {
  uptime_secs: number;
  version: string;
}`
	},
	camelTs: {
		lang: 'typescript',
		code: `// fields = "camelCase"
export interface ServiceStatus {
  uptimeSecs: number;
  version: string;
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
