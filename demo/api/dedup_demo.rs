use metaxy::rpc_query;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::SystemTime;

static REQUEST_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Serialize)]
pub struct DedupDemoResponse {
    pub request_number: u64,
    pub timestamp: String,
}

/// Slow endpoint (500ms) with a request counter.
/// Used to demonstrate request deduplication.
#[rpc_query]
async fn dedup_demo() -> DedupDemoResponse {
    let num = REQUEST_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    DedupDemoResponse {
        request_number: num,
        timestamp: format!("{}ms", now),
    }
}
