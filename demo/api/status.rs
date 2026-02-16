use serde::Serialize;
#[cfg(not(test))]
use vercel_rpc_macro::rpc_query;

/// Overall health of the service.
#[derive(Debug, PartialEq, Serialize)]
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

fn status_handler() -> ServiceStatus {
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

/// Returns current service health, uptime, and version.
#[cfg(not(test))]
#[rpc_query]
async fn status() -> ServiceStatus {
    status_handler()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status() {
        let s = status_handler();
        assert_eq!(s.name, "vercel-rpc-demo");
        assert_eq!(s.status, HealthStatus::Healthy);
        assert_eq!(s.version, "0.1.0");
        assert!(s.uptime_secs > 0);
    }
}
