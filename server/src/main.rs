//! The Volcano Markdown Viewer server. This server exposes
//! a port on localhost which the web UI (hosted on the same machine)
//! communicates with to fetch and display notes.

#![feature(fs_try_exists)]
mod handlers;
use axum::{routing::get, Router};
use std::fs;
use tower_http::cors::{Any, CorsLayer};

const LISTEN_ON: &str = "0.0.0.0:7186";

#[tokio::main]
async fn main() {
    if !fs::try_exists("notes/").unwrap() {
        fs::create_dir("notes/").unwrap();
    }

    let cors: CorsLayer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .route("/get_names", get(handlers::get_note_names))
        .route("/get_note", get(handlers::get_note_content))
        .layer(cors);

    axum::Server::bind(&LISTEN_ON.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
