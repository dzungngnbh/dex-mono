use axum::response::IntoResponse;
use axum::routing::post;
use axum::{extract, Router};
use serde::Deserialize;

use crate::auth::session_context::SessionContext;
use crate::lib::api::{check_auth, ApiResponse};

pub fn routes() -> Router {
    Router::new().route("/place_order", post(place_order))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderPayload {
    pub amount: f64,
    pub order_type: u8,
    pub price: f64,
    pub product_id: u32,
    pub timestamp: u64,
}

/// Returns the digested struct to be signed by account
async fn place_order(
    session_context: SessionContext,
    extract::Json(payload): extract::Json<PlaceOrderPayload>,
) -> impl IntoResponse {
    check_auth!(session_context);

    ApiResponse::success(None)
}
