use metaxy::rpc_query;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Deserialize, Serialize)]
pub struct TimeoutDemoInput {
    pub sleep_ms: u64,
}

#[derive(Serialize)]
pub struct TimeoutDemoResponse {
    pub requested_ms: u64,
    pub actual_ms: u64,
    pub timeout_ms: u64,
}

#[rpc_query(timeout = "3s")]
async fn timeout_demo(input: TimeoutDemoInput) -> TimeoutDemoResponse {
    let start = Instant::now();

    tokio::time::sleep(std::time::Duration::from_millis(input.sleep_ms)).await;

    TimeoutDemoResponse {
        requested_ms: input.sleep_ms,
        actual_ms: start.elapsed().as_millis() as u64,
        timeout_ms: 3000,
    }
}
