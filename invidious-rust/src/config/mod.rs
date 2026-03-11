//! Configuration module for Invidious.
//!
//! Handles loading configuration from YAML files and environment variables.

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;

/// Database configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DBConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

/// Socket binding configuration for UNIX sockets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocketBindingConfig {
    pub path: String,
    pub permissions: String,
}

/// HTTP proxy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HTTPProxyConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
}

/// User preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ConfigPreferences {
    #[serde(default)]
    pub annotations: bool,
    #[serde(default)]
    pub annotations_subscribed: bool,
    #[serde(default = "default_true")]
    pub preload: bool,
    #[serde(default)]
    pub autoplay: bool,
    #[serde(default)]
    pub captions: Vec<String>,
    #[serde(default)]
    pub comments: Vec<String>,
    #[serde(default)]
    pub continue_watching: bool,
    #[serde(default = "default_true")]
    pub continue_autoplay: bool,
    #[serde(default)]
    pub dark_mode: String,
    #[serde(default)]
    pub latest_only: bool,
    #[serde(default)]
    pub listen: bool,
    #[serde(default)]
    pub local: bool,
    #[serde(default = "default_locale")]
    pub locale: String,
    #[serde(default = "default_true")]
    pub watch_history: bool,
    #[serde(default = "default_max_results")]
    pub max_results: i32,
    #[serde(default)]
    pub notifications_only: bool,
    #[serde(default = "default_player_style")]
    pub player_style: String,
    #[serde(default = "default_quality")]
    pub quality: String,
    #[serde(default = "default_quality_dash")]
    pub quality_dash: String,
    #[serde(default)]
    pub default_home: Option<String>,
    #[serde(default)]
    pub feed_menu: Vec<String>,
    #[serde(default)]
    pub automatic_instance_redirect: bool,
    #[serde(default = "default_region")]
    pub region: String,
    #[serde(default = "default_true")]
    pub related_videos: bool,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default = "default_speed")]
    pub speed: f32,
    #[serde(default)]
    pub thin_mode: bool,
    #[serde(default)]
    pub unseen_only: bool,
    #[serde(default)]
    pub video_loop: bool,
    #[serde(default)]
    pub extend_desc: bool,
    #[serde(default = "default_volume")]
    pub volume: i32,
    #[serde(default = "default_true")]
    pub vr_mode: bool,
    #[serde(default = "default_true")]
    pub show_nick: bool,
    #[serde(default)]
    pub save_player_pos: bool,
}

fn default_true() -> bool {
    true
}

fn default_locale() -> String {
    "en-US".to_string()
}

fn default_max_results() -> i32 {
    40
}

fn default_player_style() -> String {
    "invidious".to_string()
}

fn default_quality() -> String {
    "dash".to_string()
}

fn default_quality_dash() -> String {
    "auto".to_string()
}

fn default_region() -> String {
    "US".to_string()
}

fn default_sort() -> String {
    "published".to_string()
}

fn default_speed() -> f32 {
    1.0
}

fn default_volume() -> i32 {
    100
}

impl Default for ConfigPreferences {
    fn default() -> Self {
        Self {
            annotations: false,
            annotations_subscribed: false,
            preload: true,
            autoplay: false,
            captions: vec!["".to_string(), "".to_string(), "".to_string()],
            comments: vec!["youtube".to_string(), "".to_string()],
            continue_watching: false,
            continue_autoplay: true,
            dark_mode: String::new(),
            latest_only: false,
            listen: false,
            local: false,
            locale: "en-US".to_string(),
            watch_history: true,
            max_results: 40,
            notifications_only: false,
            player_style: "invidious".to_string(),
            quality: "dash".to_string(),
            quality_dash: "auto".to_string(),
            default_home: Some("Popular".to_string()),
            feed_menu: vec![
                "Popular".to_string(),
                "Trending".to_string(),
                "Subscriptions".to_string(),
                "Playlists".to_string(),
            ],
            automatic_instance_redirect: false,
            region: "US".to_string(),
            related_videos: true,
            sort: "published".to_string(),
            speed: 1.0,
            thin_mode: false,
            unseen_only: false,
            video_loop: false,
            extend_desc: false,
            volume: 100,
            vr_mode: true,
            show_nick: true,
            save_player_pos: false,
        }
    }
}

/// Invidious companion configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanionConfig {
    #[serde(default)]
    pub private_url: String,
    #[serde(default)]
    pub public_url: String,
    #[serde(default)]
    pub builtin_proxy: bool,
}

/// Jobs configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobsConfig {
    #[serde(default = "default_channel_threads")]
    pub channel_threads: i32,
    #[serde(default = "default_channel_refresh_interval")]
    pub channel_refresh_interval: String,
    #[serde(default = "default_feed_threads")]
    pub feed_threads: i32,
    #[serde(default)]
    pub full_refresh: bool,
}

fn default_channel_threads() -> i32 {
    1
}

fn default_channel_refresh_interval() -> String {
    "30m".to_string()
}

fn default_feed_threads() -> i32 {
    1
}

impl Default for JobsConfig {
    fn default() -> Self {
        Self {
            channel_threads: 1,
            channel_refresh_interval: "30m".to_string(),
            feed_threads: 1,
            full_refresh: false,
        }
    }
}

