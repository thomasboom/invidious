//! Clear expired items job.
//!
//! Clears expired videos, nonces, and other cached items.

use crate::jobs::base_job::{Job, JobConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

/// Job configuration for clear expired items job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearExpiredConfig {
    #[serde(flatten)]
    pub base: JobConfig,
}

impl Default for ClearExpiredConfig {
    fn default() -> Self {
        Self {
            base: JobConfig::default(),
        }
    }
}

/// Clear expired items job.
///
/// Removes expired videos, nonces, and other cached items from the database.
pub struct ClearExpiredItemsJob {
    config: ClearExpiredConfig,
}

impl ClearExpiredItemsJob {
    /// Create a new clear expired items job.
    pub fn new(config: ClearExpiredConfig) -> Self {
        Self { config }
    }

    /// Execute the clear expired items logic.
    async fn do_clear(&self) -> anyhow::Result<()> {
        tracing::info!("ClearExpiredItemsJob: Running clear expired items job");
        
        // TODO: Implement actual clear logic
        // - Delete expired videos from cache
        // - Delete expired nonces
        
        tracing::info!("ClearExpiredItemsJob: Done");
        Ok(())
    }
}

#[async_trait]
impl Job for ClearExpiredItemsJob {
    fn name(&self) -> &'static str {
        "clear_expired"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(60 * 60) // 1 hour
    }

    fn config(&self) -> &JobConfig {
        &self.config.base
    }

    async fn execute(&self) -> anyhow::Result<()> {
        match self.do_clear().await {
            Ok(_) => {
                tracing::debug!("ClearExpiredItemsJob: Completed successfully");
            }
            Err(e) => {
                tracing::error!("ClearExpiredItemsJob: Failed - {}", e);
                // Retry sooner on failure
                tokio::time::sleep(Duration::from_secs(10 * 60)).await;
            }
        }
        Ok(())
    }
}
