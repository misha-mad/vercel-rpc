import { highlightCode } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' }> = {
	typesRust: {
		lang: 'rust',
		code: `use std::borrow::Cow;
use std::collections::{BTreeSet, HashSet};

#[derive(Serialize)]
pub struct TypeShowcase {
    pub tags: HashSet<String>,      // → string[]
    pub sorted_ids: BTreeSet<i32>,  // → number[] (sorted)
    pub boxed_label: Box<String>,   // → string
    pub cow_message: Cow<'static, str>, // → string
}

#[rpc_query]
async fn types(category: String) -> TypeShowcase {
    let mut tags = HashSet::new();
    tags.insert("rust".into());
    tags.insert("typescript".into());
    tags.insert("rpc".into());

    TypeShowcase {
        tags,
        sorted_ids: [3, 1, 2].into(),
        boxed_label: Box::new(format!("Category: {category}")),
        cow_message: Cow::Borrowed("Hello from Cow!"),
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
