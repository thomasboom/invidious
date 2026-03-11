//! Templates module for Invidious.
//!
//! Handles HTML templating using Tera.

use tera::{Context, Tera};

/// Template renderer.
pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    /// Create a new template engine.
    pub fn new(template_dir: &str) -> anyhow::Result<Self> {
        let tera = Tera::new(template_dir)?;

        Ok(Self { tera })
    }

    /// Render a template with the given context.
    pub fn render(&self, template: &str, context: &Context) -> anyhow::Result<String> {
        let rendered = self.tera.render(template, context)?;
        Ok(rendered)
    }
}
