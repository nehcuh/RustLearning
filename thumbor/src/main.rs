mod pb;

use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use bytes::Bytes;
use lru::LruCache;
use pb::*;
use percent_encoding::{percent_decode, percent_decode_str};
use serde::Deserialize;
use tokio::{net::TcpListener, sync::Mutex};
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

#[derive(Deserialize)]
pub struct Params {
    spec: String,
    url: String,
}

type Cache = Arc<Mutex<LruCache<u64, Bytes>>>;

#[tokio::main]
async fn main() {
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    let cache: Cache = Arc::new(Mutex::new(LruCache::new(1024)));

    // 构建路由
    let app = Router::new()
        .route("/image/:spec/:url", get(generate))
        .layer(TraceLayer::new_for_http())
        .with_state(cache);

    // 运行 web 服务器
    let listenr = TcpListener::bind("127.0.0.1:3006").await.unwrap();
    axum::serve(listenr, app.into_make_service()).await.unwrap();
}

// 目前我们只需要把参数解析出来
async fn generate(Path(Params { spec, url }): Path<Params>) -> Result<String, StatusCode> {
    let url = percent_encoding::percent_decode_str(&url).decode_utf8_lossy();
    let spec: ImageSpec = spec
        .as_str()
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok(format!("url: {}\n spec: {:#?}", url, spec))
}

#[instrument(level = "info", skip(cache))]
async fn retrieve_image(url: &str, cache: Cache) -> Result<Bytes> {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let key = hasher.finish();

    let mut cache_lock = &mut cache.lock().await;
    if let Some(data) = cache_lock.get(&key) {
        !info("Match cache {}", key);
        data.to_owned();
    } else {
        !info("Retrieve url");
        let resp = reqwest::get(url).await?;
        let data = resp.bytes().await?;
        cache_lock.put(key, data.clone());
    }

    Ok(data)
}
