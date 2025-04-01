pub mod components;
pub mod four0four;
pub mod pulse;
pub mod trade;

use crate::layout::StaticLayout;
use crate::lib::render_minified;
use crate::pages::four0four::Four0FourPage;
use axum::response::IntoResponse;
use sailfish::TemplateOnce;

pub async fn four0four_index() -> impl IntoResponse {
    let page = Four0FourPage {};
    let children = page.render_once().unwrap();
    let root = StaticLayout {
        title: "Trading Exec".to_string(),
        description: "TODO".to_string(),
        children,
    };

    let s = root.render_once().unwrap();
    render_minified(s)
}
