use metaxy::{Headers, rpc_query};
use serde::Serialize;

#[derive(Serialize)]
pub struct CookieDemoResponse {
    pub authenticated: bool,
    pub message: String,
    pub cookie_value: Option<String>,
}

/// Checks for a `session` cookie in the request headers.
/// Returns OK if present, error details if missing.
#[rpc_query]
async fn cookie_demo(headers: Headers) -> CookieDemoResponse {
    let cookies = headers.get("cookie").and_then(|v| v.to_str().ok());
    let session = cookies.and_then(|c| {
        c.split(';')
            .map(|s| s.trim())
            .find(|s| s.starts_with("session="))
            .map(|s| s.trim_start_matches("session=").to_string())
    });

    match session {
        Some(val) => CookieDemoResponse {
            authenticated: true,
            message: "Authenticated via session cookie".into(),
            cookie_value: Some(val),
        },
        None => CookieDemoResponse {
            authenticated: false,
            message: "No session cookie found — client fetch does not forward server cookies"
                .into(),
            cookie_value: None,
        },
    }
}
