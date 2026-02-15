use serde::Serialize;
use std::time::SystemTime;
use vercel_rpc_macro::rpc_query;

#[derive(Serialize)]
pub struct TimeResponse {
    pub timestamp: u64,
    pub message: String,
}

#[rpc_query]
async fn time() -> TimeResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    TimeResponse {
        timestamp: now,
        message: "Current server time in seconds since epoch".to_string(),
    }
}
