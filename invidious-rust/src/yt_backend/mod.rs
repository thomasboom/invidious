//! YouTube backend module.
//!
//! Handles communication with YouTube's internal APIs.

mod youtube_api;
mod extractors;
mod connection_pool;
mod url_sanitizer;

pub use youtube_api::{
    ClientType,
    ClientConfig,
    YoutubeApi,
};

pub use extractors::{
    Video,
    Channel,
    Playlist,
    PlaylistVideo,
    Comment,
    CommentsResponse,
    SearchResult,
    Hashtag,
    Thumbnail,
    VideoBadges,
    Continuation,
};

pub use connection_pool::{
    ConnectionPool,
    SubdomainPool,
};

pub use url_sanitizer::UrlSanitizer;
