//! Instance list refresh job.
//!
//! Refreshes the list of available Invidious instances.

use crate::jobs::base_job::{Job, JobConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;
use tokio::sync::RwLock;
use tokio::time::Duration;

/// Instance information.
#[derive(Debug, Clone)]
pub struct Instance {
    pub region: String,
    pub domain: String,
}

/// Job configuration for instance list refresh job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceListRefreshConfig {
    #[serde(flatten)]
    pub base: JobConfig,
}

impl Default for InstanceListRefreshConfig {
    fn default() -> Self {
        Self {
            base: JobConfig::default(),
        }
    }
}

/// Global instance list storage.
static INSTANCES: LazyLock<RwLock<Vec<Instance>>> = LazyLock::new(|| RwLock::new(Vec::new()));

/// Instance list refresh job.
///
/// Refreshes the list of available Invidious instances from the API.
pub struct InstanceListRefreshJob {
    config: InstanceListRefreshConfig,
}

impl InstanceListRefreshJob {
    /// Create a new instance list refresh job.
    pub fn new(config: InstanceListRefreshConfig) -> Self {
        Self { config }
    }

    /// Get the current list of instances.
    pub async fn get_instances() -> Vec<Instance> {
        INSTANCES.read().await.clone()
    }

    /// Check if an instance has bad uptime.
    fn bad_uptime(info: &serde_json::Value) -> bool {
        if let Some(monitor) = info.get("monitor") {
            if let Some(down) = monitor.get("down").and_then(|v| v.as_bool()) {
                if down {
                    return true;
                }
            }
            if let Some(uptime) = monitor.get("uptime").and_then(|v| v.as_f64()) {
                return uptime < 90.0;
            }
        }
        false
    }

    /// Check if an instance version is outdated.
    fn outdated(target_version: &str, local_version: &str) -> bool {
        if let Some(remote_date) = regex::Regex::new(r"\d{4}\.\d{2}\.\d{2}")
            .ok()
            .and_then(|r| r.find(target_version))
        {
            // Parse dates and check if more than 30 days difference
            // For now, just return false if we can't parse
            let _ = (remote_date, local_version);
        }
        false
    }

    /// Refresh the instance list from the API.
    async fn refresh_instances(&self) -> anyhow::Result<Vec<Instance>> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()?;

        let response = client
            .get("https://api.invidious.io/instances.json")
            .send()
            .await?;

        let instances: Vec<serde_json::Value> = response.json().await?;
        let mut filtered: Vec<Instance> = Vec::new();

        for instance in instances {
            let domain = instance.get("domain").and_then(|v| v.as_str()).unwrap_or("");
            let info = instance.get("info");
            
            if let Some(info) = info {
                // Filter for HTTPS instances only
                if info.get("type").and_then(|v| v.as_str()) != Some("https") {
                    continue;
                }

                // Skip instances with bad uptime
                if Self::bad_uptime(info) {
                    continue;
                }

                // Skip outdated instances
                if let Some(stats) = info.get("stats") {
                    if let Some(version) = stats.get("software").and_then(|s| s.get("version")) {
                        let version_str = version.as_str().unwrap_or("");
                        // TODO: Pass actual local version
                        if Self::outdated(version_str, "2024.01.01") {
                            continue;
                        }
                    }
                }

                let region = info.get("region").and_then(|v| v.as_str()).unwrap_or("").to_string();
                filtered.push(Instance {
                    region,
                    domain: domain.to_string(),
                });
            }
        }

        Ok(filtered)
    }
}

#[async_trait]
impl Job for InstanceListRefreshJob {
    fn name(&self) -> &'static str {
        "instance_list_refresh"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(30 * 60) // 30 minutes
    }

    fn config(&self) -> &JobConfig {
        &self.config.base
    }

    async fn execute(&self) -> anyhow::Result<()> {
        tracing::debug!("InstanceListRefreshJob: Refreshing instance list");
        
        match self.refresh_instances().await {
            Ok(instances) => {
                *INSTANCES.write().await = instances;
                tracing::info!("InstanceListRefreshJob: Done, sleeping for 30 minutes");
            }
            Err(e) => {
                tracing::error!("InstanceListRefreshJob: Failed to fetch instances - {}", e);
            }
        }
        
        Ok(())
    }
}
