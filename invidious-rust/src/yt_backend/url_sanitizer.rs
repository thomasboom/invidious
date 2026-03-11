//! URL sanitization module for YouTube URLs.
//!
//! Provides functions to sanitize and normalize YouTube URLs to prevent
//! malicious or unexpected URL parameters from being processed.

use std::collections::HashSet;
use url::Url;

/// Allowed query parameters for different YouTube URL types.
const ALLOWED_CHANNEL_PARAMS: &[&str] = &["u", "user", "lb"];
const ALLOWED_PLAYLIST_PARAMS: &[&str] = &["list"];
const ALLOWED_SEARCH_PARAMS: &[&str] = &["q", "search_query", "sp"];
const ALLOWED_WATCH_PARAMS: &[&str] = &[
    "v",             // Video ID
    "list",          // Playlist-related
    "index",         // Playlist index
    "playlist",      // Unnamed playlist
    "t",             // Timestamp
    "time_continue", // Timestamp
    "start",         // Start time
    "end",           // End time
    "lc",            // Highlighted comment
];

/// URL types that can be processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UrlType {
    Watch,
    Channel,
    Playlist,
    Search,
    Unknown,
}

/// URL sanitizer for YouTube URLs.
pub struct UrlSanitizer;

impl UrlSanitizer {
    /// Process a YouTube URL and return a sanitized version.
    ///
    /// This function takes a user-supplied YouTube URL and returns a
    /// sanitized URI with only the allowed parameters.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string to sanitize
    ///
    /// # Returns
    ///
    /// A new URL with only the allowed parameters.
    pub fn process(url: &str) -> Url {
        let url = Self::normalize_url(url);

        if let Ok(unsafe_uri) = Url::parse(&url) {
            let unsafe_host = unsafe_uri.host_str();
            let unsafe_path = unsafe_uri.path();
            let mut new_url = Url::parse("https://www.youtube.com")
                .unwrap_or_else(|_| Url::parse("https://youtube.com").unwrap());

            if unsafe_host.is_none() || unsafe_path.is_empty() {
                return new_url;
            }

            let breadcrumbs: Vec<&str> = unsafe_path
                .split('/')
                .filter(|s| !s.is_empty())
                .filter(|bc| Self::is_ascii_word(bc))
                .collect();

            if breadcrumbs.is_empty() {
                return new_url;
            }

            let url_type = Self::determine_url_type(breadcrumbs[0]);

            new_url.set_path(&format!("/{}", breadcrumbs.join("/")));

            if url_type != UrlType::Unknown {
                let allowed_params = Self::allowed_params_for_type(url_type);
                let allowed_set: HashSet<&str> = allowed_params.iter().cloned().collect();
                let filtered_params: Vec<(String, String)> = unsafe_uri
                    .query_pairs()
                    .filter(|(key, _)| allowed_set.contains(key.as_ref()))
                    .map(|(k, v)| (k.into_owned(), v.into_owned()))
                    .collect();

                let query_string: String = filtered_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", percent_escape(k), percent_escape(v)))
                    .collect::<Vec<_>>()
                    .join("&");

                new_url.set_query(if query_string.is_empty() {
                    None
                } else {
                    Some(&query_string)
                });
            }

            new_url
        } else {
            Url::parse("https://www.youtube.com")
                .unwrap_or_else(|_| Url::parse("https://youtube.com").unwrap())
        }
    }

    /// Extract video ID from a YouTube URL.
    pub fn extract_video_id(url: &str) -> Option<String> {
        let url = Self::normalize_url(url);

        if let Ok(parsed) = Url::parse(&url) {
            // Check for youtu.be short URLs
            if parsed.host_str() == Some("youtu.be") {
                return parsed
                    .path_segments()
                    .and_then(|mut s| s.next())
                    .map(String::from);
            }

            // Check for youtube.com URLs
            if parsed
                .host_str()
                .map(|h| h.contains("youtube.com"))
                .unwrap_or(false)
            {
                // Check path for /watch
                if parsed.path().starts_with("/watch") {
                    return parsed
                        .query_pairs()
                        .find(|(k, _)| k == "v")
                        .map(|(_, v)| v.into_owned());
                }

                // Check path for /embed or /v
                if parsed.path().starts_with("/embed/") || parsed.path().starts_with("/v/") {
                    let segments: Vec<&str> = parsed.path_segments()?.collect();
                    if segments.len() >= 2 {
                        return Some(segments[1].to_string());
                    }
                }

                // Check path for /shorts
                if parsed.path().starts_with("/shorts/") {
                    let segments: Vec<&str> = parsed.path_segments()?.collect();
                    if segments.len() >= 2 {
                        return Some(segments[1].to_string());
                    }
                }
            }
        }

        None
    }

