use axum::response::{Html, IntoResponse};
use minify_html::{Cfg, minify};

/// Render the page with minified HTML
pub fn render_minified(s: String) -> impl IntoResponse {
    let s = minify(s.as_bytes(), &Cfg::default());
    Html(s).into_response()
}
