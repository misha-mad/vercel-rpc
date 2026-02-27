use metaxy::rpc_mutation;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

static STORED_VALUE: AtomicU64 = AtomicU64::new(0);
static CALL_COUNT: AtomicU64 = AtomicU64::new(0);

#[derive(Deserialize, Serialize)]
pub struct IdempotentDemoInput {
    pub value: u64,
}

#[derive(Serialize)]
pub struct IdempotentDemoResponse {
    pub previous: u64,
    pub current: u64,
    pub total_calls: u64,
}

/// Idempotent upsert: sets the stored value. Repeated calls with the same
/// input produce the same result, making it safe to retry on failure.
#[rpc_mutation(idempotent)]
async fn idempotent_demo(input: IdempotentDemoInput) -> IdempotentDemoResponse {
    let previous = STORED_VALUE.swap(input.value, Ordering::Relaxed);
    let total_calls = CALL_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    IdempotentDemoResponse {
        previous,
        current: input.value,
        total_calls,
    }
}
