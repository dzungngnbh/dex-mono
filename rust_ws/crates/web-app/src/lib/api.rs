use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;

// refactor OrderCommand to Order only
#[macro_export]
macro_rules! check_auth {
    ($session_context:ident) => {
        if !$session_context.is_authenticated() {
            return ApiResponse::unauthorized_error();
        }
    };
}
pub use check_auth;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub error: Option<serde_json::Value>,
}

impl ApiResponse {
    /// Create a new ApiResponse with success = true and optional data
    /// Usage: ApiResponse::success(Some(json!({"message": "success"})))
    pub fn success(data: Option<serde_json::Value>) -> Response {
        (
            StatusCode::OK,
            Json(ApiResponse {
                success: true,
                data,
                error: None,
            }),
        )
            .into_response()
    }

    pub fn unauthorized_error() -> Response {
        (
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(json!({"message": "Unauthorized"})),
            }),
        )
            .into_response()
    }

    pub fn invalid_input_error() -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(json!({"message": "Invalid input"})),
            }),
        )
            .into_response()
    }

    pub fn no_content_error() -> Response {
        (
            StatusCode::NO_CONTENT,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(json!({"message": "No content"})),
            }),
        )
            .into_response()
    }

    pub fn internal_server_error() -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                success: false,
                data: None,
                error: Some(json!({"message": "Internal server error"})),
            }),
        )
            .into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_input() {
        let response = ApiResponse::invalid_input_error();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
