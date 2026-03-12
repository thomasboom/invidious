//! Pull popular videos job.
//!
//! Pulls and caches popular videos.

use crate::jobs::base_job::{Job, JobConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use tokio::time::Duration;

static POPULAR_VIDEOS: OnceLock<Vec<serde_json::Value>> = OnceLock::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Job configuration for pull popular videos job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullPopularVideosConfig {
    #[serde(flatten)]
    pub base: JobConfig,
}

impl Default for PullPopularVideosConfig {
    fn default() -> Self {
        Self {
            base: JobConfig::default(),
        }
    }
}

/// Pull popular videos job.
///
/// Pulls and caches popular videos from the database.
pub struct PullPopularVideosJob {
    config: PullPopularVideosConfig,
}

impl PullPopularVideosJob {
    /// Create a new pull popular videos job.
    pub fn new(config: PullPopularVideosConfig) -> Self {
        Self { config }
    }

    /// Get the cached popular videos.
    pub fn get_popular_videos() -> Option<&'static Vec<serde_json::Value>> {
        POPULAR_VIDEOS.get()
    }
}

#[async_trait]
impl Job for PullPopularVideosJob {
    fn name(&self) -> &'static str {
        "pull_popular_videos"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(60) // 1 minute
    }

    fn config(&self) -> &JobConfig {
        &self.config.base
    }

    async fn execute(&self) -> anyhow::Result<()> {
        tracing::debug!("PullPopularVideosJob: Pulling popular videos");
        
        // TODO: Implement actual popular videos pull logic
        // - Query database for popular videos
        // - Sort by published date (newest first)
        // - Cache in POPULAR_VIDEOS
        
        // Initialize with empty array for now
        if !INITIALIZED.load(Ordering::SeqCst) {
            let _ = POPULAR_VIDEOS.set(Vec::new());
            INITIALIZED.store(true, Ordering::SeqCst);
        }
        
        tracing::debug!("PullPopularVideosJob: Done");
        Ok(())
    }
}
