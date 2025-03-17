pub mod components;
pub mod four0four;
pub mod trade;

use anyhow::Result;
use axum::response::{Html, IntoResponse};
use minify_html::{minify, Cfg};
use sailfish::TemplateOnce;

use crate::app::four0four::Four0FourPage;

#[derive(TemplateOnce, Default)]
#[template(path = "layout.stpl")]
pub struct AppLayout {
    // metadata
    pub title: String,
    pub description: String,

    // main app
    pub children: String,
    pub main_sidebar_ui: String,

    pub toast_ui: String,
}

#[derive(TemplateOnce, Default)]
#[template(path = "static_layout.stpl")]
pub struct StaticLayout {
    pub title: String,
    pub description: String,
    pub children: String,
}

impl AppLayout {
    pub fn new() -> Result<Self> {
        let toast_ui = components::ui::toast::render()?;

        Ok(Self {
            toast_ui,
            ..Default::default()
        })
    }
}

pub async fn four0four_index() -> impl IntoResponse {
    let page = Four0FourPage {};
    let children = page.render_once().unwrap();
    let root = StaticLayout {
        title: "Trading Exec".to_string(),
        description: "TODO".to_string(),
        children,
    };

    let s = root.render_once().unwrap();
    let s = minify(s.as_bytes(), &Cfg::default());
    Html(s).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_index_template() -> Result<()> {
        let template = AppLayout {
            title: "title".to_string(),
            description: "description".to_string(),
            ..Default::default()
        };
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }

    #[test]
    fn test_render_static_template() -> Result<()> {
        let template = StaticLayout {
            title: "title".to_string(),
            description: "description".to_string(),
            children: "children".to_string(),
        };
        let html = template.render_once().unwrap();
        dbg!(html);

        Ok(())
    }
}
