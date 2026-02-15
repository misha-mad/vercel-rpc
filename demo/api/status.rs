use serde::Serialize;
use vercel_rpc_macro::rpc_query;

/// Overall health of the service.
#[derive(Serialize)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Down,
}

/// Snapshot of service health and version info.
#[derive(Serialize)]
pub struct ServiceStatus {
    pub name: String,
    pub status: HealthStatus,
    pub uptime_secs: u64,
    pub version: String,
}

/// Returns current service health, uptime, and version.
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
