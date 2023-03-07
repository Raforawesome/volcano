use dioxus::prelude::*;
use crate::components::note_view::NoteView;

const TEST_FILE: &str = 
r#"
# Header 1
## Header 2
### Header 3

**bold**
Inline **bold** text

*italic*
Inline *italic* test
"#;

pub fn app(cx: Scope) -> Element {
	cx.render(
		rsx!(
			style { include_str!("./css/index.css") }
			h1 { "Hello, world!" }
			NoteView { contents: TEST_FILE }
		)
	)
}
