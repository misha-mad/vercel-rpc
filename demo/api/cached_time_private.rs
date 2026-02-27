use metaxy::rpc_query;
use serde::Serialize;
use std::time::SystemTime;

/// Server timestamp with private (browser-only) cache.
#[derive(Serialize)]
pub struct CachedTimePrivateResponse {
    pub timestamp: u64,
    pub generated_at: String,
}

/// Returns server time, cached in browser only for 1 minute (no CDN).
#[rpc_query(cache = "private, 1m")]
async fn cached_time_private() -> CachedTimePrivateResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    CachedTimePrivateResponse {
        timestamp: now,
        generated_at: format!("{}s since epoch", now),
    }
}
