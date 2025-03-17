use axum::response::IntoResponse;
use axum::Router;
use axum::{
    extract,
    routing::{get, post},
};
use ecow::EcoString;
use serde::Deserialize;
use serde_json::json;
use session_context::SessionContext;
use std::sync::LazyLock;
use tower_cookies::Cookies;

use crate::auth::constants::USER_KEY;
use crate::auth::cookie::{build_cookie, remove_leftover_user_id_cookie};
use crate::auth::evm_utils::is_valid_signature;
use crate::lib::api::ApiResponse;

mod constants;
pub mod cookie;
mod evm_utils;
pub mod extractor;
pub mod session_context;

pub(crate) static COOKIE_KEY: LazyLock<tower_cookies::Key> = LazyLock::new(|| {
    let cookie_key = std::env::var("COOKIE_KEY").expect("COOKIE_KEY must be set in env.");
    let cookie_key_bytes = cookie_key.as_bytes();
    tower_cookies::Key::from(cookie_key_bytes)
});

pub fn routes() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/is_login", get(is_login))
        .route("/logout", post(login)) // TODO
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    pub sender: EcoString,
    pub signed_message: EcoString,
}

/// Login user with signed message from user, we dont persist user for now
async fn login(
    cookies: Cookies,
    extract::Json(payload): extract::Json<LoginPayload>,
) -> impl IntoResponse {
    let private_cookies = cookies.private(&COOKIE_KEY);
    remove_leftover_user_id_cookie(&private_cookies);

    match is_valid_signature(
        &payload.sender,
        constants::LOGIN_MESSAGE,
        &payload.signed_message,
    ) {
        Ok(is_valid) => {
            if is_valid {
                private_cookies.add(build_cookie(USER_KEY, &payload.sender).unwrap())
            } else {
                return ApiResponse::invalid_input_error();
            }
        }
        Err(e) => {
            return ApiResponse::invalid_input_error();
        }
    }

    ApiResponse::success(None)
}

/// Check if user is login
async fn is_login(session_context: SessionContext) -> impl IntoResponse {
    let mut is_login = false;
    if session_context.account_address.is_some() {
        is_login = true;
    }

    ApiResponse::success(Some(json!({"is_login": is_login})))
}

/// Force logout
async fn logout(cookies: Cookies) -> impl IntoResponse {
    let private_cookies = cookies.private(&COOKIE_KEY);
    remove_leftover_user_id_cookie(&private_cookies);

    ApiResponse::success(None)
}
