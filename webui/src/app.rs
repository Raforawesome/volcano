use crate::components::note_view::NoteView;
use dioxus::prelude::*;

const TEST_FILE: &str = r#"
# Welcome!
## To Volcano.
### A fully featured markdown renderer,
#### made in
##### Dioxus


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
    cx.render(rsx!(
        style { include_str!("./css/index.css") }
		div {
			class: "sidebar",
			button {
				"note 1"
			}
		}
        NoteView {
			contents: TEST_FILE
		}
    ))
}
