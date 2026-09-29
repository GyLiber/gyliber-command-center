use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct StateSnapshot {
    pub schema_version: u8,
    pub release: &'static str,
    pub environment: Option<String>,
    pub deployment_commit: Option<String>,
    pub deployment_branch: Option<String>,
    pub public_surface: &'static str,
    pub authenticated_surface: &'static str,
    pub sensitive_data: &'static str,
    pub data_freshness: &'static str,
    pub observed_at_unix: u64,
}

pub fn snapshot() -> StateSnapshot {
    StateSnapshot {
        schema_version: 2,
        release: env!("CARGO_PKG_VERSION"),
        environment: std::env::var("APP_ENV").ok(),
        deployment_commit: std::env::var("RENDER_GIT_COMMIT").ok(),
        deployment_branch: std::env::var("RENDER_GIT_BRANCH").ok(),
        public_surface: "operational",
        authenticated_surface: "protected",
        sensitive_data: "disabled",
        data_freshness: "fresh",
        observed_at_unix: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after Unix epoch")
            .as_secs(),
    }
}

#[cfg(test)]
mod tests {
    use super::snapshot;

    #[test]
    fn snapshot_contains_safe_operational_metadata_only() {
        let snapshot = snapshot();
        assert_eq!(snapshot.schema_version, 2);
        assert_eq!(snapshot.release, env!("CARGO_PKG_VERSION"));
        assert_eq!(snapshot.sensitive_data, "disabled");
        assert_eq!(snapshot.authenticated_surface, "protected");
        assert!(snapshot.observed_at_unix > 0);
    }
}
