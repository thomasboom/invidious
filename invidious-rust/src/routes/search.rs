//! Search routes for Invidious.
//!
//! Handles search functionality, results, hashtags, and opensearch.

use axum::{
    extract::{Path, Query},
    response::Html,
};
use serde::Deserialize;

/// Query parameters for search routes.
#[derive(Debug, Deserialize)]
pub struct SearchParams {
    #[serde(default)]
    pub q: Option<String>,
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
    #[serde(default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub features: Option<String>,
    #[serde(default)]
    pub ch: Option<String>,
    #[serde(default)]
    pub uc: Option<String>,
}

/// Search page handler.
pub async fn search(Query(params): Query<SearchParams>) -> Html<String> {
    if let Some(query) = params.q {
        Html(format!(
            "<html><body><h1>Search: {}</h1></body></html>",
            query
        ))
    } else {
        Html("<html><body><h1>Search</h1></body></html>".to_string())
    }
}

/// Search results page handler.
pub async fn results(Query(params): Query<SearchParams>) -> Html<String> {
    if let Some(query) = params.q {
        Html(format!(
            "<html><body><h1>Results for: {}</h1></body></html>",
            query
        ))
    } else {
        Html("<html><body><h1>Search Results</h1></body></html>".to_string())
    }
}

/// Hashtag handler.
pub async fn hashtag(Path(hashtag): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Hashtag: {}</h1></body></html>",
        hashtag
    ))
}

/// OpenSearch XML handler.
pub async fn opensearch() -> Html<&'static str> {
    Html(r#"<?xml version="1.0" encoding="UTF-8"?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
  <ShortName>Invidious</ShortName>
  <Description>Search Invidious</Description>
  <Url type="text/html" template="https://invidious.site/search?q={searchTerms}"/>
</OpenSearchDescription>"#)
}
