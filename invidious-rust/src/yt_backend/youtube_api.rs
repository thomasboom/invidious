//! YouTube API client module.
//!
//! Provides a client for communicating with YouTube's internal APIs.

use reqwest::Client;

use crate::yt_backend::extractors::{
    Video,
    Channel,
    Playlist,
    CommentsResponse,
    SearchResult,
};

const ANDROID_APP_VERSION: &str = "19.35.36";
const ANDROID_VERSION: &str = "13";
const ANDROID_USER_AGENT: &str = "com.google.android.youtube/19.35.36 (Linux; U; Android 13; en_US; SM-S908E Build/TP1A.220624.014) gzip";
const ANDROID_SDK_VERSION: i64 = 33;

const IOS_APP_VERSION: &str = "20.11.6";
const IOS_USER_AGENT: &str = "com.google.ios.youtube/20.11.6 (iPhone14,5; U; CPU iOS 18_5 like Mac OS X;)";
const IOS_VERSION: &str = "18.5.0.22F76";

const WINDOWS_VERSION: &str = "10.0";

/// Client type for YouTube API requests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ClientType {
    #[default]
    Web,
    WebEmbeddedPlayer,
    WebMobile,
    WebScreenEmbed,
    WebCreator,
    Android,
    AndroidEmbeddedPlayer,
    AndroidScreenEmbed,
    AndroidTestSuite,
    IOS,
    IOSEmbedded,
    IOSMusic,
    TvHtml5,
    TvHtml5ScreenEmbed,
    TvSimply,
}