    /// Extract playlist ID from a YouTube URL.
    pub fn extract_playlist_id(url: &str) -> Option<String> {
        let url = Self::normalize_url(url);

        if let Ok(parsed) = Url::parse(&url) {
            return parsed
                .query_pairs()
                .find(|(k, _)| k == "list")
                .map(|(_, v)| v.into_owned());
        }

        None
    }

    /// Extract channel ID from a YouTube URL.
    pub fn extract_channel_id(url: &str) -> Option<String> {
        let url = Self::normalize_url(url);

        if let Ok(parsed) = Url::parse(&url) {
            let path = parsed.path();

            // Handle custom URLs like /c/name, /user/name, /@handle
            if path.starts_with("/c/") {
                return Some(path.strip_prefix("/c/").unwrap_or("").to_string());
            }
            if path.starts_with("/user/") {
                return Some(path.strip_prefix("/user/").unwrap_or("").to_string());
            }
            if path.starts_with("/@") {
                return Some(path.strip_prefix("/@").unwrap_or("").to_string());
            }
            if path.starts_with("/channel/") {
                return path.strip_prefix("/channel/").map(String::from);
            }
        }

        None
    }

    /// Determine the URL type based on the path.
    fn determine_url_type(path_root: &str) -> UrlType {
        match path_root {
            "watch" | "w" | "v" | "embed" | "e" | "shorts" | "clip" => UrlType::Watch,
            "channel" | "c" | "user" | "profile" | "attribution_link" => UrlType::Channel,
            "playlist" | "mix" => UrlType::Playlist,
            "results" | "search" => UrlType::Search,
            _ if path_root.starts_with('@') => UrlType::Channel,
            _ => UrlType::Unknown,
        }
    }

    /// Get allowed parameters for a URL type.
    fn allowed_params_for_type(url_type: UrlType) -> &'static [&'static str] {
        match url_type {
            UrlType::Watch => ALLOWED_WATCH_PARAMS,
            UrlType::Channel => ALLOWED_CHANNEL_PARAMS,
            UrlType::Playlist => ALLOWED_PLAYLIST_PARAMS,
            UrlType::Search => ALLOWED_SEARCH_PARAMS,
            UrlType::Unknown => &[],
        }
    }

    /// Normalize a URL by adding https:// if missing.
    fn normalize_url(url: &str) -> String {
        if url.starts_with("http://") || url.starts_with("https://") {
            url.to_string()
        } else {
            format!("https://{}", url)
        }
    }

    /// Check if a string is an ASCII word (alphanumeric and underscore/hyphen only).
    fn is_ascii_word(s: &str) -> bool {
        if s.bytes().len() != s.chars().count() {
            return false;
        }

        s.bytes().all(|b| {
            b.is_ascii_lowercase()
                || b.is_ascii_uppercase()
                || b.is_ascii_digit()
                || b == b'-'
                || b == b'_'
        })
    }
}

/// Simple percent encoding for query parameters.
fn percent_escape(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push_str(&format!("%{:02X}", byte));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determine_url_type() {
        assert_eq!(UrlSanitizer::determine_url_type("watch"), UrlType::Watch);
        assert_eq!(UrlSanitizer::determine_url_type("shorts"), UrlType::Watch);
        assert_eq!(
            UrlSanitizer::determine_url_type("channel"),
            UrlType::Channel
        );
        assert_eq!(
            UrlSanitizer::determine_url_type("@handle"),
            UrlType::Channel
        );
        assert_eq!(
            UrlSanitizer::determine_url_type("playlist"),
            UrlType::Playlist
        );
        assert_eq!(UrlSanitizer::determine_url_type("search"), UrlType::Search);
    }

    #[test]
    fn test_extract_video_id() {
        assert_eq!(
            UrlSanitizer::extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            UrlSanitizer::extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            UrlSanitizer::extract_video_id("https://www.youtube.com/embed/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
        assert_eq!(
            UrlSanitizer::extract_video_id("https://www.youtube.com/shorts/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ".to_string())
        );
    }

    #[test]
    fn test_is_ascii_word() {
        assert!(UrlSanitizer::is_ascii_word("video123"));
        assert!(UrlSanitizer::is_ascii_word("test_video"));
        assert!(UrlSanitizer::is_ascii_word("abc-def"));
        assert!(!UrlSanitizer::is_ascii_word("test video"));
        assert!(!UrlSanitizer::is_ascii_word("test/video"));
    }
}
