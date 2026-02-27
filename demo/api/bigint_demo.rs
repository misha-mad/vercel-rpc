use metaxy::rpc_query;
use serde::Serialize;

/// A single row comparing `number` (u64) vs `bigint` (u128) for the same value.
#[derive(Serialize)]
pub struct BigIntDemoValue {
    /// Human-readable label for this row.
    pub label: String,
    /// Exact decimal string for reference.
    pub exact: String,
    /// `u64` field — maps to `number` by default. May lose precision.
    pub as_number: u64,
    /// `u128` field — maps to `bigint` via `bigint_types`. Always exact.
    pub as_bigint: u128,
}

/// Demonstrates `number` vs `bigint` precision for large integers.
///
/// Each row carries the same value as both `u64` (`number`) and
/// `u128` (`bigint`). Compare them to see where `number` loses
/// precision and `bigint` stays exact.
#[derive(Serialize)]
pub struct BigIntDemoResponse {
    pub values: Vec<BigIntDemoValue>,
}

/// Return a set of u64 values at precision boundaries.
#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let cases: &[(&str, u64)] = &[
        ("small (42)", 42),
        ("MAX_SAFE_INTEGER", 9_007_199_254_740_991),       // 2^53 - 1
        ("MAX_SAFE + 2", 9_007_199_254_740_993),           // 2^53 + 1
        ("u64::MAX", u64::MAX),                             // 2^64 - 1
    ];

    BigIntDemoResponse {
        values: cases
            .iter()
            .map(|(label, val)| BigIntDemoValue {
                label: label.to_string(),
                exact: val.to_string(),
                as_number: *val,
                as_bigint: *val as u128,
            })
            .collect(),
    }
}
