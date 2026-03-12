//! Notification job.
//!
//! Handles video notifications for users.

use crate::jobs::base_job::{Job, JobConfig};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio::time::Duration;

/// Video notification structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNotification {
    pub video_id: String,
    pub channel_id: String,
    pub published: i64,
}

/// Job configuration for notification job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationJobConfig {
    #[serde(flatten)]
    pub base: JobConfig,
    #[serde(default = "default_user_notifications")]
    pub enable_user_notifications: bool,
}

fn default_user_notifications() -> bool {
    true
}

impl Default for NotificationJobConfig {
    fn default() -> Self {
        Self {
            base: JobConfig::default(),
            enable_user_notifications: true,
        }
    }
}

/// Notification job.
///
/// Handles video notifications by listening to PostgreSQL notifications
/// and delivering them to users.
pub struct NotificationJob {
    config: NotificationJobConfig,
    notification_tx: mpsc::Sender<VideoNotification>,
}

impl NotificationJob {
    /// Create a new notification job.
    pub fn new(config: NotificationJobConfig, notification_tx: mpsc::Sender<VideoNotification>) -> Self {
        Self { config, notification_tx }
    }

    /// Get the notification channel sender.
    pub fn get_sender(&self) -> mpsc::Sender<VideoNotification> {
        self.notification_tx.clone()
    }
}

#[async_trait]
impl Job for NotificationJob {
    fn name(&self) -> &'static str {
        "notification"
    }

    fn interval(&self) -> Duration {
        Duration::from_secs(60) // 1 minute
    }

    fn config(&self) -> &JobConfig {
        &self.config.base
    }

    async fn execute(&self) -> anyhow::Result<()> {
        tracing::debug!("NotificationJob: Processing notifications");
        
        // TODO: Implement actual notification processing logic
        // - Listen to PostgreSQL NOTIFY for new videos
        // - Cache notifications per channel
        // - Persist to database periodically
        // - Deliver notifications via WebSocket if enabled
        
        tracing::debug!("NotificationJob: Done");
        Ok(())
    }
}
