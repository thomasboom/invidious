//! Statistics refresh job.
//!
//! Updates instance statistics.

use crate::jobs::base_job::{Job, JobConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

/// Job configuration for statistics refresh job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsConfig {
    #[serde(flatten)]
    pub base: JobConfig,
}

impl Default for StatisticsConfig {
    fn default() -> Self {
        Self {
            base: JobConfig::default(),
        }
    }
}

/// Instance statistics data structure.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Statistics {
    pub version: String,
    pub software: SoftwareInfo,
    pub open_registrations: bool,
    pub usage: UsageStats,
    pub metadata: MetadataStats,
    pub playback: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SoftwareInfo {
    pub name: String,
    pub version: String,
    pub branch: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UsageStats {
    pub total: i64,
    pub active_halfyear: i64,
    pub active_month: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MetadataStats {
    pub updated_at: i64,
    pub last_channel_refreshed_at: i64,
}

/// Statistics refresh job.
///
/// Updates instance statistics periodically.
pub struct StatisticsRefreshJob {
    config: StatisticsConfig,
    software_name: String,
    software_version: String,
    software_branch: String,
}

impl StatisticsRefreshJob {
    /// Create a new statistics refresh job.
    pub fn new(config: StatisticsConfig, name: String, version: String, branch: String) -> Self {
        Self {
            config,
            software_name: name,
            software_version: version,
            software_branch: branch,
        }
    }

    /// Refresh statistics from database.
    async fn refresh_stats(&self) -> anyhow::Result<Statistics> {
        let mut stats = Statistics {
            version: "2.0".to_string(),
            software: SoftwareInfo {
                name: self.software_name.clone(),
                version: self.software_version.clone(),
                branch: self.software_branch.clone(),
            },
            open_registrations: true,
            ..Default::default()
        };

        // TODO: Query database for actual statistics
        // - Count total users
        // - Count active users (last 6 months)
        // - Count active users (last month)
        // - Get last channel refresh timestamp

        stats.metadata.updated_at = chrono::Utc::now().timestamp();

        Ok(stats)
    }
}

#[async_trait]
impl Job for StatisticsRefreshJob {
    fn name(&self) -> &'static str {
        "statistics"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(10 * 60) // 10 minutes
    }

    fn config(&self) -> &JobConfig {
        &self.config.base
    }

    async fn execute(&self) -> anyhow::Result<()> {
        tracing::debug!("StatisticsRefreshJob: Refreshing statistics");
        
        let _stats = self.refresh_stats().await?;
        
        tracing::debug!("StatisticsRefreshJob: Done");
        Ok(())
    }
}
