#![feature(try_blocks)]
mod app;
mod fs;

fn main() {
	// let note_names: Vec<String> = fs::get_note_names().unwrap();
	dioxus_web::launch(app::app);
	// dioxus_web::launch_with_props(app::app, app::appProps { note_names }, dioxus_web::Config::default());
}
