//! HTTP routes for Invidious.
//!
//! This module provides all HTTP route handlers for the Invidious application.

pub mod account;
pub mod api;
pub mod channels;
pub mod feeds;
pub mod home;
pub mod login;
pub mod playlists;
pub mod search;
pub mod watch;

use crate::config;
use crate::templates::TemplateEngine;
use api::AppState;
use axum::{
    routing::{get, post},
    Router,
};

/// Create the main router for the application.
pub fn create_router(config: config::Config, templates: TemplateEngine) -> Router<()> {
    let state = AppState::new(config, templates);
    
    Router::new()
        .layer(axum::Extension(state.clone()))
        // Home routes
        .route("/", get(home::home))
        .route("/privacy", get(home::privacy))
        .route("/licenses", get(home::licenses))
        .route("/redirect", get(home::redirect))
        
        // Watch routes
        .route("/watch", get(watch::watch))
        .route("/watch/{id}", get(watch::watch_id))
        .route("/live/{id}", get(watch::live_redirect))
        .route("/shorts/{id}", get(watch::shorts_redirect))
        .route("/clip/{clip}", get(watch::clip))
        .route("/embed/", get(watch::embed_redirect))
        .route("/embed/{id}", get(watch::embed))
        .route("/w/{id}", get(watch::watch_short))
        .route("/v/{id}", get(watch::video_path))
        .route("/e/{id}", get(watch::embed_path))
        .route("/download", post(watch::download))
        .route("/watch_ajax", post(watch::mark_watched))
        
        // Channel routes
        .route("/channel/{ucid}", get(channels::channel_home))
        .route("/channel/{ucid}/home", get(channels::channel_home_tab))
        .route("/channel/{ucid}/videos", get(channels::channel_videos))
        .route("/channel/{ucid}/shorts", get(channels::channel_shorts))
        .route("/channel/{ucid}/streams", get(channels::channel_streams))
        .route("/channel/{ucid}/podcasts", get(channels::channel_podcasts))
        .route("/channel/{ucid}/releases", get(channels::channel_releases))
        .route("/channel/{ucid}/courses", get(channels::channel_courses))
        .route("/channel/{ucid}/playlists", get(channels::channel_playlists))
        .route("/channel/{ucid}/community", get(channels::channel_community))
        .route("/channel/{ucid}/posts", get(channels::channel_posts))
        .route("/channel/{ucid}/channels", get(channels::channel_channels))
        .route("/channel/{ucid}/about", get(channels::channel_about))
        .route("/channel/{ucid}/live", get(channels::channel_live))
        .route("/channel/{ucid}/*", get(channels::channel_redirect))
        .route("/user/{user}", get(channels::user_channel))
        .route("/user/{user}/live", get(channels::user_live))
        .route("/user/{user}/{tab}", get(channels::user_channel_tab))
        .route("/c/{user}", get(channels::c_channel))
        .route("/c/{user}/live", get(channels::c_live))
        .route("/c/{user}/{tab}", get(channels::c_channel_tab))
        .route("/@{user}", get(channels::handle_channel))
        .route("/@{user}/{tab}", get(channels::handle_channel_tab))
        .route("/attribution_link", get(channels::attribution_link))
        .route("/attribution_link/{tab}", get(channels::attribution_link_tab))
        .route("/profile", get(channels::profile))
        .route("/profile/*", get(channels::profile_path))
        .route("/post/{id}", get(channels::post))
        
        // Playlist routes
        .route("/playlist", get(playlists::show_playlist))
        .route("/create_playlist", get(playlists::create_playlist_page))
        .route("/create_playlist", post(playlists::create_playlist))
        .route("/subscribe_playlist", get(playlists::subscribe_playlist))
        .route("/delete_playlist", get(playlists::delete_playlist_page))
        .route("/delete_playlist", post(playlists::delete_playlist))
        .route("/edit_playlist", get(playlists::edit_playlist))
        .route("/edit_playlist", post(playlists::update_playlist))
        .route("/add_playlist_items", get(playlists::add_playlist_items_page))
        .route("/playlist_ajax", post(playlists::playlist_ajax))
        .route("/mix", get(playlists::mix))
        .route("/watch_videos", get(playlists::watch_videos))
        
        // Search routes
        .route("/search", get(search::search))
        .route("/results", get(search::results))
        .route("/hashtag/{hashtag}", get(search::hashtag))
        .route("/opensearch.xml", get(search::opensearch))
        
        // Feed routes
        .route("/view_all_playlists", get(feeds::view_all_playlists))
        .route("/feed/playlists", get(feeds::playlists))
        .route("/feed/popular", get(feeds::popular))
        .route("/feed/trending", get(feeds::trending))
        .route("/feed/subscriptions", get(feeds::subscriptions))
        .route("/feed/history", get(feeds::history))
        .route("/feed/channel/{ucid}", get(feeds::rss_channel))
        .route("/feed/private", get(feeds::rss_private))
        .route("/feed/playlist/{plid}", get(feeds::rss_playlist))
        .route("/feeds/videos.xml", get(feeds::rss_videos))
        .route("/feed/webhook/{token}", get(feeds::push_notifications_get))
        .route("/feed/webhook/{token}", post(feeds::push_notifications_post))
        .route("/modify_notifications", get(feeds::modify_notifications))
        
        // Login routes
        .route("/login", get(login::login_page))
        .route("/login", post(login::login))
        .route("/signout", post(login::signout))
        
        // Account routes
        .route("/preferences", get(account::preferences))
        .route("/preferences", post(account::update_preferences))
        .route("/toggle_theme", get(account::toggle_theme))
        .route("/data_control", get(account::data_control))
        .route("/data_control", post(account::update_data_control))
        .route("/change_password", get(account::change_password_get))
        .route("/change_password", post(account::change_password_post))
        .route("/delete_account", get(account::delete_account_get))
        .route("/delete_account", post(account::delete_account_post))
        .route("/clear_watch_history", get(account::clear_history_get))
        .route("/clear_watch_history", post(account::clear_history_post))
        .route("/authorize_token", get(account::authorize_token_get))
        .route("/authorize_token", post(account::authorize_token_post))
        .route("/token_manager", get(account::token_manager))
        .route("/token_ajax", post(account::token_ajax))
        .route("/subscription_ajax", post(channels::subscription_ajax))
        .route("/subscription_manager", get(channels::subscription_manager))
        
        // API routes
        .route("/api/v1/", get(api::v1_index))
        .nest("/api/v1", api::create_router(state.clone()))
}
