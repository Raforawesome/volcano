#![allow(unused_assignments, dead_code)]
use dioxus::prelude::*;

// #[inline_props]
// , note_names: Vec<String>
pub fn app(cx: Scope) -> Element {
	// let note_names: Vec<String> = vec![
	// 	"Unit 1 - Day 1".into(),
	// 	"Unit 1 - Day 2".into(),
	// ];
	cx.render(rsx!(
		style { include_str!("./index.css") }
		div {
			class: "sidebar-div",
			// note_names.iter().map(|s| rsx!(
			// 	button {
			// 		class: "note-name",
			// 		"{s}"
			// 	}
			// ))
		}
	))
}
