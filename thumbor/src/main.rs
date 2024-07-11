use axum::{extract::Path, http::StatusCode, Router};
use percent_encoding::percent_decode_str;
use serde::Deserialize;

mod pb;
use pb::*;
use tokio::net::TcpListener;

// 参数使用 serde 做 Deserialize, axum 会自动识别并解析
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
    let app = Router::new().route("/image/:spec/:url", axum::routing::get(generate));

    let listener = TcpListener::bind("127.0.0.1:3456").await.unwrap();
    tracing::debug!("listening on 3456",);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// 目前我们就只把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}
