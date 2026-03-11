//! Invidious is an alternative front-end to YouTube.
//!
//! This crate provides the core functionality for the Invidious project,
//! including YouTube backend integration, database access, and HTTP routing.

/// Configuration module for loading and managing Invidious settings.
pub mod config;
/// Database module for PostgreSQL connectivity.
pub mod db;
/// Data models for videos, channels, playlists, and users.
pub mod models;
/// HTTP route handlers.
pub mod routes;
/// YouTube backend API client.
pub mod yt_backend;
/// Background job runners.
pub mod jobs;
/// HTML template rendering.
pub mod templates;

pub use config::Config;