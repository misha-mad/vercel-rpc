use serde::{Deserialize, Serialize};
use vercel_rpc::rpc_query;

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

/// Perform a math operation. Returns an error on division by zero.
#[rpc_query]
async fn math(input: MathInput) -> Result<MathResult, String> {
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
