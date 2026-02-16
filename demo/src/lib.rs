pub mod handlers {
    pub mod hello {
        pub fn hello(name: String) -> String {
            format!("Hello, {} from Rust on Vercel!", name)
        }
    }

    pub mod echo {
        use serde::{Deserialize, Serialize};

        /// Input for the echo mutation.
        #[derive(Deserialize, Serialize)]
        pub struct EchoInput {
            pub message: String,
            pub uppercase: bool,
        }

        /// Output returned by the echo mutation.
        #[derive(Serialize)]
        pub struct EchoOutput {
            pub original: String,
            pub transformed: String,
            pub length: u32,
        }

        pub fn echo(input: EchoInput) -> EchoOutput {
            let transformed = if input.uppercase {
                input.message.to_uppercase()
            } else {
                input.message.clone()
            };
            EchoOutput {
                length: transformed.len() as u32,
                original: input.message,
                transformed,
            }
        }
    }

    pub mod math {
        use serde::{Deserialize, Serialize};

        /// Arithmetic operation to perform.
        #[derive(Deserialize, Serialize)]
        pub enum Operation {
            Add,
            Subtract,
            Multiply,
            Divide,
        }

        /// Input for a math calculation.
        #[derive(Deserialize, Serialize)]
        pub struct MathInput {
            pub a: f64,
            pub b: f64,
            pub op: Operation,
        }

        /// Result of a math calculation with a formatted expression.
        #[derive(Debug, Serialize)]
        pub struct MathResult {
            pub result: f64,
            pub expression: String,
        }

        pub fn math(input: MathInput) -> Result<MathResult, String> {
            let (result, symbol) = match input.op {
                Operation::Add => (input.a + input.b, "+"),
                Operation::Subtract => (input.a - input.b, "-"),
                Operation::Multiply => (input.a * input.b, "×"),
                Operation::Divide => {
                    if input.b == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    (input.a / input.b, "÷")
                }
            };
            Ok(MathResult {
                result,
                expression: format!("{} {} {} = {}", input.a, symbol, input.b, result),
            })
        }
    }

    pub mod stats {
        use serde::Serialize;
        use std::collections::HashMap;

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

        pub fn stats(numbers: Vec<f64>) -> Result<Stats, String> {
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
    }

    pub mod status {
        use serde::Serialize;

        /// Overall health of the service.
        #[derive(Serialize, Debug, PartialEq)]
        pub enum HealthStatus {
            Healthy,
            Degraded,
            Down,
        }

        /// Snapshot of service health and version info.
        #[derive(Serialize)]
        pub struct ServiceStatus {
            pub name: String,
            pub status: HealthStatus,
            pub uptime_secs: u64,
            pub version: String,
        }

        pub fn status() -> ServiceStatus {
            let uptime = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            ServiceStatus {
                name: "vercel-rpc-demo".to_string(),
                status: HealthStatus::Healthy,
                uptime_secs: uptime,
                version: "0.1.0".to_string(),
            }
        }
    }

    pub mod time {
        use serde::Serialize;
        use std::time::SystemTime;

        /// Server timestamp with a human-readable message.
        #[derive(Serialize)]
        pub struct TimeResponse {
            pub timestamp: u64,
            pub message: String,
        }

        pub fn time() -> TimeResponse {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            TimeResponse {
                timestamp: now,
                message: "Current server time in seconds since epoch".to_string(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::handlers::*;

    #[test]
    fn test_hello() {
        let result = hello::hello("World".to_string());
        assert_eq!(result, "Hello, World from Rust on Vercel!");
    }

    #[test]
    fn test_hello_empty() {
        let result = hello::hello(String::new());
        assert_eq!(result, "Hello,  from Rust on Vercel!");
    }

    #[test]
    fn test_echo_lowercase() {
        let input = echo::EchoInput {
            message: "Hello".to_string(),
            uppercase: false,
        };
        let output = echo::echo(input);
        assert_eq!(output.original, "Hello");
        assert_eq!(output.transformed, "Hello");
        assert_eq!(output.length, 5);
    }

    #[test]
    fn test_echo_uppercase() {
        let input = echo::EchoInput {
            message: "Hello".to_string(),
            uppercase: true,
        };
        let output = echo::echo(input);
        assert_eq!(output.original, "Hello");
        assert_eq!(output.transformed, "HELLO");
        assert_eq!(output.length, 5);
    }

    #[test]
    fn test_math_add() {
        let input = math::MathInput {
            a: 2.0,
            b: 3.0,
            op: math::Operation::Add,
        };
        let result = math::math(input).unwrap();
        assert_eq!(result.result, 5.0);
        assert_eq!(result.expression, "2 + 3 = 5");
    }

    #[test]
    fn test_math_subtract() {
        let input = math::MathInput {
            a: 10.0,
            b: 4.0,
            op: math::Operation::Subtract,
        };
        let result = math::math(input).unwrap();
        assert_eq!(result.result, 6.0);
    }

    #[test]
    fn test_math_multiply() {
        let input = math::MathInput {
            a: 3.0,
            b: 7.0,
            op: math::Operation::Multiply,
        };
        let result = math::math(input).unwrap();
        assert_eq!(result.result, 21.0);
    }

    #[test]
    fn test_math_divide() {
        let input = math::MathInput {
            a: 10.0,
            b: 2.0,
            op: math::Operation::Divide,
        };
        let result = math::math(input).unwrap();
        assert_eq!(result.result, 5.0);
    }

    #[test]
    fn test_math_divide_by_zero() {
        let input = math::MathInput {
            a: 10.0,
            b: 0.0,
            op: math::Operation::Divide,
        };
        let result = math::math(input);
        assert_eq!(result.unwrap_err(), "Division by zero");
    }

    #[test]
    fn test_stats_basic() {
        let result = stats::stats(vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.sum, 15.0);
        assert_eq!(result.mean, 3.0);
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 5.0);
    }

    #[test]
    fn test_stats_frequencies() {
        let result = stats::stats(vec![1.0, 2.0, 2.0, 3.0, 3.0, 3.0]).unwrap();
        assert_eq!(result.frequencies.get("1"), Some(&1));
        assert_eq!(result.frequencies.get("2"), Some(&2));
        assert_eq!(result.frequencies.get("3"), Some(&3));
    }

    #[test]
    fn test_stats_single() {
        let result = stats::stats(vec![42.0]).unwrap();
        assert_eq!(result.count, 1);
        assert_eq!(result.sum, 42.0);
        assert_eq!(result.mean, 42.0);
        assert_eq!(result.min, 42.0);
        assert_eq!(result.max, 42.0);
    }

    #[test]
    fn test_stats_empty() {
        let result = stats::stats(vec![]);
        assert_eq!(result.unwrap_err(), "Cannot compute stats for empty list");
    }

    #[test]
    fn test_status() {
        let s = status::status();
        assert_eq!(s.name, "vercel-rpc-demo");
        assert_eq!(s.status, status::HealthStatus::Healthy);
        assert_eq!(s.version, "0.1.0");
        assert!(s.uptime_secs > 0);
    }

    #[test]
    fn test_time() {
        let t = time::time();
        assert!(t.timestamp > 0);
        assert_eq!(t.message, "Current server time in seconds since epoch");
    }
}
