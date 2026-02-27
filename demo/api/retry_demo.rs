use metaxy::rpc_query;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU32, Ordering};

static CALL_COUNT: AtomicU32 = AtomicU32::new(0);

#[derive(Deserialize, Serialize)]
pub struct RetryDemoInput {
    /// How many initial calls should return 500
    pub fail_count: u32,
    /// Pass true to reset the counter before this call
    pub reset: bool,
}

#[derive(Serialize)]
pub struct RetryDemoResponse {
    pub call_number: u32,
    pub total_calls: u32,
    pub message: String,
}

/// Returns 500 for the first `fail_count` calls, then 200.
/// Use `reset: true` to restart the counter.
#[rpc_query]
async fn retry_demo(input: RetryDemoInput) -> Result<RetryDemoResponse, String> {
    if input.reset {
        CALL_COUNT.store(0, Ordering::Relaxed);
    }
    let call_number = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    if call_number <= input.fail_count {
        return Err(format!(
            "Simulated failure (call {} of {} requested failures)",
            call_number, input.fail_count
        ));
    }

    Ok(RetryDemoResponse {
        call_number,
        total_calls: CALL_COUNT.load(Ordering::Relaxed),
        message: format!("Success on call {}", call_number),
    })
}
