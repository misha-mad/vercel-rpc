import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'shellscript' }> = {
	basic: {
		lang: 'shellscript',
		code: `# Scan default ./api directory
metaxy scan`
	},
	custom: {
		lang: 'shellscript',
		code: `# Scan a custom directory
metaxy scan --dir src/handlers

# Include only specific files
metaxy scan --include "api/v2/**/*.rs"

# Exclude test files
metaxy scan --exclude "**/*_test.rs" --exclude "**/tests/**"`
	},
	output: {
		lang: 'shellscript',
		code: `$ metaxy scan

Discovered procedures:
  query    hello        (String → String)
  query    list_users   (ListUsersInput → Paginated<User>)
  query    get_user     (u64 → User)
  mutation echo         (EchoInput → EchoOutput)
  mutation create_order (OrderInput → Order)

Discovered types:
  struct  User           { id: u64, name: String, email: String }
  struct  ListUsersInput { page: u32, per_page: u32 }
  struct  Paginated<T>   { items: Vec<T>, total: u64, page: u32 }
  struct  EchoInput      { message: String, uppercase: bool }
  struct  EchoOutput     { original: String, transformed: String, length: u32 }
  enum    Role           Viewer | Editor | Admin`
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
