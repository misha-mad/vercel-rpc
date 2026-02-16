#[cfg(not(test))]
use vercel_rpc_macro::rpc_query;

fn hello_handler(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}

/// Greet a user by name.
/// Returns a personalized greeting string.
#[cfg(not(test))]
#[rpc_query]
async fn hello(name: String) -> String {
    hello_handler(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello() {
        assert_eq!(hello_handler("World".to_string()), "Hello, World from Rust on Vercel!");
    }

    #[test]
    fn test_hello_empty() {
        assert_eq!(hello_handler(String::new()), "Hello,  from Rust on Vercel!");
    }
}
