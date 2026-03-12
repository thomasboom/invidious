//! Templates module for Invidious.
//!
//! Handles HTML templating using Tera.

use serde::Serialize;
use std::sync::Arc;
use tera::{Context, Tera as TeraEngine, Value};

#[derive(Clone)]
pub struct TemplateEngine {
    tera: Arc<TeraEngine>,
}

impl TemplateEngine {
    pub fn new(template_dir: &str) -> anyhow::Result<Self> {
        let mut tera = TeraEngine::new(template_dir)?;
        tera.register_filter("length_seconds", length_seconds_filter);
        Ok(Self {
            tera: Arc::new(tera),
        })
    }

    pub fn render(&self, template: &str, context: &Context) -> anyhow::Result<String> {
        let rendered = self.tera.render(template, context)?;
        Ok(rendered)
    }

    pub fn render_with_data<S: Serialize>(
        &self,
        template: &str,
        data: &S,
    ) -> anyhow::Result<String> {
        let mut context = Context::new();
        context.insert("version", env!("CARGO_PKG_VERSION"));
        context.insert("asset_commit", "dev");

        let json_value = serde_json::to_value(data)?;

        if let Some(obj) = json_value.as_object() {
            for (key, value) in obj {
                context.insert(key, value);
            }
        }

        self.render(template, &context)
    }

    pub fn render_base(&self, content: &str, data: &BaseTemplateData) -> anyhow::Result<String> {
        let mut context = Context::new();

        context.insert("version", env!("CARGO_PKG_VERSION"));
        context.insert("asset_commit", "dev");
        context.insert("content", &content);

        context.insert("locale", &data.locale);
        context.insert("dark_mode", &data.dark_mode);
        context.insert("navbar_search", &data.navbar_search);
        context.insert("current_page", &data.current_page);
        context.insert("user", &data.user.is_some());
        context.insert("user_email", &data.user_email);
        context.insert("notification_count", &data.notification_count);
        context.insert("show_nick", &data.preferences.show_nick);
        context.insert("csrf_token", &data.csrf_token);
        context.insert("banner", &data.banner);
        context.insert("modified_source_code_url", &data.modified_source_code_url);
        context.insert("login_enabled", &data.login_enabled);
        context.insert("thin_mode", &data.preferences.thin_mode);

        self.render("base.html", &context)
    }
}

fn length_seconds_filter(
    value: &Value,
    _: &std::collections::HashMap<String, Value>,
) -> Result<Value, tera::Error> {
    match value {
        Value::Number(n) => {
            if let Some(seconds) = n.as_i64() {
                let hours = seconds / 3600;
                let minutes = (seconds % 3600) / 60;
                let secs = seconds % 60;

                let result = if hours > 0 {
                    format!("{}:{:02}:{:02}", hours, minutes, secs)
                } else {
                    format!("{}:{:02}", minutes, secs)
                };
                Ok(Value::String(result))
            } else {
                Ok(Value::String("0:00".to_string()))
            }
        }
        _ => Ok(Value::String("0:00".to_string())),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BaseTemplateData {
    pub locale: String,
    pub dark_mode: String,
    pub navbar_search: bool,
    pub current_page: String,
    pub user: Option<crate::models::User>,
    pub user_email: String,
    pub notification_count: i32,
    pub preferences: crate::config::ConfigPreferences,
    pub csrf_token: String,
    pub banner: Option<String>,
    pub modified_source_code_url: Option<String>,
    pub login_enabled: bool,
}

impl Default for BaseTemplateData {
    fn default() -> Self {
        Self {
            locale: "en-US".to_string(),
            dark_mode: String::new(),
            navbar_search: true,
            current_page: "/".to_string(),
            user: None,
            user_email: String::new(),
            notification_count: 0,
            preferences: crate::config::ConfigPreferences::default(),
            csrf_token: String::new(),
            banner: None,
            modified_source_code_url: None,
            login_enabled: true,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VideoTemplateData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub published: String,
    pub author: String,
    pub author_id: String,
    pub author_thumbnails: Vec<crate::models::Thumbnail>,
    pub length_seconds: i64,
    pub view_count: i64,
    pub like_count: Option<i64>,
    pub live_now: bool,
    pub paid: bool,
    pub premium: bool,
    pub is_upcoming: bool,
    pub genre: Option<String>,
    pub video_thumbnails: Vec<crate::models::Thumbnail>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChannelTemplateData {
    pub id: String,
    pub display_name: String,
    pub name: String,
    pub description: Option<String>,
    pub author_thumbnails: Vec<crate::models::Thumbnail>,
    pub banner_thumbnails: Vec<crate::models::Thumbnail>,
    pub subscriber_count: i64,
    pub video_count: i64,
    pub is_verified: bool,
    pub banner: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaylistTemplateData {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub author: String,
    pub author_id: String,
    pub video_count: i64,
    pub thumbnail: Option<String>,
    pub videos: Vec<PlaylistVideoTemplateData>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlaylistVideoTemplateData {
    pub title: String,
    pub length_seconds: i64,
    pub video_id: String,
    pub author: String,
    pub author_id: String,
}
