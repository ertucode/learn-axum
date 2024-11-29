#![allow(unused)]

use std::net::SocketAddr;

pub use self::error::{Error, Result};

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod error;
mod web;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let app = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello {name}"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello {name}"))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
}
