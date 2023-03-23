use crate::{components::note_view::NoteView, get_notes};
use dioxus::prelude::*;

const TEST_FILE: &str = r#"
# Welcome!
## To Volcano.
### A fully featured markdown renderer,
#### created purely in
##### Rust


**Supporting bold,**
**italic,**
Inline **bold** and *italic* text,

LaTeX in display mode,
$$\frac{1}{2}$$

and LaTeX in $\frac{1}{2}$ inline mode.

All features of LaTeX are supported:
$$f'(x)=\lim_{\Delta x \to 0} \frac{f(x+\Delta x)-f(x)}{\Delta x}$$
"#;

pub fn app(cx: Scope) -> Element {
	let note_names: Vec<&str> = get_notes::get_note_list();
	let note_raw = use_state(cx, || TEST_FILE.to_string());

    cx.render(rsx!(
        style { include_str!("./css/index.css") }
		div {
			class: "sidebar",
			note_names.clone().into_iter().map(|s| rsx!(
				button {
					class: "notebutton",
					onclick: move |_| {
						note_raw.set(crate::get_notes::get_note_content(s));
					},
					"{s}"
				}
			))
		}
        NoteView {
			contents: "{note_raw}"
		}
    ))
}
