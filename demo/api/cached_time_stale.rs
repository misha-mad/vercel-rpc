use metaxy::rpc_query;
use serde::Serialize;
use std::time::SystemTime;

/// Server timestamp with stale-while-revalidate.
#[derive(Serialize)]
pub struct CachedTimeStaleResponse {
    pub timestamp: u64,
    pub generated_at: String,
}

/// Returns server time, cached 10s + stale for 30s while revalidating.
#[rpc_query(cache = "10s", stale = "30s")]
async fn cached_time_stale() -> CachedTimeStaleResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    CachedTimeStaleResponse {
        timestamp: now,
        generated_at: format!("{}s since epoch", now),
    }
}
