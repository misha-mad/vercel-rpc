use serde::Serialize;
use std::collections::HashMap;
use metaxy::rpc_query;

/// Descriptive statistics for a list of numbers.
#[derive(Debug, Serialize)]
pub struct Stats {
    pub count: u32,
    pub sum: f64,
    pub mean: f64,
    pub min: f64,
    pub max: f64,
    pub frequencies: HashMap<String, u32>,
}

/// Compute descriptive statistics for a list of numbers.
#[rpc_query]
async fn stats(numbers: Vec<f64>) -> Result<Stats, String> {
    if numbers.is_empty() {
        return Err("Cannot compute stats for empty list".to_string());
    }

    let count = numbers.len() as u32;
    let sum: f64 = numbers.iter().sum();
    let mean = sum / count as f64;
    let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let mut frequencies = HashMap::new();
    for n in &numbers {
        let key = format!("{}", n);
        *frequencies.entry(key).or_insert(0) += 1;
    }

    Ok(Stats {
        count,
        sum,
        mean,
        min,
        max,
        frequencies,
    })
}
