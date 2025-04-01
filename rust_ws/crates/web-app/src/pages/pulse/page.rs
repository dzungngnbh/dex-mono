use crate::backend::Backend;
use crate::pages::AppLayout;
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
            "Pulse",
            "Page description",
            page.render_once().unwrap().as_str(),
        )
        .unwrap();

        let s = app_layout.render_once().unwrap();
        let s = minify(s.as_bytes(), &Cfg::default());
        Html(s).into_response()
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
