use dioxus::prelude::*;

fn app(cx: &Scope) -> Element {
	cx.render(rsx!(
		style { include_str!("./index.css") }
	))
}
