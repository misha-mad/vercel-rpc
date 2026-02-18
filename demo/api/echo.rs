use serde::{Deserialize, Serialize};
use vercel_rpc::rpc_mutation;

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

/// Echo a message back, optionally transforming it to uppercase.
#[rpc_mutation]
async fn echo(input: EchoInput) -> EchoOutput {
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
