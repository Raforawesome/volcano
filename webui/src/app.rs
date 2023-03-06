use dioxus::prelude::*;

pub fn app(cx: Scope) -> Element {
	cx.render(
		rsx!(
			style { include_str!("./css/index.css") }
			h1 { "Hello, world!" }
		)
	)
}
