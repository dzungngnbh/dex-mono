use crate::backend::Backend;
use crate::layout::AppLayout;
use crate::lib::render_minified;
use axum::Extension;
use axum::response::{Html, IntoResponse};
use minify_html::{Cfg, minify};
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "pulse/page.stpl")]
pub struct Page<'a> {
    title: &'a str,
}

impl Page<'_> {
    pub fn new() -> Self {
        Self { title: "Pulse" }
    }

    pub async fn index(Extension(backend): Extension<Backend>) -> impl IntoResponse {
        let page = Page::new();
        let app_layout = AppLayout::new(
            "head".to_string(),
            "body_extra_attrs",
            "Pulse",
            "Page description",
            page.render_once().unwrap().as_str(),
        )
        .unwrap();

        let s = app_layout.render_once().unwrap();
        render_minified(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_render() -> Result<()> {
        let template = Page::new();
        let html = template.render_once()?;

        Ok(())
    }
}
