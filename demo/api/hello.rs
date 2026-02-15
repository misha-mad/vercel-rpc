use vercel_rpc_macro::rpc_query;

#[rpc_query]
async fn hello(name: String) -> String {
    format!("Hello, {} from Rust on Vercel!", name)
}
