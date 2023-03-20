use crate::components::note_view::NoteView;
use dioxus::prelude::*;

const TEST_FILE: &str = r#"
# Header 1
## Header 2
### Header 3
#### Header 4
##### Header 5

double newline test
no newline test

**bold**
Inline **bold** text

*italic*
Inline *italic* test

Inline **bold** and *italic* test

KaTeX display test:
$$\frac{1}{2}$$

KaTeX inline in a $\frac{1}{2}$ sentence test

Complex KaTeX test:
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
