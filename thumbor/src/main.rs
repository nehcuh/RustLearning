mod pb;
use axum::{extract::Path, http::StatusCode, routing::get, Router};
pub use pb::*;

use percent_encoding::percent_decode_str;
use serde::Deserialize;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Params {
    spec: String,
    url: String,
}

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();

    // 构建路由
    let app = Router::new().route("/image/:spec/:url", get(generate));

    // 运行服务器
    let listener = TcpListener::bind("127.0.0.1:3456").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
