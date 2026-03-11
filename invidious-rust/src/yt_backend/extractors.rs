//! Data extraction module for YouTube API responses.
//!
//! Provides structures and functions to parse YouTube API responses into
//! usable data types.

use serde::{Deserialize, Serialize};

/// Video badge types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VideoBadges(u32);

impl VideoBadges {
    pub const NONE: Self = Self(0);
    pub const LIVE_NOW: Self = Self(1 << 0);
    pub const NEW: Self = Self(1 << 1);
    pub const FOUR_K: Self = Self(1 << 2);
    pub const EIGHT_K: Self = Self(1 << 3);
    pub const VR180: Self = Self(1 << 4);
    pub const VR360: Self = Self(1 << 5);
    pub const THREE_D: Self = Self(1 << 6);
    pub const CLOSED_CAPTIONS: Self = Self(1 << 7);
    pub const PREMIUM: Self = Self(1 << 8);

    pub fn has_live_now(&self) -> bool {
        self.0 & Self::LIVE_NOW.0 != 0
    }

    pub fn has_new(&self) -> bool {
        self.0 & Self::NEW.0 != 0
    }

    pub fn has_premium(&self) -> bool {
        self.0 & Self::PREMIUM.0 != 0
    }
}

/// Video data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub author: String,
    pub author_id: String,
    pub author_thumbnail: Option<String>,
    pub published: Option<String>,
    pub views: i64,
    pub length_seconds: i32,
    pub description_html: String,
    pub thumbnails: Vec<Thumbnail>,
    pub badges: VideoBadges,
    pub premiere_timestamp: Option<i64>,
    pub live_now: bool,
    pub author_verified: bool,
}

