use crate::{components::note_view::NoteView, get_notes};
use dioxus::prelude::*;
use reqwest_wasm as reqwest;
use std::future::Future;
use reqwest::Client;
use std::sync::LazyLock;

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

async fn get_note_content(note_name: &str) -> String {
	static client: LazyLock<Client> = LazyLock::new(|| Client::new());
	client.get("http://127.0.0.1:7186/get_note")
		.json(note_name)
		.send()
		.await
		.unwrap()
		.text()
		.await
		.unwrap()
}

pub fn app(cx: Scope) -> Element {
	static CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| reqwest::Client::new());
	let selected_note = use_state(cx, || "".to_string());
    let mut note_raw = use_future(cx, selected_note, |_| {
		let note_name: String = selected_note.to_string();
		async move {
		CLIENT
			.request(reqwest::Method::GET, "http://127.0.0.1:7186/get_note")
			.header("note", note_name)
			.send()
			.await
			.unwrap()
			.text()
			.await 
			.unwrap_or(TEST_FILE.to_string())
	}});
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
								selected_note.set(s.to_string());
                            },
                            "{s}"
                        }
                    )))
                },
                None => {rsx!(strong { class: "load-text", "Loading notes..." })}
            }
        }
        NoteView { contents: TEST_FILE }
        NoteView { contents: match note_raw.value() {
            Some(v) => { &v }
            None => { "Loading note..." }
        }}
    ))
}
