use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use metaxy::rpc_query;
use serde::Serialize;

/// Showcases every Rust → TypeScript type mapping in a single struct.
#[derive(Serialize)]
pub struct TypeShowcase {
    // ── Primitives ──────────────────────────────────────────────
    /// `String` → `string`
    pub string_val: String,
    /// `i32` → `number`
    pub integer: i32,
    /// `f64` → `number`
    pub float: f64,
    /// `bool` → `boolean`
    pub flag: bool,

    // ── Collections ─────────────────────────────────────────────
    /// `Vec<String>` → `string[]`
    pub vec_items: Vec<String>,
    /// `HashSet<String>` → `string[]`
    pub hash_set: HashSet<String>,
    /// `BTreeSet<i32>` → `number[]` (sorted in JSON)
    pub btree_set: BTreeSet<i32>,

    // ── Option ──────────────────────────────────────────────────
    /// `Option<String>` → `string | null` (present)
    pub optional_present: Option<String>,
    /// `Option<String>` → `string | null` (absent → null)
    pub optional_absent: Option<String>,

    // ── Maps ────────────────────────────────────────────────────
    /// `HashMap<String, i32>` → `Record<string, number>`
    pub hash_map: HashMap<String, i32>,
    /// `BTreeMap<String, i32>` → `Record<string, number>` (sorted keys)
    pub btree_map: BTreeMap<String, i32>,

    // ── Smart pointers (transparent) ────────────────────────────
    /// `Box<String>` → `string`
    pub boxed: Box<String>,
    /// `Cow<str>` → `string`
    pub cow: Cow<'static, str>,

    // ── Tuples & arrays ─────────────────────────────────────────
    /// `(String, i32, bool)` → `[string, number, boolean]`
    pub tuple: (String, i32, bool),
    /// `[i32; 3]` → `number[]`
    pub fixed_array: [i32; 3],
}

/// Return a showcase of every supported type mapping.
#[rpc_query]
async fn types() -> TypeShowcase {
    let mut hash_set = HashSet::new();
    hash_set.insert("rust".to_string());
    hash_set.insert("typescript".to_string());
    hash_set.insert("rpc".to_string());

    let mut btree_set = BTreeSet::new();
    btree_set.insert(3);
    btree_set.insert(1);
    btree_set.insert(2);

    let mut hash_map = HashMap::new();
    hash_map.insert("alpha".to_string(), 1);
    hash_map.insert("beta".to_string(), 2);

    let mut btree_map = BTreeMap::new();
    btree_map.insert("x".to_string(), 10);
    btree_map.insert("y".to_string(), 20);
    btree_map.insert("z".to_string(), 30);

    TypeShowcase {
        string_val: "hello".to_string(),
        integer: 42,
        float: 2.718,
        flag: true,
        vec_items: vec!["one".into(), "two".into(), "three".into()],
        hash_set,
        btree_set,
        optional_present: Some("I'm here".to_string()),
        optional_absent: None,
        hash_map,
        btree_map,
        boxed: Box::new("boxed value".to_string()),
        cow: Cow::Borrowed("borrowed cow"),
        tuple: ("hello".to_string(), 42, true),
        fixed_array: [10, 20, 30],
    }
}
