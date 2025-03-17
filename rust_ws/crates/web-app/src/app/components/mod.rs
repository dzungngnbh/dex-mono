use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::Extension;
use ecow::EcoString;
use minify_html::{minify, Cfg};
use serde::Deserialize;

use crate::app::components::ui::toast;
use crate::auth::session_context::SessionContext;
use crate::backend::Backend;
use crate::lib::serde::empty_ecostring_as_none;

pub mod breadcrumb;
pub mod hotwired_turbo;
pub mod link;
pub mod meta;
pub mod ui;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Params {
    // Toast
    #[serde(default, deserialize_with = "empty_ecostring_as_none")]
    pub toast_title: Option<EcoString>,

    #[serde(default, deserialize_with = "empty_ecostring_as_none")]
    pub toast_description: Option<EcoString>,
}

pub async fn components(
    Extension(backend): Extension<Backend>,
    Path(component_name): Path<String>,
    Query(params): Query<Params>,
    session_context: SessionContext,
) -> impl IntoResponse {
    let res = match component_name.as_str() {
        "toast_item" => {
            if params.toast_title.is_none() && params.toast_description.is_none() {
                "".to_string()
            } else {
                let toast_title = params.toast_title.unwrap_or_default();
                let toast_description = params.toast_description.unwrap_or_default();
                toast::render_toast_item(toast_title, toast_description).unwrap()
            }
        }

        _ => "".to_string(),
    };

    let res = minify(res.as_bytes(), &Cfg::default());
    Html(res).into_response()
}
