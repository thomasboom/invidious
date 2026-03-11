//! Channel routes for Invidious.
//!
//! Handles channel pages, user profiles, and related routes.

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;

/// Query parameters for channel routes.
#[derive(Debug, Deserialize)]
pub struct ChannelParams {
    #[serde(default)]
    pub page: Option<String>,
    #[serde(default)]
    pub sort: Option<String>,
    #[serde(default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub variant: Option<String>,
}

/// Channel home page handler.
pub async fn channel_home(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel: {}</h1></body></html>",
        ucid
    ))
}

/// Channel home with explicit home tab.
pub async fn channel_home_tab(Path((ucid, _tab)): Path<(String, String)>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Home</h1></body></html>",
        ucid
    ))
}

/// Channel videos tab handler.
pub async fn channel_videos(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Videos</h1></body></html>",
        ucid
    ))
}

/// Channel shorts tab handler.
pub async fn channel_shorts(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Shorts</h1></body></html>",
        ucid
    ))
}

/// Channel streams tab handler.
pub async fn channel_streams(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Live Streams</h1></body></html>",
        ucid
    ))
}

/// Channel podcasts tab handler.
pub async fn channel_podcasts(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Podcasts</h1></body></html>",
        ucid
    ))
}

/// Channel releases tab handler.
pub async fn channel_releases(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Releases</h1></body></html>",
        ucid
    ))
}

/// Channel courses tab handler.
pub async fn channel_courses(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Courses</h1></body></html>",
        ucid
    ))
}

/// Channel playlists tab handler.
pub async fn channel_playlists(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Playlists</h1></body></html>",
        ucid
    ))
}

/// Channel community tab handler.
pub async fn channel_community(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Community</h1></body></html>",
        ucid
    ))
}

/// Channel posts tab handler (alias for community).
pub async fn channel_posts(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Posts</h1></body></html>",
        ucid
    ))
}

/// Channel subchannels tab handler.
pub async fn channel_channels(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} Channels</h1></body></html>",
        ucid
    ))
}

/// Channel about tab handler.
pub async fn channel_about(Path(ucid): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Channel {} About</h1></body></html>",
        ucid
    ))
}

/// Channel live redirect handler.
pub async fn channel_live(Path(ucid): Path<String>) -> Redirect {
    Redirect::to(&format!("/channel/{}/streams", ucid))
}

/// User live redirect handler.
pub async fn user_live(Path(user): Path<String>) -> Redirect {
    Redirect::to(&format!("/user/{}/streams", user))
}

/// C channel live redirect handler.
pub async fn c_live(Path(user): Path<String>) -> Redirect {
    Redirect::to(&format!("/c/{}/streams", user))
}

/// Channel catch-all redirect handler.
pub async fn channel_redirect(Path((ucid, _rest)): Path<(String, String)>) -> Redirect {
    Redirect::to(&format!("/channel/{}", ucid))
}

/// User handler (brand channel).
pub async fn user_channel(Path(user): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>User: {}</h1></body></html>",
        user
    ))
}

/// User with tab handler.
pub async fn user_channel_tab(Path((user, tab)): Path<(String, String)>) -> Html<String> {
    Html(format!(
        "<html><body><h1>User: {} - {}</h1></body></html>",
        user, tab
    ))
}

/// C handler (brand channel short form).
pub async fn c_channel(Path(user): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>C: {}</h1></body></html>",
        user
    ))
}

/// C with tab handler.
pub async fn c_channel_tab(Path((user, tab)): Path<(String, String)>) -> Html<String> {
    Html(format!(
        "<html><body><h1>C: {} - {}</h1></body></html>",
        user, tab
    ))
}

/// @handle handler (channel handle).
pub async fn handle_channel(Path(user): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>@{}</h1></body></html>",
        user
    ))
}

/// @handle with tab handler.
pub async fn handle_channel_tab(Path((user, tab)): Path<(String, String)>) -> Html<String> {
    Html(format!(
        "<html><body><h1>@{} - {}</h1></body></html>",
        user, tab
    ))
}

/// Attribution link handler.
pub async fn attribution_link(Query(params): Query<ChannelParams>) -> impl IntoResponse {
    if let Some(a) = params.type_ {
        Redirect::to(&a)
    } else {
        Redirect::to("/")
    }
}

/// Attribution link with tab handler.
pub async fn attribution_link_tab(Path(_tab): Path<String>) -> Redirect {
    Redirect::to("/")
}

/// Profile handler.
pub async fn profile(Query(params): Query<ChannelParams>) -> impl IntoResponse {
    if let Some(user) = params.type_ {
        Redirect::to(&format!("/@{}", user))
    } else {
        Redirect::to("/")
    }
}

/// Profile with path handler.
pub async fn profile_path(Path(_rest): Path<String>) -> Redirect {
    Redirect::to("/")
}

/// Post handler.
pub async fn post(Path(id): Path<String>) -> Html<String> {
    Html(format!(
        "<html><body><h1>Post: {}</h1></body></html>",
        id
    ))
}

/// Toggle subscription handler.
pub async fn subscription_ajax() -> Html<&'static str> {
    Html("{}")
}

/// Subscription manager handler.
pub async fn subscription_manager() -> Html<&'static str> {
    Html("<html><body><h1>Subscription Manager</h1></body></html>")
}
