//! Background jobs module.
//!
//! Handles background tasks like subscription updates and feed refreshes.

use tokio::time::{interval, Duration};

/// Job runner for background tasks.
#[allow(dead_code)]
pub struct JobRunner {
    channel_threads: i32,
    feed_threads: i32,
    channel_refresh_interval: Duration,
}

impl JobRunner {
    /// Create a new job runner.
    pub fn new(
        channel_threads: i32,
        feed_threads: i32,
        refresh_interval: &str,
    ) -> anyhow::Result<Self> {
        let duration = parse_duration(refresh_interval)?;

        Ok(Self {
            channel_threads,
            feed_threads,
            channel_refresh_interval: duration,
        })
    }

    /// Start the background jobs.
    pub async fn start(&self) {
        let mut ticker = interval(self.channel_refresh_interval);
        
        loop {
            ticker.tick().await;
            tracing::info!("Running background jobs...");
        }
    }
}

/// Parse duration string like "30m" into Duration.
fn parse_duration(s: &str) -> anyhow::Result<Duration> {
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
