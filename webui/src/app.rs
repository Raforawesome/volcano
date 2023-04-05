use crate::{components::note_view::NoteView, get_notes};
use dioxus::prelude::*;
use reqwest_wasm as reqwest;
use std::future::Future;

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
    let note_raw = use_state(cx, || TEST_FILE.to_string());
    let note_names = use_future(cx, (), |_| async move {
        reqwest::get("http://127.0.0.1:7186/get_names")
            .await
            .unwrap()
            .json::<Vec<String>>()
            .await
            .unwrap()
    });

    cx.render(rsx!(
        style { include_str!("./css/index.css") }
        div { class: "sidebar",
            match note_names.value() {
                Some(v) => {
                    rsx!(
                    v.iter().map(|s| rsx!(
                        button {
                            class: "notebutton",
                            onclick: move |_| {
                                note_raw.set(crate::get_notes::get_note_content(&s));
                            },
                            "{s}"
                        }
                    )))
                },
                None => {rsx!(strong { class: "load-text", "Loading notes..." })}
            }
        }
        NoteView { contents: "{note_raw}" }
    ))
}
