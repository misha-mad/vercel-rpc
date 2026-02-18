use vercel_rpc::rpc_query;

/// Greet a user by name.
/// Returns a personalized greeting string.
#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}
