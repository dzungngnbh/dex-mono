pub mod components;
pub mod four0four;
pub mod pulse;
pub mod trade;

use crate::pages::components::ui::toast::TOAST_UI;
use crate::pages::four0four::Four0FourPage;
use anyhow::Result;
use axum::response::{Html, IntoResponse};
use minify_html::{Cfg, minify};
use sailfish::TemplateOnce;

#[derive(TemplateOnce, Default)]
#[template(path = "layout.stpl")]
pub struct AppLayout {
    // metadata
    pub title: String,
    pub description: String,

    // main pages
    pub children: String,
    pub main_sidebar_ui: String,

    // components
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
    pub fn new(title: &str, description: &str, children: &str) -> Result<Self> {
        Ok(Self {
            title: title.to_string(),
            description: description.to_string(),
            children: children.to_string(),
            toast_ui: TOAST_UI.to_owned(),
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
        let template = AppLayout::new("test", "test", "test")?;
        let html = template.render_once()?;

        println!("{}", html);

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

        Ok(())
    }
}
