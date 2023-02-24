#![feature(try_blocks)]
mod app;
mod fs;

fn main() {
	dioxus_web::launch(app::app);
}
