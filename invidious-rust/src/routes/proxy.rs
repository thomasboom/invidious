//! Proxy routes for static assets and YouTube content.
//!
//! This module provides handlers for:
//! - Image proxy routes (ggpht, vi, yts/img, sb)
//! - Video playback proxy (videoplayback)

use axum::{
    body::Body,
    extract::{Query, Extension},
    response::{IntoResponse, Response},
};
use axum::http::{HeaderMap, StatusCode};
use reqwest::Client;
use std::collections::HashMap;
use std::time::Duration;

const REQUEST_HEADERS_WHITELIST: &[&str] = &[
    "accept",
    "accept-language",
    "cache-control",
    "content-length",
    "content-type",
    "forwarded",
    "from",
    "host",
    "if-match",
    "if-modified-since",
    "if-none-match",
    "if-range",
    "origin",
    "pragma",
    "referer",
    "user-agent",
    "x-forwarded-for",
    "x-forwarded-host",
];

const RESPONSE_HEADERS_BLACKLIST: &[&str] = &[
    "connection",
    "content-encoding",
    "content-length",
    "content-security-policy",
    "strict-transport-security",
    "transfer-encoding",
    "www-authenticate",
];

#[derive(Clone)]
pub struct ProxyClient {
    client: Client,
}

impl ProxyClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        Self { client }
    }

    pub async fn proxy_request(
        &self,
        url: &str,
        headers: &HeaderMap,
    ) -> Result<axum::response::Response, StatusCode> {
        let mut builder = self.client.get(url);

        for (name, value) in headers.iter() {
            let name_str = name.as_str();
            if REQUEST_HEADERS_WHITELIST.contains(&name_str) {
                if let Ok(value_str) = value.to_str() {
                    builder = builder.header(name_str, value_str);
                }
            }
        }

        let response = builder.send().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

        let status = response.status();
        let mut axum_response = Response::builder().status(status);

        for (name, value) in response.headers().iter() {
            let name_str = name.as_str();
            if !RESPONSE_HEADERS_BLACKLIST.contains(&name_str) {
                if let Ok(value_str) = value.to_str() {
                    axum_response = axum_response.header(name_str, value_str);
                }
            }
        }

        axum_response = axum_response.header("access-control-allow-origin", "*");

        let bytes = response.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
        let body = Body::from(bytes);

        Ok(axum_response.body(body).unwrap())
    }
}

impl Default for ProxyClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct StaticAssets {
    client: ProxyClient,
}

impl StaticAssets {
    pub fn new() -> Self {
        Self {
            client: ProxyClient::new(),
        }
    }

    pub async fn ggpht(
        Extension(state): Extension<Self>,
        axum::extract::Path(path): axum::extract::Path<String>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let url = format!("https://yt3.ggpht.com/{}", path);
        state.client.proxy_request(&url, &headers).await.unwrap_or_else(|_| {
            (StatusCode::NOT_FOUND, "Not found").into_response()
        })
    }

    pub async fn video_thumbnail(
        Extension(state): Extension<Self>,
        axum::extract::Path((id, name)): axum::extract::Path<(String, String)>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let url = format!("https://i.ytimg.com/vi/{}/{}", id, name);
        state.client.proxy_request(&url, &headers).await.unwrap_or_else(|_| {
            (StatusCode::NOT_FOUND, "Not found").into_response()
        })
    }

    pub async fn yts_image(
        Extension(state): Extension<Self>,
        axum::extract::Path(name): axum::extract::Path<String>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let url = format!("https://www.youtube.com/yts/img/{}", name);
        state.client.proxy_request(&url, &headers).await.unwrap_or_else(|_| {
            (StatusCode::NOT_FOUND, "Not found").into_response()
        })
    }

    pub async fn storyboard(
        Extension(state): Extension<Self>,
        axum::extract::Path((_authority, id, storyboard, index)): axum::extract::Path<(String, String, String, String)>,
        query: Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let query_string = query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        let url = format!(
            "https://i.ytimg.com/sb/{}/{}/{}/{}?{}",
            id, storyboard, index, "L1", query_string
        );
        state.client.proxy_request(&url, &headers).await.unwrap_or_else(|_| {
            (StatusCode::NOT_FOUND, "Not found").into_response()
        })
    }

    pub async fn videoplayback(
        Extension(state): Extension<Self>,
        axum::extract::Path(path): axum::extract::Path<String>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let url = format!("https://r3---sn-4g5e6l7e.googlevideo.com/videoplayback/{}", path);

        let final_url = if let Some(host_header) = headers.get("host") {
            if let Ok(host) = host_header.to_str() {
                if host.contains("googlevideo.com") || host.contains("c.youtube.com") {
                    format!("https://{}/videoplayback/{}", host, path)
                } else {
                    url
                }
            } else {
                url
            }
        } else {
            url
        };

        state.client.proxy_request(&final_url, &headers).await.unwrap_or_else(|_| {
            (StatusCode::NOT_FOUND, "Not found").into_response()
        })
    }
}

impl Default for StaticAssets {
    fn default() -> Self {
        Self::new()
    }
}