/// Main configuration structure for Invidious.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(default = "default_channel_threads")]
    pub channel_threads: i32,
    #[serde(default = "default_channel_refresh_interval")]
    pub channel_refresh_interval: String,
    #[serde(default = "default_feed_threads")]
    pub feed_threads: i32,
    #[serde(default = "default_output")]
    pub output: String,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default)]
    pub colorize_logs: bool,
    #[serde(default)]
    pub db: Option<DBConfig>,
    #[serde(default)]
    pub database_url: Option<String>,
    #[serde(default)]
    pub full_refresh: bool,
    #[serde(default)]
    pub jobs: JobsConfig,
    #[serde(default)]
    pub https_only: Option<bool>,
    #[serde(default)]
    pub hmac_key: String,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub use_pubsub_feeds: bool,
    #[serde(default = "default_true")]
    pub popular_enabled: bool,
    #[serde(default = "default_true")]
    pub captcha_enabled: bool,
    #[serde(default = "default_true")]
    pub login_enabled: bool,
    #[serde(default = "default_true")]
    pub registration_enabled: bool,
    #[serde(default)]
    pub statistics_enabled: bool,
    #[serde(default)]
    pub admins: Vec<String>,
    #[serde(default)]
    pub external_port: Option<u16>,
    #[serde(default)]
    pub default_user_preferences: ConfigPreferences,
    #[serde(default)]
    pub dmca_content: Vec<String>,
    #[serde(default)]
    pub check_tables: bool,
    #[serde(default)]
    pub cache_annotations: bool,
    #[serde(default)]
    pub banner: Option<String>,
    #[serde(default = "default_hsts")]
    pub hsts: Option<bool>,
    #[serde(default)]
    pub disable_proxy: Option<serde_yaml::Value>,
    #[serde(default = "default_true")]
    pub enable_user_notifications: bool,
    #[serde(default)]
    pub modified_source_code_url: Option<String>,
    #[serde(default)]
    pub force_resolve: Option<String>,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host_binding")]
    pub host_binding: String,
    #[serde(default)]
    pub socket_binding: Option<SocketBindingConfig>,
    #[serde(default = "default_pool_size")]
    pub pool_size: i32,
    #[serde(default)]
    pub http_proxy: Option<HTTPProxyConfig>,
    #[serde(default)]
    pub use_innertube_for_captions: bool,
    #[serde(default)]
    pub invidious_companion: Vec<CompanionConfig>,
    #[serde(default)]
    pub invidious_companion_key: String,
    #[serde(default = "default_playlist_length_limit")]
    pub playlist_length_limit: i32,
}

fn default_output() -> String {
    "STDOUT".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_hsts() -> Option<bool> {
    Some(true)
}

fn default_port() -> u16 {
    3000
}

fn default_host_binding() -> String {
    "0.0.0.0".to_string()
}

fn default_pool_size() -> i32 {
    100
}

fn default_playlist_length_limit() -> i32 {
    500
}

impl Default for Config {
    fn default() -> Self {
        Self {
            channel_threads: 1,
            channel_refresh_interval: "30m".to_string(),
            feed_threads: 1,
            output: "STDOUT".to_string(),
            log_level: "info".to_string(),
            colorize_logs: false,
            db: None,
            database_url: None,
            full_refresh: false,
            jobs: JobsConfig::default(),
            https_only: None,
            hmac_key: String::new(),
            domain: None,
            use_pubsub_feeds: false,
            popular_enabled: true,
            captcha_enabled: true,
            login_enabled: true,
            registration_enabled: true,
            statistics_enabled: false,
            admins: Vec::new(),
            external_port: None,
            default_user_preferences: ConfigPreferences::default(),
            dmca_content: Vec::new(),
            check_tables: false,
            cache_annotations: false,
            banner: None,
            hsts: Some(true),
            disable_proxy: None,
            enable_user_notifications: true,
            modified_source_code_url: None,
            force_resolve: None,
            port: 3000,
            host_binding: "0.0.0.0".to_string(),
            socket_binding: None,
            pool_size: 100,
            http_proxy: None,
            use_innertube_for_captions: false,
            invidious_companion: Vec::new(),
            invidious_companion_key: String::new(),
            playlist_length_limit: 500,
        }
    }
}

impl Config {
    /// Load configuration from a YAML file.
    pub fn load_from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Load configuration from environment variable.
    pub fn load_from_env() -> anyhow::Result<Self> {
        let contents = std::env::var("INVIDIOUS_CONFIG")
            .map_err(|_| anyhow::anyhow!("INVIDIOUS_CONFIG not set"))?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    /// Get the database URL.
    pub fn database_url(&self) -> String {
        if let Some(url) = &self.database_url {
            return url.clone();
        }

        if let Some(db) = &self.db {
            return format!(
                "postgres://{}:{}@{}:{}/{}",
                db.user, db.password, db.host, db.port, db.dbname
            );
        }

        String::new()
    }

    /// Get the server address.
    pub fn address(&self) -> SocketAddr {
        format!("{}:{}", self.host_binding, self.port)
            .parse()
            .unwrap_or_else(|_| SocketAddr::from(([0, 0, 0, 0], 3000)))
    }
}
