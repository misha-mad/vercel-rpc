use serde::{Deserialize, Serialize};
#[cfg(not(test))]
use vercel_rpc_macro::rpc_mutation;

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

fn echo_handler(input: EchoInput) -> EchoOutput {
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

/// Echo a message back, optionally transforming it to uppercase.
#[cfg(not(test))]
#[rpc_mutation]
async fn echo(input: EchoInput) -> EchoOutput {
    echo_handler(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_lowercase() {
        let output = echo_handler(EchoInput {
            message: "Hello".to_string(),
            uppercase: false,
        });
        assert_eq!(output.original, "Hello");
        assert_eq!(output.transformed, "Hello");
        assert_eq!(output.length, 5);
    }

    #[test]
    fn test_echo_uppercase() {
        let output = echo_handler(EchoInput {
            message: "Hello".to_string(),
            uppercase: true,
        });
        assert_eq!(output.original, "Hello");
        assert_eq!(output.transformed, "HELLO");
        assert_eq!(output.length, 5);
    }
}
