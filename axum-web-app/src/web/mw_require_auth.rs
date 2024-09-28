use crate::{web::AUTH_TOKEN, Error, Result};
use axum::{body::Body, http::Request, middleware::Next, response::Response};
use tower_cookies::Cookies;

pub async fn mw_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    auth_token.ok_or(Error::AuthFailNotAuthTokenCookie);

    Ok(next.run(req).await)
}
