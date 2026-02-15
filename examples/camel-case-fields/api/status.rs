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
    pub service_name: String,
    pub status: HealthStatus,
    pub uptime_secs: u64,
    pub api_version: String,
    pub is_healthy: bool,
}

/// Returns current service health, uptime, and version.
#[rpc_query]
async fn status() -> ServiceStatus {
    ServiceStatus {
        service_name: "my-api".to_string(),
        status: HealthStatus::Healthy,
        uptime_secs: 12345,
        api_version: "1.0.0".to_string(),
        is_healthy: true,
    }
}
