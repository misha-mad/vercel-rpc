use vercel_rpc::rpc_query;

/// Access a protected secret.
/// Requires a valid Bearer token in the Authorization header.
#[rpc_query]
async fn secret() -> String {
    "Top secret: the cake is a lie.".to_string()
}
