use serde::Serialize;
use vercel_rpc_macro::rpc_query;

#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Down,
}

#[derive(Serialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: HealthStatus,
    pub uptime_secs: u64,
    pub version: String,
}

#[rpc_query]
async fn status() -> ServiceStatus {
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    ServiceStatus {
        name: "vercel-rpc-demo".to_string(),
        status: HealthStatus::Healthy,
        uptime_secs: uptime,
        version: "0.1.0".to_string(),
    }
}
