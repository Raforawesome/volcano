//! The web frontend for the Volcano markdown server.
//! This web UI attempts to find a Volcano server running on localhost,
//! so it currently does not support working with remote servers.
//! As this crate uses Dioxus, it compiles to WASM and therefore can use
//! normal web features like web requests, but not things like manipulate
//! the filesystem of the server on which it's hosted.

mod app;
mod components;

fn main() {
    dioxus_web::launch(app::app);
}
