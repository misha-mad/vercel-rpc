import { highlightBlocks } from '$lib/highlight.server';
import type { PageServerLoad } from './$types';

const codeBlocks: Record<string, { code: string; lang: 'rust' }> = {
	typesRust: {
		lang: 'rust',
		code: `use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

#[derive(Serialize)]
pub struct TypeShowcase {
    pub string_val: String,                   // → string
    pub integer: i32,                         // → number
    pub float: f64,                           // → number
    pub flag: bool,                           // → boolean
    pub vec_items: Vec<String>,               // → string[]
    pub hash_set: HashSet<String>,            // → string[]
    pub btree_set: BTreeSet<i32>,             // → number[]
    pub optional_present: Option<String>,     // → string | null
    pub optional_absent: Option<String>,      // → string | null
    pub hash_map: HashMap<String, i32>,       // → Record<string, number>
    pub btree_map: BTreeMap<String, i32>,     // → Record<string, number>
    pub boxed: Box<String>,                   // → string
    pub cow: Cow<'static, str>,               // → string
    pub tuple: (String, i32, bool),           // → [string, number, boolean]
    pub fixed_array: [i32; 3],                // → number[]
}

#[rpc_query]
async fn types() -> TypeShowcase { /* ... */ }`
	}
};

export const load: PageServerLoad = () => highlightBlocks(codeBlocks);
