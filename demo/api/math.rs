use serde::{Deserialize, Serialize};
#[cfg(not(test))]
use vercel_rpc_macro::rpc_query;

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

fn math_handler(input: MathInput) -> Result<MathResult, String> {
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

/// Perform a math operation. Returns an error on division by zero.
#[cfg(not(test))]
#[rpc_query]
async fn math(input: MathInput) -> Result<MathResult, String> {
    math_handler(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let r = math_handler(MathInput { a: 2.0, b: 3.0, op: Operation::Add }).unwrap();
        assert_eq!(r.result, 5.0);
        assert_eq!(r.expression, "2 + 3 = 5");
    }

    #[test]
    fn test_subtract() {
        let r = math_handler(MathInput { a: 10.0, b: 4.0, op: Operation::Subtract }).unwrap();
        assert_eq!(r.result, 6.0);
    }

    #[test]
    fn test_multiply() {
        let r = math_handler(MathInput { a: 3.0, b: 7.0, op: Operation::Multiply }).unwrap();
        assert_eq!(r.result, 21.0);
    }

    #[test]
    fn test_divide() {
        let r = math_handler(MathInput { a: 10.0, b: 2.0, op: Operation::Divide }).unwrap();
        assert_eq!(r.result, 5.0);
    }

    #[test]
    fn test_divide_by_zero() {
        let r = math_handler(MathInput { a: 10.0, b: 0.0, op: Operation::Divide });
        assert_eq!(r.unwrap_err(), "Division by zero");
    }
}
