use metaxy::rpc_query;
use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime};

#[derive(Debug)]
pub struct AppState {
    pub cold_start_at: u64,
    pub init_duration_ms: u64,
    pub request_count: AtomicU64,
}

async fn setup() -> AppState {
    let start = Instant::now();

    let cold_start_at = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Simulate real init work (loading config, connecting to services, etc.)
    tokio::time::sleep(std::time::Duration::from_millis(5)).await;

    let init_duration_ms = start.elapsed().as_millis() as u64;

    AppState {
        cold_start_at,
        init_duration_ms,
        request_count: AtomicU64::new(0),
    }
}

#[derive(Serialize)]
pub struct InitDemoResponse {
    pub cold_start_at: u64,
    pub init_duration_ms: u64,
    pub request_count: u64,
    pub now: u64,
}

#[rpc_query(init = "setup")]
async fn init_demo(state: &AppState) -> InitDemoResponse {
    let count = state.request_count.fetch_add(1, Ordering::Relaxed) + 1;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    InitDemoResponse {
        cold_start_at: state.cold_start_at,
        init_duration_ms: state.init_duration_ms,
        request_count: count,
        now,
    }
}