impl ClientType {
    /// Get the client name for the API.
    pub fn name(&self) -> &'static str {
        match self {
            ClientType::Web => "WEB",
            ClientType::WebEmbeddedPlayer => "WEB_EMBEDDED_PLAYER",
            ClientType::WebMobile => "MWEB",
            ClientType::WebScreenEmbed => "WEB",
            ClientType::WebCreator => "WEB_CREATOR",
            ClientType::Android => "ANDROID",
            ClientType::AndroidEmbeddedPlayer => "ANDROID_EMBEDDED_PLAYER",
            ClientType::AndroidScreenEmbed => "ANDROID",
            ClientType::AndroidTestSuite => "ANDROID_TESTSUITE",
            ClientType::IOS => "IOS",
            ClientType::IOSEmbedded => "IOS_MESSAGES_EXTENSION",
            ClientType::IOSMusic => "IOS_MUSIC",
            ClientType::TvHtml5 => "TVHTML5",
            ClientType::TvHtml5ScreenEmbed => "TVHTML5_SIMPLY_EMBEDDED_PLAYER",
            ClientType::TvSimply => "TVHTML5_SIMPLY",
        }
    }

    /// Get the client name for protobuf.
    pub fn name_proto(&self) -> &'static str {
        match self {
            ClientType::Web => "1",
            ClientType::WebEmbeddedPlayer => "56",
            ClientType::WebMobile => "2",
            ClientType::WebScreenEmbed => "1",
            ClientType::WebCreator => "62",
            ClientType::Android => "3",
            ClientType::AndroidEmbeddedPlayer => "55",
            ClientType::AndroidScreenEmbed => "3",
            ClientType::AndroidTestSuite => "30",
            ClientType::IOS => "5",
            ClientType::IOSEmbedded => "66",
            ClientType::IOSMusic => "26",
            ClientType::TvHtml5 => "7",
            ClientType::TvHtml5ScreenEmbed => "85",
            ClientType::TvSimply => "74",
        }
    }

    /// Get the client version.
    pub fn version(&self) -> &'static str {
        match self {
            ClientType::Web => "2.20250222.10.00",
            ClientType::WebEmbeddedPlayer => "1.20250219.01.00",
            ClientType::WebMobile => "2.20250224.01.00",
            ClientType::WebScreenEmbed => "2.20250222.10.00",
            ClientType::WebCreator => "1.20241203.01.00",
            ClientType::Android => ANDROID_APP_VERSION,
            ClientType::AndroidEmbeddedPlayer => ANDROID_APP_VERSION,
            ClientType::AndroidScreenEmbed => ANDROID_APP_VERSION,
            ClientType::AndroidTestSuite => "1.9",
            ClientType::IOS => IOS_APP_VERSION,
            ClientType::IOSEmbedded => IOS_APP_VERSION,
            ClientType::IOSMusic => "7.14",
            ClientType::TvHtml5 => "7.20250219.14.00",
            ClientType::TvHtml5ScreenEmbed => "2.0",
            ClientType::TvSimply => "1.0",
        }
    }

    /// Get the screen type if applicable.
    pub fn screen(&self) -> Option<&'static str> {
        match self {
            ClientType::WebScreenEmbed => Some("EMBED"),
            ClientType::AndroidScreenEmbed => Some("EMBED"),
            ClientType::AndroidEmbeddedPlayer => Some("EMBED"),
            ClientType::TvHtml5ScreenEmbed => Some("EMBED"),
            _ => None,
        }
    }

    /// Get the Android SDK version if applicable.
    pub fn android_sdk_version(&self) -> Option<i64> {
        match self {
            ClientType::Android
            | ClientType::AndroidScreenEmbed
            | ClientType::AndroidTestSuite => Some(ANDROID_SDK_VERSION),
            _ => None,
        }
    }

    /// Get the user agent for this client.
    pub fn user_agent(&self) -> Option<&'static str> {
        match self {
            ClientType::Android
            | ClientType::AndroidScreenEmbed => Some(ANDROID_USER_AGENT),
            ClientType::AndroidTestSuite => Some("com.google.android.youtube/1.9 (Linux; U; Android 12; US) gzip"),
            ClientType::IOS
            | ClientType::IOSEmbedded => Some(IOS_USER_AGENT),
            ClientType::IOSMusic => Some("com.google.ios.youtubemusic/7.14 (iPhone14,5; U; CPU iOS 17_6 like Mac OS X;)"),
            _ => None,
        }
    }

    /// Get the OS name.
    pub fn os_name(&self) -> Option<&'static str> {
        match self {
            ClientType::Web
            | ClientType::WebEmbeddedPlayer
            | ClientType::WebScreenEmbed
            | ClientType::WebCreator => Some("Windows"),
            ClientType::WebMobile
            | ClientType::Android
            | ClientType::AndroidEmbeddedPlayer
            | ClientType::AndroidScreenEmbed
            | ClientType::AndroidTestSuite => Some("Android"),
            ClientType::IOS
            | ClientType::IOSEmbedded
            | ClientType::IOSMusic => Some("iPhone"),
            _ => None,
        }
    }

    /// Get the OS version.
    pub fn os_version(&self) -> Option<&'static str> {
        match self {
            ClientType::Web
            | ClientType::WebEmbeddedPlayer
            | ClientType::WebScreenEmbed
            | ClientType::WebCreator => Some(WINDOWS_VERSION),
            ClientType::WebMobile => Some(ANDROID_VERSION),
            ClientType::Android
            | ClientType::AndroidEmbeddedPlayer
            | ClientType::AndroidScreenEmbed
            | ClientType::AndroidTestSuite => Some(ANDROID_VERSION),
            ClientType::IOS
            | ClientType::IOSEmbedded => Some(IOS_VERSION),
            _ => None,
        }
    }

    /// Get the platform.
    pub fn platform(&self) -> Option<&'static str> {
        match self {
            ClientType::Web
            | ClientType::WebEmbeddedPlayer
            | ClientType::WebScreenEmbed
            | ClientType::WebCreator => Some("DESKTOP"),
            ClientType::WebMobile
            | ClientType::Android
            | ClientType::AndroidEmbeddedPlayer
            | ClientType::AndroidScreenEmbed
            | ClientType::AndroidTestSuite
            | ClientType::IOS
            | ClientType::IOSEmbedded
            | ClientType::IOSMusic => Some("MOBILE"),
            _ => None,
        }
    }

    /// Get the device make.
    pub fn device_make(&self) -> Option<&'static str> {
        match self {
            ClientType::IOS
            | ClientType::IOSEmbedded
            | ClientType::IOSMusic => Some("Apple"),
            _ => None,
        }
    }

    /// Get the device model.
    pub fn device_model(&self) -> Option<&'static str> {
        match self {
            ClientType::IOS
            | ClientType::IOSEmbedded
            | ClientType::IOSMusic => Some("iPhone14,5"),
            _ => None,
        }
    }
}

/// Client configuration for YouTube API requests.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub client_type: ClientType,
    pub region: String,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            client_type: ClientType::Web,
            region: "US".to_string(),
        }
    }
}

