use crate::api::auth::constants::USER_KEY;
use tower_cookies::{Cookie, PrivateCookies};

pub fn build_cookie(k: &str, v: &str) -> anyhow::Result<Cookie<'static>> {
    Ok(Cookie::build(k.to_string(), v.to_string())
        .path("/")
        .same_site(tower_cookies::cookie::SameSite::Lax)
        .http_only(true)
        .secure(true)
        .finish())
}

pub fn remove_leftover_user_id_cookie(signed_cookies: &PrivateCookies) {
    if let Some(cookie) = signed_cookies.get(USER_KEY) {
        signed_cookies.remove(cookie);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_cookie() {
        let cookie = build_cookie(USER_KEY, "bar").unwrap();
        assert_eq!(cookie.name(), USER_KEY);
        assert_eq!(cookie.value(), "bar");
    }
}
