use dioxus::prelude::*;
use volcano_parser::TokenType;

#[inline_props]
#[allow(non_snake_case)]
pub fn NoteView<'a>(cx: Scope, contents: &'a str) -> Element {
    let tokens = volcano_parser::tokenize_markdown(contents);

    cx.render(rsx!(
        p {
            "Running a "
            b {
                "bold"
            }
            " test"
        }

        tokens.iter()
        // .filter(|t| (&contents[t.span.0..t.span.1]).trim() != "")
        .map(|t| {
            match t.ty {
                TokenType::H1 => {
                    let sub: &'a str = &contents[t.span.0..t.span.1];
                    rsx!(
                        h1 { "{sub}" }
                    )
                }
                _ => rsx!(
                    b { "Invalid token!" }
                )
            }
        })
    ))
}
