use metaxy::rpc_query;
use serde::Serialize;

/// Demonstrates precision loss when large `u64` values are mapped to JS `number`.
///
/// Returns several `u64` values alongside their string representations.
/// Compare the numeric fields (which may lose precision in JavaScript)
/// with the string fields (always exact) to see the difference.
#[derive(Serialize)]
pub struct BigIntDemoResponse {
    /// A small value — fits safely in JS `number`.
    pub small: u64,
    pub small_str: String,

    /// Exactly `Number.MAX_SAFE_INTEGER` (2^53 − 1) — still safe.
    pub max_safe: u64,
    pub max_safe_str: String,

    /// `Number.MAX_SAFE_INTEGER + 2` — precision loss begins.
    pub above_safe: u64,
    pub above_safe_str: String,

    /// `u64::MAX` (2^64 − 1) — massive precision loss.
    pub u64_max: u64,
    pub u64_max_str: String,
}

/// Return several `u64` values to demonstrate BigInt precision boundaries.
///
/// The string fields always carry the exact decimal representation,
/// while the numeric fields may silently lose precision when parsed
/// by JavaScript's `JSON.parse` (which uses IEEE 754 `number`).
#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let small: u64 = 42;
    let max_safe: u64 = 9_007_199_254_740_991; // 2^53 - 1
    let above_safe: u64 = 9_007_199_254_740_993; // 2^53 + 1
    let u64_max: u64 = u64::MAX;

    BigIntDemoResponse {
        small,
        small_str: small.to_string(),
        max_safe,
        max_safe_str: max_safe.to_string(),
        above_safe,
        above_safe_str: above_safe.to_string(),
        u64_max,
        u64_max_str: u64_max.to_string(),
    }
}
