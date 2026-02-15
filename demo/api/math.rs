use serde::{Deserialize, Serialize};
use vercel_rpc_macro::rpc_query;

#[derive(Deserialize, Serialize)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Deserialize, Serialize)]
pub struct MathInput {
    pub a: f64,
    pub b: f64,
    pub op: Operation,
}

#[derive(Serialize)]
pub struct MathResult {
    pub result: f64,
    pub expression: String,
}

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