impl ClientConfig {
    /// Create a new client configuration.
    pub fn new(client_type: ClientType, region: &str) -> Self {
        Self {
            client_type,
            region: region.to_string(),
        }
    }

    /// Get the client name.
    pub fn name(&self) -> &str {
        self.client_type.name()
    }

    /// Get the client name for protobuf.
    pub fn name_proto(&self) -> &str {
        self.client_type.name_proto()
    }

    /// Get the client version.
    pub fn version(&self) -> &str {
        self.client_type.version()
    }

    /// Build the context for API requests.
    pub fn build_context(&self, video_id: Option<&str>) -> serde_json::Value {
        let mut context = serde_json::json!({
            "client": {
                "hl": "en",
                "gl": self.region,
                "clientName": self.name(),
                "clientVersion": self.version(),
            }
        });

        let client_obj = context["client"].as_object_mut().unwrap();

        if let Some(screen) = self.client_type.screen() {
            client_obj.insert("clientScreen".to_string(), serde_json::Value::String(screen.to_string()));
        }

        if let Some(android_sdk) = self.client_type.android_sdk_version() {
            client_obj.insert("androidSdkVersion".to_string(), serde_json::Value::Number(android_sdk.into()));
        }

        if let Some(device_make) = self.client_type.device_make() {
            client_obj.insert("deviceMake".to_string(), serde_json::Value::String(device_make.to_string()));
        }

        if let Some(device_model) = self.client_type.device_model() {
            client_obj.insert("deviceModel".to_string(), serde_json::Value::String(device_model.to_string()));
        }

        if let Some(os_name) = self.client_type.os_name() {
            client_obj.insert("osName".to_string(), serde_json::Value::String(os_name.to_string()));
        }

        if let Some(os_version) = self.client_type.os_version() {
            client_obj.insert("osVersion".to_string(), serde_json::Value::String(os_version.to_string()));
        }

        if let Some(platform) = self.client_type.platform() {
            client_obj.insert("platform".to_string(), serde_json::Value::String(platform.to_string()));
        }

        if let Some(video_id) = video_id {
            if self.client_type.screen() == Some("EMBED") {
                context["thirdParty"] = serde_json::json!({
                    "embedUrl": format!("https://www.youtube.com/embed/{}", video_id)
                });
            }
        }

        context
    }

    /// Get headers for API requests.
    pub fn headers(&self) -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "application/json; charset=UTF-8".parse().unwrap());
        headers.insert("Accept-Encoding", "gzip, deflate".parse().unwrap());
        headers.insert("x-goog-api-format-version", "2".parse().unwrap());
        headers.insert("x-youtube-client-name", self.name_proto().parse().unwrap());
        headers.insert("x-youtube-client-version", self.version().parse().unwrap());

        if let Some(user_agent) = self.client_type.user_agent() {
            headers.insert("User-Agent", user_agent.parse().unwrap());
        }

        headers
    }
}

/// YouTube API client for making requests to YouTube's internal APIs.
pub struct YoutubeApi {
    client: Client,
    client_config: ClientConfig,
}

impl YoutubeApi {
    /// Create a new YouTube API client with default configuration.
    pub fn new() -> anyhow::Result<Self> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36")
            .build()?;

