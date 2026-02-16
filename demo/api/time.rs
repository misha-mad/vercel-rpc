use serde::Serialize;
use std::time::SystemTime;
#[cfg(not(test))]
use vercel_rpc_macro::rpc_query;

/// Server timestamp with a human-readable message.
#[derive(Serialize)]
pub struct TimeResponse {
    pub timestamp: u64,
    pub message: String,
}

fn time_handler() -> TimeResponse {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    TimeResponse {
        timestamp: now,
        message: "Current server time in seconds since epoch".to_string(),
    }
}

/// Returns the current server time as a Unix timestamp.
#[cfg(not(test))]
#[rpc_query]
async fn time() -> TimeResponse {
    time_handler()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time() {
        let t = time_handler();
        assert!(t.timestamp > 0);
        assert_eq!(t.message, "Current server time in seconds since epoch");
    }
}
