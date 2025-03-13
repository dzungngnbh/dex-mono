use crate::backend::Backend;
use anyhow::Result;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::Extension;
use log::info;
use minify_html::{minify, Cfg};
use sailfish::TemplateOnce;
use tracing::log;

#[derive(TemplateOnce, Default)]
#[template(path = "trade/page.stpl")]
pub struct Page {
    pub title: String,
}

impl Page {
    pub fn new() -> Self {
        Self {
            title: "46,858.00 BTC-PERP | <ProjectName> ".to_string(),
        }
    }

    pub async fn index(
        Extension(backend): Extension<Backend>,
        Path(symbol): Path<String>,
    ) -> impl IntoResponse {
        info!("symbol: {}", symbol);

        let page = Page::new();

        let s = page.render_once().unwrap();
        let s = minify(s.as_bytes(), &Cfg::default());
        Html(s).into_response()
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_render() -> Result<()> {
        let template = Page {
            title: "46,858.00 BTC-PERP | <ProjectName> ".to_string(),
        };
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }
}