        Ok(Self {
            client,
            client_config: ClientConfig::default(),
        })
    }

    /// Create a new YouTube API client with custom configuration.
    pub fn with_config(client_config: ClientConfig) -> anyhow::Result<Self> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36".parse().unwrap());
        headers.insert("Accept-Charset", "ISO-8859-1,utf-8;q=0.7,*;q=0.7".parse().unwrap());
        headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap());
        headers.insert("Accept-Language", "en-us,en;q=0.5".parse().unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            client_config,
        })
    }

    /// Set the client configuration.
    pub fn with_client_config(mut self, client_config: ClientConfig) -> Self {
        self.client_config = client_config;
        self
    }

    /// Make a POST request to the YouTube API.
    async fn post_json(
        &self,
        endpoint: &str,
        data: serde_json::Value,
    ) -> anyhow::Result<serde_json::Value> {
        let url = format!("https://www.youtube.com{}?prettyPrint=false", endpoint);

        let response = self.client
            .post(&url)
            .headers(self.client_config.headers())
            .json(&data)
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("YouTube API returned status: {}", response.status());
        }

        let data: serde_json::Value = response.json().await?;

        if let Some(error) = data.get("error") {
            let code = error.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
            let message = error.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
            anyhow::bail!("YouTube API error {}: {}", code, message);
        }

        Ok(data)
    }

    /// Get video details by video ID.
    pub async fn get_video(&self, video_id: &str) -> anyhow::Result<Video> {
        let context = self.client_config.build_context(Some(video_id));
        
        let data = serde_json::json!({
            "context": context,
            "videoId": video_id,
        });

        let response = self.post_json("/youtubei/v1/player", data).await?;
        
        Ok(Video::from_api_response(response, video_id))
    }

    /// Get channel details by channel ID.
    pub async fn get_channel(&self, channel_id: &str) -> anyhow::Result<Channel> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "browseId": channel_id,
        });

        let response = self.post_json("/youtubei/v1/browse", data).await?;
        
        Ok(Channel::from_api_response(response, channel_id))
    }

    /// Search for videos and channels.
    pub async fn search(&self, query: &str, params: Option<&str>) -> anyhow::Result<Vec<SearchResult>> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "query": query,
            "params": params.unwrap_or(""),
        });

        let response = self.post_json("/youtubei/v1/search", data).await?;
        
        Ok(SearchResult::from_search_response(response))
    }

    /// Get trending videos.
    pub async fn get_trending(&self) -> anyhow::Result<Vec<SearchResult>> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "browseId": "FEtrending",
        });

        let response = self.post_json("/youtubei/v1/browse", data).await?;
        
        Ok(SearchResult::from_search_response(response))
    }

    /// Get comments for a video.
    pub async fn get_comments(&self, video_id: &str) -> anyhow::Result<CommentsResponse> {
        let context = self.client_config.build_context(Some(video_id));
        
        let data = serde_json::json!({
            "context": context,
            "videoId": video_id,
        });

        let response = self.post_json("/youtubei/v1/next", data).await?;
        
        Ok(CommentsResponse::from_api_response(response))
    }

    /// Get related videos for a video.
    pub async fn get_related_videos(&self, video_id: &str) -> anyhow::Result<Vec<Video>> {
        let context = self.client_config.build_context(Some(video_id));
        
        let data = serde_json::json!({
            "context": context,
            "videoId": video_id,
        });

        let response = self.post_json("/youtubei/v1/next", data).await?;
        
        Ok(Video::from_related_videos(response))
    }

    /// Get playlist details.
    pub async fn get_playlist(&self, playlist_id: &str) -> anyhow::Result<Playlist> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "browseId": playlist_id,
        });

        let response = self.post_json("/youtubei/v1/browse", data).await?;
        
        Ok(Playlist::from_api_response(response, playlist_id))
    }

    /// Browse endpoint with continuation.
    pub async fn browse_continuation(&self, continuation: &str) -> anyhow::Result<serde_json::Value> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "continuation": continuation,
        });

        self.post_json("/youtubei/v1/browse", data).await
    }

    /// Next endpoint with continuation.
    pub async fn next_continuation(&self, continuation: &str) -> anyhow::Result<serde_json::Value> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "continuation": continuation,
        });

        self.post_json("/youtubei/v1/next", data).await
    }

    /// Resolve a YouTube URL to get the browse ID.
    pub async fn resolve_url(&self, url: &str) -> anyhow::Result<serde_json::Value> {
        let context = self.client_config.build_context(None);
        
        let data = serde_json::json!({
            "context": context,
            "url": url,
        });

        self.post_json("/youtubei/v1/navigation/resolve_url", data).await
    }
}

impl Default for YoutubeApi {
    fn default() -> Self {
        Self::new().expect("Failed to create YoutubeApi")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_type_names() {
        assert_eq!(ClientType::Web.name(), "WEB");
        assert_eq!(ClientType::Android.name(), "ANDROID");
        assert_eq!(ClientType::IOS.name(), "IOS");
    }

    #[test]
    fn test_client_config_default() {
        let config = ClientConfig::default();
        assert_eq!(config.region, "US");
        assert_eq!(config.name(), "WEB");
    }

    #[test]
    fn test_client_config_context() {
        let config = ClientConfig::new(ClientType::Android, "US");
        let context = config.build_context(Some("test_video_id"));
        
        assert_eq!(context["client"]["clientName"], "ANDROID");
        assert_eq!(context["client"]["gl"], "US");
    }
}
