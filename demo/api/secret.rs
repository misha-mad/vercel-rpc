use vercel_rpc::{Headers, rpc_query};

/// Access a protected secret.
/// Requires a valid Bearer token in the Authorization header.
#[rpc_query]
async fn secret(headers: Headers) -> Result<String, String> {
    let auth = headers.get("authorization").and_then(|v| v.to_str().ok());
    if auth != Some("Bearer secret-token-123") {
        return Err("Unauthorized: invalid or missing token".into());
    }
    Ok("Top secret: the cake is a lie.".to_string())
}
