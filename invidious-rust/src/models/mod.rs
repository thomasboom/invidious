//! Data models for Invidious.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A YouTube video.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub published: DateTime<Utc>,
    pub author: String,
    pub author_id: String,
    pub author_url: String,
    pub video_thumbnails: Vec<Thumbnail>,
    pub length_seconds: i64,
    pub view_count: i64,
    pub like_count: Option<i64>,
    pub dislike_count: Option<i64>,
    pub live_now: bool,
    pub paid: bool,
    pub premium: bool,
    pub is_upcoming: bool,
}

/// Thumbnail information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: Option<i32>,
    pub height: Option<i32>,
}

/// A YouTube channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub author_thumbnails: Vec<Thumbnail>,
    pub banner_thumbnails: Vec<Thumbnail>,
    pub subscriber_count: i64,
    pub video_count: i64,
    pub is_verified: bool,
}

/// A YouTube playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub author_id: String,
    pub video_count: i64,
    pub videos: Vec<PlaylistVideo>,
}

/// A video in a playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistVideo {
    pub title: String,
    pub length_seconds: i64,
    pub video_id: String,
    pub author: String,
    pub author_id: String,
    pub video_thumbnails: Vec<Thumbnail>,
}

/// User account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub email: Option<String>,
    pub username: String,
    pub preferences: crate::config::ConfigPreferences,
    pub notifications: Vec<Notification>,
    pub subscription_ids: Vec<String>,
    pub watched_playlist: Vec<String>,
}

/// User notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub video_id: String,
    pub read: bool,
}
