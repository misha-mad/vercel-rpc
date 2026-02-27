use metaxy::rpc_query;
use serde::Serialize;
use std::time::SystemTime;

/// Cached server timestamp — public CDN cache for 30 seconds.
#[derive(Serialize)]
pub struct CachedTimeResponse {
    pub timestamp: u64,
    pub generated_at: String,
}

/// Returns server time, cached on CDN for 30 seconds.
#[rpc_query(cache = "30s")]
async fn cached_time() -> CachedTimeResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    CachedTimeResponse {
        timestamp: now,
        generated_at: format!("{}s since epoch", now),
    }
}
