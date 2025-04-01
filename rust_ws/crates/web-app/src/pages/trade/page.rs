use crate::backend::Backend;
use crate::layout::AppLayout;
use crate::lib::render_minified;
use crate::pages::components::ui::toast::render;
use anyhow::Context;
use axum::Extension;
use axum::extract::Path;
use axum::response::{Html, IntoResponse};
use log::{debug, info};
use minify_html::{Cfg, minify};
use sailfish::TemplateOnce;
use tracing::log;

#[derive(TemplateOnce, Default)]
#[template(path = "trade/head.stpl")]
pub struct Head {}

#[derive(TemplateOnce, Default)]
#[template(path = "trade/page.stpl")]
pub struct Page {}

impl Page {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn index(
        Extension(backend): Extension<Backend>,
        Path(symbol): Path<String>,
    ) -> impl IntoResponse {
        debug!("symbol: {}", symbol);

        let body_extra_attrs = r#"
          data-controller="trade-page"
          data-trade-page-product-id-value="3"
          data-trade-page-product-symbol-value="BTC_DAI"
          data-trade-size-increment-value="0.01"
        "#;

        let head = Head {}.render_once().unwrap();
        let page = Page::new();
        let app_layout = AppLayout::new(
            head,
            body_extra_attrs,
            "46,858.00 BTC-PERP | <ProjectName>",
            "Page description",
            page.render_once().unwrap().as_str(),
        )
        .unwrap();
        let s = app_layout
            .render_once()
            .context("failed to render page")
            .unwrap();
        render_minified(s)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_render() -> anyhow::Result<()> {
        let template = Page::new();
        let html = template.render_once().unwrap();
        dbg!(&html);

        Ok(())
    }
}
