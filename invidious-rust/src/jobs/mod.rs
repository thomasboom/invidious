//! Background jobs module.
//!
//! Handles background tasks like subscription updates and feed refreshes.

mod base_job;
mod clear_expired;
mod instance_list_refresh;
mod notification;
mod pull_popular_videos;
mod refresh_channels;
mod refresh_feeds;
mod statistics;
mod subscribe_to_feeds;

pub use base_job::{Job, JobConfig};
pub use clear_expired::ClearExpiredItemsJob;
pub use instance_list_refresh::InstanceListRefreshJob;
pub use notification::NotificationJob;
pub use pull_popular_videos::PullPopularVideosJob;
pub use refresh_channels::RefreshChannelsJob;
pub use refresh_feeds::RefreshFeedsJob;
pub use statistics::StatisticsRefreshJob;
pub use subscribe_to_feeds::SubscribeToFeedsJob;

use std::sync::Arc;
use std::sync::LazyLock;
use tokio::sync::RwLock;
use tokio::time::Duration;

/// Registry for managing all background jobs.
pub struct JobRegistry {
    jobs: RwLock<Vec<Arc<dyn Job>>>,
}

impl JobRegistry {
    /// Create a new empty job registry.
    pub fn new() -> Self {
        Self {
            jobs: RwLock::new(Vec::new()),
        }
    }

    /// Register a new job with the registry.
    pub async fn register(&self, job: Arc<dyn Job>) {
        self.jobs.write().await.push(job);
    }

    /// Start all registered jobs.
    pub async fn start_all(&self) {
        let jobs = self.jobs.read().await;
        for job in jobs.iter() {
            let name = job.name();
            tracing::info!("Starting background job: {}", name);
            let job = Arc::clone(job);
            tokio::spawn(async move {
                if let Err(e) = job.run().await {
                    tracing::error!("Job {} failed: {}", name, e);
                }
            });
        }
    }
}

impl Default for JobRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Global job registry instance.
pub static JOB_REGISTRY: LazyLock<JobRegistry> = LazyLock::new(JobRegistry::new);

/// Parse duration string like "30m" into Duration.
pub fn parse_duration(s: &str) -> anyhow::Result<Duration> {
    if s.ends_with('m') {
        let mins: u64 = s.trim_end_matches('m').parse()?;
        Ok(Duration::from_secs(mins * 60))
    } else if s.ends_with('h') {
        let hours: u64 = s.trim_end_matches('h').parse()?;
        Ok(Duration::from_secs(hours * 3600))
    } else if s.ends_with('s') {
        let secs: u64 = s.trim_end_matches('s').parse()?;
        Ok(Duration::from_secs(secs))
    } else {
        Ok(Duration::from_secs(s.parse()?))
    }
}
