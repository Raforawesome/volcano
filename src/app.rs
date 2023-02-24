use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
	cx.render(rsx!(
		style { include_str!("./index.css") }
		div {
			class: "sidebar-div"
		}
	))
}
