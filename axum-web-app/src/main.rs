use axum::middleware;
use axum::response::Response;
use axum::Router;
use model::ModelController;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;

mod error;
mod model;
mod web;

pub use crate::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mc = ModelController::new().await?;
    let router = Router::new()
        .merge(web::routes_hello::routes_hello())
        .merge(web::routes_login::routes_login())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(web::routes_static::routes_static());
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "MAP_RESPONSE");
    println!();
    res
}
