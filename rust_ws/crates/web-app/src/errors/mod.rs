use crate::lib::api::ApiResponse;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;

pub fn unauthorized_error() -> (StatusCode, Json<ApiResponse>) {
    (
        StatusCode::UNAUTHORIZED,
        axum::Json(ApiResponse {
            success: false,
            data: None,
            error: Some(json!({"message": "Unauthorized"})),
        }),
    )
}

pub fn invalid_input_error(error_details: &str) -> (StatusCode, Json<ApiResponse>) {
    let message = format!("Invalid inputs: {}", error_details);

    (
        StatusCode::BAD_REQUEST,
        axum::Json(ApiResponse {
            success: false,
            data: None,
            error: Some(json!({"message": message})),
        }),
    )
}
