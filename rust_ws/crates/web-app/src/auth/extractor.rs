use crate::auth::constants::USER_KEY;
use crate::auth::session_context::SessionContext;
use crate::auth::COOKIE_KEY;
use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use tower_cookies::Cookies;

#[async_trait]
impl<S> FromRequestParts<S> for SessionContext
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let mut session_context = SessionContext {
            account_address: None,
        };

        let cookies = Cookies::from_request_parts(req, state).await?;
        let signed_cookies = cookies.private(&COOKIE_KEY);
        let user_id_cookie = signed_cookies.get(USER_KEY);
        if user_id_cookie.is_some() {
            let sender = user_id_cookie.unwrap().value().to_string();
            session_context.account_address = Some(sender);
        }
        Ok(session_context)
    }
}
