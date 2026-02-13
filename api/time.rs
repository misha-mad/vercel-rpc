use serde_json::{Value, json};
use vercel_runtime::{Error, Request, run, service_fn};
use std::time::SystemTime;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let service = service_fn(handler);
    run(service).await
}

async fn handler(_req: Request) -> Result<Value, Error> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(json!({
        "timestamp": now,
        "message": "Current server time in seconds since epoch",
    }))
}
