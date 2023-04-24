//! The web frontend for the Volcano markdown server.
//! This web UI attempts to find a Volcano server running on localhost,
//! so it currently does not support working with remote servers.
//! As this crate uses Dioxus, it compiles to WASM and therefore can use
//! normal web features like web requests, but not things like manipulate
//! the filesystem of the server on which it's hosted.

#![feature(lazy_cell)]
mod app;
mod components;
mod get_notes;

#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(app::app);
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    println!("INCOMPATIBLE PLATFORM: This crate is only compatible with WASM.");
}