impl Video {
    /// Create a Video from the player API response.
    pub fn from_api_response(response: serde_json::Value, video_id: &str) -> Self {
        let video_details = response.get("videoDetails");
        let _microformat = response
            .get("microformat")
            .and_then(|m| m.get("playerMicroformatRenderer"));

        let title = video_details
            .and_then(|v| v.get("title"))
            .and_then(|t| t.get("simpleText"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let author = video_details
            .and_then(|v| v.get("author"))
            .and_then(|a| a.get("simpleText"))
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .to_string();

        let author_id = video_details
            .and_then(|v| v.get("channelId"))
            .and_then(|c| c.as_str())
            .unwrap_or("")
            .to_string();

        let views = video_details
            .and_then(|v| v.get("viewCount"))
            .and_then(|v| v.as_str())
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);

        let length_seconds = video_details
            .and_then(|v| v.get("lengthSeconds"))
            .and_then(|l| l.as_str())
            .and_then(|l| l.parse().ok())
            .unwrap_or(0);

        let description = video_details
            .and_then(|v| v.get("shortDescription"))
            .and_then(|d| d.as_str())
            .unwrap_or("")
            .to_string();

        let thumbnails = video_details
            .and_then(|v| v.get("thumbnail"))
            .and_then(|t| t.get("thumbnails"))
            .and_then(|t| t.as_array())
            .map(|thumbs| {
                thumbs
                    .iter()
                    .filter_map(|thumb| {
                        Some(Thumbnail {
                            url: thumb.get("url")?.as_str()?.to_string(),
                            width: thumb.get("width")?.as_i64().unwrap_or(0) as u32,
                            height: thumb.get("height")?.as_i64().unwrap_or(0) as u32,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let author_thumbnail = video_details
            .and_then(|v| v.get("authorThumbnail"))
            .and_then(|t| t.get("thumbnails"))
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let live_now = video_details
            .and_then(|v| v.get("isLive"))
            .and_then(|l| l.as_bool())
            .unwrap_or(false);

        let badges = VideoBadges::default();

        Self {
            id: video_id.to_string(),
            title,
            author,
            author_id,
            author_thumbnail,
            published: None,
            views,
            length_seconds,
            description_html: description,
            thumbnails,
            badges,
            premiere_timestamp: None,
            live_now,
            author_verified: false,
        }
    }

    /// Extract related videos from the next API response.
    pub fn from_related_videos(response: serde_json::Value) -> Vec<Self> {
        let mut videos = Vec::new();

        if let Some(contents) = response
            .get("contents")
            .and_then(|c| c.get("twoColumnWatchNextResults"))
            .and_then(|c| c.get("secondaryResults"))
            .and_then(|s| s.get("secondaryResults"))
            .and_then(|s| s.get("results"))
            .and_then(|r| r.as_array())
        {
            for item in contents {
                if let Some(video_renderer) = item.get("videoRenderer") {
                    if let Some(id) = video_renderer.get("videoId").and_then(|v| v.as_str()) {
                        let title = video_renderer
                            .get("title")
                            .and_then(|t| t.get("simpleText"))
                            .or_else(|| video_renderer.get("title").and_then(|t| t.get("runs")))
                            .and_then(|r| r.as_array())
                            .and_then(|r| r.first())
                            .and_then(|r| r.get("text"))
                            .and_then(|t| t.as_str())
                            .unwrap_or("")
                            .to_string();

                        let author = video_renderer
                            .get("shortBylineText")
                            .and_then(|b| b.get("runs"))
                            .and_then(|r| r.as_array())
                            .and_then(|r| r.first())
                            .and_then(|r| r.get("text"))
                            .and_then(|a| a.as_str())
                            .unwrap_or("")
                            .to_string();

                        let author_id = video_renderer
                            .get("shortBylineText")
                            .and_then(|b| b.get("runs"))
                            .and_then(|r| r.as_array())
                            .and_then(|r| r.first())
                            .and_then(|r| r.get("navigationEndpoint"))
                            .and_then(|n| n.get("browseEndpoint"))
                            .and_then(|b| b.get("browseId"))
                            .and_then(|b| b.as_str())
                            .unwrap_or("")
                            .to_string();

                        let length_seconds = video_renderer
                            .get("lengthText")
                            .and_then(|l| l.get("simpleText"))
                            .and_then(|l| l.as_str())
                            .and_then(|l| parse_length_seconds(l))
                            .unwrap_or(0);

                        let thumbnails = video_renderer
                            .get("thumbnail")
                            .and_then(|t| t.get("thumbnails"))
                            .and_then(|t| t.as_array())
                            .map(|thumbs| {
                                thumbs
                                    .iter()
                                    .filter_map(|thumb| {
                                        Some(Thumbnail {
                                            url: thumb.get("url")?.as_str()?.to_string(),
                                            width: thumb.get("width")?.as_i64().unwrap_or(0) as u32,
                                            height: thumb.get("height")?.as_i64().unwrap_or(0)
                                                as u32,
                                        })
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        videos.push(Self {
                            id: id.to_string(),
                            title,
                            author,
                            author_id,
                            author_thumbnail: None,
                            published: None,
                            views: 0,
                            length_seconds,
                            description_html: String::new(),
                            thumbnails,
                            badges: VideoBadges::default(),
                            premiere_timestamp: None,
                            live_now: false,
                            author_verified: false,
                        });
                    }
                }
            }
        }

        videos
    }
}

/// Thumbnail structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

/// Channel data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub name: String,
    pub handle: Option<String>,
    pub thumbnail: Option<String>,
    pub subscriber_count: Option<i64>,
    pub video_count: Option<i64>,
    pub description_html: String,
    pub author_verified: bool,
    pub auto_generated: bool,
}

impl Channel {
    /// Create a Channel from the browse API response.
    pub fn from_api_response(response: serde_json::Value, channel_id: &str) -> Self {
        let metadata = response
            .get("metadata")
            .and_then(|m| m.get("channelMetadataRenderer"));

        let name = metadata
            .and_then(|m| m.get("title"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let handle = metadata
            .and_then(|m| m.get("handle"))
            .and_then(|h| h.as_str())
            .map(|s| s.to_string());

        let thumbnail = metadata
            .and_then(|m| m.get("avatar"))
            .and_then(|a| a.get("thumbnails"))
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let contents = response
            .get("contents")
            .and_then(|c| c.get("twoColumnBrowseResultsRenderer"))
            .and_then(|c| c.get("tabs"))
            .and_then(|t| t.as_array());

        let subscriber_count = contents
            .and_then(|tabs| tabs.first())
            .and_then(|tab| tab.get("tabRenderer"))
            .and_then(|tab| tab.get("content"))
            .and_then(|c| c.get("sectionListRenderer"))
            .and_then(|s| s.get("contents"))
            .and_then(|c| c.as_array())
            .and_then(|c| c.first())
            .and_then(|item| item.get("itemSectionRenderer"))
            .and_then(|s| s.get("contents"))
            .and_then(|c| c.as_array())
            .and_then(|c| c.first())
            .and_then(|c| c.get("channelHeaderLinksRenderer"))
            .and_then(|l| l.get("subscriberCount"))
            .and_then(|s| s.as_str())
            .and_then(|s| parse_count(s));

        Self {
            id: channel_id.to_string(),
            name,
            handle,
            thumbnail,
            subscriber_count,
            video_count: None,
            description_html: String::new(),
            author_verified: false,
            auto_generated: false,
        }
    }
}

/// Playlist data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub author: String,
    pub author_id: String,
    pub video_count: i64,
    pub thumbnail: Option<String>,
    pub videos: Vec<PlaylistVideo>,
}

impl Playlist {
    /// Create a Playlist from the browse API response.
    pub fn from_api_response(response: serde_json::Value, playlist_id: &str) -> Self {
        let metadata = response
            .get("metadata")
            .and_then(|m| m.get("playlistMetadataRenderer"));

        let title = metadata
            .and_then(|m| m.get("title"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let contents = response
            .get("contents")
            .and_then(|c| c.get("twoColumnBrowseResultsRenderer"))
            .and_then(|c| c.get("tabs"))
            .and_then(|t| t.as_array())
            .and_then(|tabs| tabs.first())
            .and_then(|tab| tab.get("tabRenderer"))
            .and_then(|tab| tab.get("content"))
            .and_then(|c| c.get("sectionListRenderer"))
            .and_then(|s| s.get("contents"))
            .and_then(|c| c.as_array())
            .and_then(|c| c.first())
            .and_then(|item| item.get("itemSectionRenderer"))
            .and_then(|s| s.get("contents"))
            .and_then(|c| c.as_array())
            .and_then(|c| c.first())
            .and_then(|c| c.get("playlistVideoListRenderer"));

        let videos: Vec<PlaylistVideo> = contents
            .and_then(|l| l.get("contents"))
            .and_then(|c| c.as_array())
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| {
                        let renderer = item.get("playlistVideoRenderer")?;
                        let id = renderer.get("videoId")?.as_str()?.to_string();
                        let title = renderer
                            .get("title")
                            .and_then(|t| t.get("simpleText"))
                            .or_else(|| renderer.get("title").and_then(|t| t.get("runs")))
                            .and_then(|r| r.as_array())
                            .and_then(|r| r.first())
                            .and_then(|r| r.get("text"))
                            .and_then(|t| t.as_str())
                            .unwrap_or("")
                            .to_string();
                        let length_seconds = renderer
                            .get("lengthText")
                            .and_then(|l| l.get("simpleText"))
                            .and_then(|l| l.as_str())
                            .and_then(|l| parse_length_seconds(l))
                            .unwrap_or(0);

                        Some(PlaylistVideo {
                            id,
                            title,
                            length_seconds,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let video_count = videos.len() as i64;

        Self {
            id: playlist_id.to_string(),
            title,
            author: String::new(),
            author_id: String::new(),
            video_count,
            thumbnail: None,
            videos,
        }
    }
}

/// Video within a playlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistVideo {
    pub id: String,
    pub title: String,
    pub length_seconds: i32,
}

/// Comment data structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub author_thumbnail: Option<String>,
    pub author_id: String,
    pub content: String,
    pub published: String,
    pub like_count: i64,
    pub is_verified: bool,
    pub is_verified_author: bool,
    pub replies: Vec<Comment>,
}

impl Comment {
    fn from_comments_section(response: serde_json::Value) -> Vec<Self> {
        let mut comments = Vec::new();

        if let Some(contents) = response
            .get("contents")
            .and_then(|c| c.get("twoColumnWatchNextResults"))
            .and_then(|c| c.get("results"))
            .and_then(|r| r.get("results"))
            .and_then(|r| r.get("comments"))
            .and_then(|c| c.get("header"))
            .and_then(|c| c.get("commentsHeaderRenderer"))
            .and_then(|c| c.get("commentSort"))
            .and_then(|s| s.get("sortFilterMenuRenderer"))
            .and_then(|s| s.get("comments"))
            .and_then(|c| c.as_array())
        {
            for item in contents {
                if let Some(comment_thread) = item.get("commentThreadRenderer") {
                    if let Some(comment) = Self::parse_comment_renderer(
                        comment_thread
                            .get("comment")
                            .and_then(|c| c.get("commentRenderer")),
                    ) {
                        let replies = comment_thread
                            .get("replies")
                            .and_then(|r| r.get("commentRepliesRenderer"))
                            .and_then(|r| r.get("comments"))
                            .and_then(|c| c.as_array())
                            .map(|replies| {
                                replies
                                    .iter()
                                    .filter_map(|reply| {
                                        Self::parse_comment_renderer(reply.get("commentRenderer"))
                                    })
                                    .collect()
                            })
                            .unwrap_or_default();

                        comments.push(Self { replies, ..comment });
                    }
                }
            }
        }

        comments
    }

    fn parse_comment_renderer(renderer: Option<&serde_json::Value>) -> Option<Self> {
        let renderer = renderer?;

        let id = renderer.get("commentId")?.as_str()?.to_string();

        let author = renderer
            .get("authorText")
            .and_then(|a| a.get("simpleText"))
            .or_else(|| renderer.get("authorText").and_then(|a| a.get("runs")))
            .and_then(|r| r.as_array())
            .and_then(|r| r.first())
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let author_thumbnail = renderer
            .get("authorThumbnail")
            .and_then(|a| a.get("thumbnails"))
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let author_id = renderer
            .get("authorEndpoint")
            .and_then(|a| a.get("browseEndpoint"))
            .and_then(|b| b.get("browseId"))
            .and_then(|b| b.as_str())
            .unwrap_or("")
            .to_string();

        let content = renderer
            .get("contentText")
            .and_then(|c| c.get("runs"))
            .and_then(|r| r.as_array())
            .map(|runs| {
                runs.iter()
                    .filter_map(|r| r.get("text").and_then(|t| t.as_str()))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .unwrap_or_default();

        let published = renderer
            .get("publishedTimeText")
            .and_then(|p| p.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("publishedTimeText")
                    .and_then(|p| p.get("runs"))
            })
            .and_then(|r| r.as_array())
            .and_then(|r| r.first())
            .and_then(|r| r.get("text"))
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let like_count = renderer
            .get("voteCount")
            .and_then(|v| v.get("simpleText"))
            .and_then(|t| t.as_str())
            .and_then(|t| parse_count(t))
            .unwrap_or(0);

        let is_verified = renderer.get("authorCommentBadge").is_some();

        Some(Self {
            id,
            author,
            author_thumbnail,
            author_id,
            content,
            published,
            like_count,
            is_verified,
            is_verified_author: false,
            replies: Vec::new(),
        })
    }
}

/// Comments response containing comments and continuation token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsResponse {
    pub comments: Vec<Comment>,
    pub continuation: Option<String>,
}

impl CommentsResponse {
    /// Create from the next API response.
    pub fn from_api_response(response: serde_json::Value) -> Self {
        let comments = Comment::from_comments_section(response.clone());

        let continuation = response
            .get("contents")
            .and_then(|c| c.get("twoColumnWatchNextResults"))
            .and_then(|c| c.get("secondaryResults"))
            .and_then(|s| s.get("secondaryResults"))
            .and_then(|r| r.get("continuations"))
            .and_then(|c| c.as_array())
            .and_then(|c| c.first())
            .and_then(|c| c.get("nextContinuationData"))
            .and_then(|d| d.get("continuation"))
            .and_then(|c| c.as_str())
            .map(|s| s.to_string());

        Self {
            comments,
            continuation,
        }
    }
}

/// Search result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SearchResult {
    Video(Video),
    Channel(Channel),
    Playlist(Playlist),
    Hashtag(Hashtag),
}

impl SearchResult {
    /// Parse search results from the search API response.
    pub fn from_search_response(response: serde_json::Value) -> Vec<Self> {
        let mut results = Vec::new();

        if let Some(contents) = response
            .get("contents")
            .and_then(|c| c.get("twoColumnSearchResultsRenderer"))
            .and_then(|c| c.get("primaryContents"))
            .and_then(|p| p.get("sectionListRenderer"))
            .and_then(|s| s.get("contents"))
            .and_then(|c| c.as_array())
        {
            for item in contents {
                if let Some(item_section) = item.get("itemSectionRenderer") {
                    if let Some(items) = item_section.get("contents").and_then(|c| c.as_array()) {
                        for item in items {
                            if let Some(result) = Self::parse_item(item) {
                                results.push(result);
                            }
                        }
                    }
                }
            }
        }

        results
    }

    fn parse_item(item: &serde_json::Value) -> Option<Self> {
        if let Some(video_renderer) = item.get("videoRenderer") {
            return Some(SearchResult::Video(Video::from_video_renderer(
                video_renderer,
            )?));
        }

        if let Some(channel_renderer) = item.get("channelRenderer") {
            return Some(SearchResult::Channel(Channel::from_channel_renderer(
                channel_renderer,
            )?));
        }

        if let Some(playlist_renderer) = item.get("playlistRenderer") {
            return Some(SearchResult::Playlist(Playlist::from_playlist_renderer(
                playlist_renderer,
            )?));
        }

        if let Some(hashtag_renderer) = item.get("hashtagTileRenderer") {
            return Some(SearchResult::Hashtag(Hashtag::from_hashtag_renderer(
                hashtag_renderer,
            )?));
        }

        None
    }
}

impl Video {
    fn from_video_renderer(renderer: &serde_json::Value) -> Option<Self> {
        let id = renderer.get("videoId")?.as_str()?.to_string();

        let title = renderer
            .get("title")
            .and_then(|t| t.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("title")
                    .and_then(|t| t.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("text"))
            })
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let author = renderer
            .get("ownerText")
            .and_then(|o| o.get("runs"))
            .and_then(|r| r.as_array())
            .and_then(|r| r.first())
            .and_then(|r| r.get("text"))
            .and_then(|a| a.as_str())
            .or_else(|| {
                renderer
                    .get("shortBylineText")
                    .and_then(|o| o.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("text"))
                    .and_then(|a| a.as_str())
            })
            .unwrap_or("")
            .to_string();

        let author_id = renderer
            .get("ownerText")
            .and_then(|o| o.get("runs"))
            .and_then(|r| r.as_array())
            .and_then(|r| r.first())
            .and_then(|r| r.get("navigationEndpoint"))
            .and_then(|n| n.get("browseEndpoint"))
            .and_then(|b| b.get("browseId"))
            .or_else(|| {
                renderer
                    .get("shortBylineText")
                    .and_then(|o| o.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("navigationEndpoint"))
                    .and_then(|n| n.get("browseEndpoint"))
                    .and_then(|b| b.get("browseId"))
            })
            .and_then(|b| b.as_str())
            .unwrap_or("")
            .to_string();

        let views = renderer
            .get("viewCountText")
            .and_then(|v| v.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("viewCountText")
                    .and_then(|v| v.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("text"))
            })
            .and_then(|v| v.as_str())
            .and_then(|v| {
                v.replace(",", "")
                    .replace(" views", "")
                    .replace(" view", "")
                    .parse()
                    .ok()
            })
            .unwrap_or(0);

        let length_seconds = renderer
            .get("lengthText")
            .and_then(|l| l.get("simpleText"))
            .and_then(|l| l.as_str())
            .and_then(|l| parse_length_seconds(l))
            .unwrap_or(0);

        let thumbnails = renderer
            .get("thumbnail")
            .and_then(|t| t.get("thumbnails"))
            .and_then(|t| t.as_array())
            .map(|thumbs| {
                thumbs
                    .iter()
                    .filter_map(|thumb| {
                        Some(Thumbnail {
                            url: thumb.get("url")?.as_str()?.to_string(),
                            width: thumb.get("width")?.as_i64().unwrap_or(0) as u32,
                            height: thumb.get("height")?.as_i64().unwrap_or(0) as u32,
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let author_thumbnail = renderer
            .get("channelThumbnailSupportedRenderers")
            .and_then(|c| c.get("channelThumbnailWithLinkRenderer"))
            .and_then(|c| c.get("thumbnail"))
            .and_then(|t| t.get("thumbnails"))
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let live_now = renderer
            .get("badges")
            .and_then(|b| b.as_array())
            .map(|badges| {
                badges.iter().any(|b| {
                    b.get("metadataBadgeRenderer")
                        .and_then(|m| m.get("label"))
                        .and_then(|l| l.as_str())
                        .map(|l| l == "LIVE")
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);

        Some(Self {
            id,
            title,
            author,
            author_id,
            author_thumbnail,
            published: None,
            views,
            length_seconds,
            description_html: String::new(),
            thumbnails,
            badges: VideoBadges::default(),
            premiere_timestamp: None,
            live_now,
            author_verified: false,
        })
    }
}

impl Channel {
    fn from_channel_renderer(renderer: &serde_json::Value) -> Option<Self> {
        let id = renderer.get("channelId")?.as_str()?.to_string();

        let name = renderer
            .get("title")
            .and_then(|t| t.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("title")
                    .and_then(|t| t.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("text"))
            })
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let handle = renderer
            .get("subscriberCountText")
            .and_then(|s| s.get("simpleText"))
            .and_then(|t| t.as_str())
            .and_then(|t| {
                if t.starts_with('@') {
                    Some(t.to_string())
                } else {
                    None
                }
            });

        let subscriber_count = renderer
            .get("subscriberCountText")
            .and_then(|s| s.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("videoCountText")
                    .and_then(|s| s.get("simpleText"))
            })
            .and_then(|t| t.as_str())
            .and_then(|t| parse_count(t));

        let thumbnail = renderer
            .get("thumbnail")
            .and_then(|t| t.get("thumbnails"))
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        let video_count = renderer
            .get("videoCountText")
            .and_then(|v| v.get("simpleText"))
            .and_then(|t| t.as_str())
            .and_then(|t| parse_count(t));

        Some(Self {
            id,
            name,
            handle,
            thumbnail,
            subscriber_count,
            video_count,
            description_html: String::new(),
            author_verified: false,
            auto_generated: false,
        })
    }
}

impl Playlist {
    fn from_playlist_renderer(renderer: &serde_json::Value) -> Option<Self> {
        let id = renderer.get("playlistId")?.as_str()?.to_string();

        let title = renderer
            .get("title")
            .and_then(|t| t.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("title")
                    .and_then(|t| t.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("text"))
            })
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let author = renderer
            .get("shortBylineText")
            .and_then(|s| s.get("runs"))
            .and_then(|r| r.as_array())
            .and_then(|r| r.first())
            .and_then(|r| r.get("text"))
            .and_then(|a| a.as_str())
            .unwrap_or("")
            .to_string();

        let author_id = renderer
            .get("shortBylineText")
            .and_then(|s| s.get("runs"))
            .and_then(|r| r.as_array())
            .and_then(|r| r.first())
            .and_then(|r| r.get("navigationEndpoint"))
            .and_then(|n| n.get("browseEndpoint"))
            .and_then(|b| b.get("browseId"))
            .and_then(|b| b.as_str())
            .unwrap_or("")
            .to_string();

        let video_count = renderer
            .get("videoCount")
            .and_then(|v| v.as_str())
            .or_else(|| {
                renderer
                    .get("videoCountText")
                    .and_then(|v| v.get("simpleText"))
                    .and_then(|t| t.as_str())
            })
            .and_then(|t| parse_count(t))
            .unwrap_or(0);

        let thumbnail = renderer
            .get("thumbnails")
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("thumbnails"))
            .and_then(|t| t.as_array())
            .and_then(|t| t.first())
            .and_then(|t| t.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string());

        Some(Self {
            id,
            title,
            author,
            author_id,
            video_count,
            thumbnail,
            videos: Vec::new(),
        })
    }
}

/// Hashtag search result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hashtag {
    pub tag: String,
    pub url: String,
    pub video_count: i64,
}

impl Hashtag {
    fn from_hashtag_renderer(renderer: &serde_json::Value) -> Option<Self> {
        let tag = renderer
            .get("hashtag")
            .and_then(|h| h.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("hashtag")
                    .and_then(|h| h.get("runs"))
                    .and_then(|r| r.as_array())
                    .and_then(|r| r.first())
                    .and_then(|r| r.get("text"))
            })
            .and_then(|t| t.as_str())
            .unwrap_or("")
            .to_string();

        let url = renderer
            .get("onTapCommand")
            .and_then(|o| o.get("commandMetadata"))
            .and_then(|c| c.get("webCommandMetadata"))
            .and_then(|w| w.get("url"))
            .and_then(|u| u.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("/hashtag/{}", tag.trim_start_matches('#')));

        let video_count = renderer
            .get("hashtagVideoCount")
            .and_then(|v| v.get("simpleText"))
            .or_else(|| {
                renderer
                    .get("hashtagInfoText")
                    .and_then(|i| i.get("simpleText"))
            })
            .and_then(|t| t.as_str())
            .and_then(|t| parse_count(t))
            .unwrap_or(0);

        Some(Self {
            tag,
            url,
            video_count,
        })
    }
}

/// Continuation token for pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continuation {
    pub token: String,
}

/// Parse length string (e.g., "10:30") to seconds.
fn parse_length_seconds(s: &str) -> Option<i32> {
    let parts: Vec<&str> = s.split(':').collect();
    match parts.len() {
        1 => {
            // Plain number of seconds without colon
            let seconds: i32 = parts[0].parse().ok()?;
            Some(seconds)
        }
        2 => {
            let minutes: i32 = parts[0].parse().ok()?;
            let seconds: i32 = parts[1].parse().ok()?;
            Some(minutes * 60 + seconds)
        }
        3 => {
            let hours: i32 = parts[0].parse().ok()?;
            let minutes: i32 = parts[1].parse().ok()?;
            let seconds: i32 = parts[2].parse().ok()?;
            Some(hours * 3600 + minutes * 60 + seconds)
        }
        _ => None,
    }
}

/// Parse count string (e.g., "1.5M subscribers") to i64.
fn parse_count(s: &str) -> Option<i64> {
    let s = s.replace(",", "");

    let multiplier: i64 = if s.contains("M") {
        1_000_000
    } else if s.contains("K") {
        1_000
    } else if s.contains("B") {
        1_000_000_000
    } else {
        1
    };

    let num_str = s
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect::<String>();

    let num: f64 = num_str.parse().ok()?;
    Some((num * multiplier as f64) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_length_seconds() {
        assert_eq!(parse_length_seconds("10:30"), Some(630));
        assert_eq!(parse_length_seconds("1:30:45"), Some(5445));
        assert_eq!(parse_length_seconds("59"), Some(59));
    }

    #[test]
    fn test_parse_count() {
        assert_eq!(parse_count("1.5M"), Some(1500000));
        assert_eq!(parse_count("500K"), Some(500000));
        assert_eq!(parse_count("100"), Some(100));
    }
}
