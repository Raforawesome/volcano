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
"#;

pub fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        style { include_str!("./css/index.css") }
        NoteView { contents: TEST_FILE }
    ))
}
