use std::borrow::Cow;
use std::collections::{BTreeSet, HashSet};

use serde::Serialize;
use vercel_rpc::rpc_query;

/// Showcases expanded type mappings: sets, smart pointers, and Cow.
///
/// All wrapper types serialize transparently — `Box<T>` and `Cow<T>` become `T`,
/// while `HashSet<T>` and `BTreeSet<T>` become `T[]`.
#[derive(Serialize)]
pub struct TypeShowcase {
    /// `HashSet<String>` → `string[]` in TypeScript.
    pub tags: HashSet<String>,
    /// `BTreeSet<i32>` → `number[]` in TypeScript (sorted in JSON).
    pub sorted_ids: BTreeSet<i32>,
    /// `Box<String>` → `string` in TypeScript (transparent wrapper).
    pub boxed_label: Box<String>,
    /// `Cow<'static, str>` → `string` in TypeScript (transparent wrapper).
    pub cow_message: Cow<'static, str>,
}

/// Return a type showcase demonstrating expanded type mappings.
///
/// Accepts a category name and returns a `TypeShowcase` struct using
/// `HashSet`, `BTreeSet`, `Box`, and `Cow` — all mapped to their
/// natural TypeScript equivalents.
#[rpc_query]
async fn types(category: String) -> TypeShowcase {
    let mut tags = HashSet::new();
    tags.insert("rust".to_string());
    tags.insert("typescript".to_string());
    tags.insert("rpc".to_string());

    let mut sorted_ids = BTreeSet::new();
    sorted_ids.insert(3);
    sorted_ids.insert(1);
    sorted_ids.insert(2);

    TypeShowcase {
        tags,
        sorted_ids,
        boxed_label: Box::new(format!("Category: {}", category)),
        cow_message: Cow::Borrowed("Hello from Cow — serialized as a plain string!"),
    }
}
