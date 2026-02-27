use metaxy::rpc_query;
use serde::Serialize;

/// A single row showing a u64 value with its exact string representation.
#[derive(Serialize)]
pub struct BigIntDemoValue {
    /// Human-readable label for this row.
    pub label: String,
    /// Exact decimal string (never loses precision).
    pub exact: String,
    /// `u64` field — maps to `number`. May lose precision for large values.
    pub as_number: u64,
}

/// Demonstrates precision loss when large `u64` values are mapped to JS `number`.
#[derive(Serialize)]
pub struct BigIntDemoResponse {
    pub values: Vec<BigIntDemoValue>,
}

/// Return a set of u64 values at precision boundaries.
#[rpc_query]
async fn bigint_demo() -> BigIntDemoResponse {
    let cases: &[(&str, u64)] = &[
        ("small (42)", 42),
        ("MAX_SAFE_INTEGER", 9_007_199_254_740_991), // 2^53 - 1
        ("MAX_SAFE + 2", 9_007_199_254_740_993),     // 2^53 + 1
        ("u64::MAX", u64::MAX),                      // 2^64 - 1
    ];

    BigIntDemoResponse {
        values: cases
            .iter()
            .map(|(label, val)| BigIntDemoValue {
                label: label.to_string(),
                exact: val.to_string(),
                as_number: *val,
            })
            .collect(),
    }
}
