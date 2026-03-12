//! Base job trait definition.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::time::{interval, Duration};

/// Configuration for a background job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobConfig {
    /// Whether the job is enabled.
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

impl Default for JobConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Trait for background jobs.
///
/// Implement this trait to create a new background job.
#[async_trait]
pub trait Job: Send + Sync {
    /// Return the name of the job.
    fn name(&self) -> &'static str;

    /// Return the interval at which the job should run.
    fn interval(&self) -> Duration;

    /// Return the job configuration.
    fn config(&self) -> &JobConfig;

    /// Check if the job is enabled.
    fn is_enabled(&self) -> bool {
        self.config().enabled
    }

    /// Run the job continuously until shutdown.
    async fn run(&self) -> anyhow::Result<()> {
        if !self.is_enabled() {
            tracing::debug!("Job {} is disabled, skipping", self.name());
            return Ok(());
        }

        let mut ticker = interval(self.interval());
        
        loop {
            ticker.tick().await;
            tracing::debug!("Running job: {}", self.name());
            
            if let Err(e) = self.execute().await {
                tracing::error!("Job {} error: {}", self.name(), e);
            }
        }
    }

    /// Execute a single iteration of the job.
    async fn execute(&self) -> anyhow::Result<()>;
}
