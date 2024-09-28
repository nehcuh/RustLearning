use crate::{web, Error, Result};
use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Deserialize)]
pub struct LoginPayload {
    username: String,
    password: String,
}

pub fn routes_login() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, Json(payload): Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sig"));

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));
    Ok(body)
}
