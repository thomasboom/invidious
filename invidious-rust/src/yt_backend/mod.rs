//! YouTube backend module.
//!
//! Handles communication with YouTube's APIs.

use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Client for YouTube backend communication.
#[allow(dead_code)]
pub struct YouTubeBackend {
    client: Client,
    inner_api_url: String,
}

impl YouTubeBackend {
    /// Create a new YouTube backend client.
    pub fn new() -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent("Invidious Rust/0.1.0")
            .build()?;

        Ok(Self {
            client,
            inner_api_url: "https://www.youtube.com/browse_ajax".to_string(),
        })
    }

    /// Get video information.
    pub async fn get_video(&self, video_id: &str) -> anyhow::Result<VideoResponse> {
        let url = format!(
            "https://www.youtube.com/watch?v={}&pbj=1",
            video_id
        );

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        
        Ok(VideoResponse {
            data,
        })
    }

    /// Search for videos.
    pub async fn search(&self, query: &str) -> anyhow::Result<SearchResponse> {
        let url = "https://www.youtube.com/results";
        
        let response = self.client
            .get(url)
            .query(&[("search_query", query)])
            .send()
            .await?;

        let body = response.text().await?;
        
        Ok(SearchResponse {
            html: body,
        })
    }
}

impl Default for YouTubeBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create YouTubeBackend")
    }
}

/// Video response from YouTube API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoResponse {
    pub data: serde_json::Value,
}

/// Search response from YouTube.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub html: String,
}
