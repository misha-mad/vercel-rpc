import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' | 'typescript' }> = {
	genericsRust: {
		lang: 'rust',
		code: `#[derive(Serialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub total: u64,
    pub page: u32,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
}

#[rpc_query]
async fn list_users() -> Paginated<User> {
    // ...
}`
	},
	genericsTs: {
		lang: 'typescript',
		code: `interface Paginated<T> {
  items: T[];
  total: number;
  page: number;
}

interface User {
  id: number;
  name: string;
}

// list_users returns Paginated<User>`
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
