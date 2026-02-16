use serde::Serialize;
use std::collections::HashMap;
#[cfg(not(test))]
use vercel_rpc_macro::rpc_query;

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

fn stats_handler(numbers: Vec<f64>) -> Result<Stats, String> {
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

/// Compute descriptive statistics for a list of numbers.
#[cfg(not(test))]
#[rpc_query]
async fn stats(numbers: Vec<f64>) -> Result<Stats, String> {
    stats_handler(numbers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let r = stats_handler(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(r.count, 5);
        assert_eq!(r.sum, 15.0);
        assert_eq!(r.mean, 3.0);
        assert_eq!(r.min, 1.0);
        assert_eq!(r.max, 5.0);
    }

    #[test]
    fn test_frequencies() {
        let r = stats_handler(vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0]).unwrap();
        assert_eq!(r.frequencies.get("1"), Some(&1));
        assert_eq!(r.frequencies.get("2"), Some(&2));
        assert_eq!(r.frequencies.get("3"), Some(&3));
    }

    #[test]
    fn test_single() {
        let r = stats_handler(vec![42.0]).unwrap();
        assert_eq!(r.count, 1);
        assert_eq!(r.mean, 42.0);
        assert_eq!(r.min, 42.0);
        assert_eq!(r.max, 42.0);
    }

    #[test]
    fn test_empty() {
        let r = stats_handler(vec![]);
        assert_eq!(r.unwrap_err(), "Cannot compute stats for empty list");
    }
}
