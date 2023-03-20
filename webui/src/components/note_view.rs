use dioxus::prelude::*;
use dioxus_web::use_eval;
#[allow(unused)]
use log::{error, info};
use volcano_parser::{TextType, TokenType};

#[inline_props]
#[allow(non_snake_case)]
pub fn NoteView<'a>(cx: Scope, contents: &'a str) -> Element {
    let tokens = volcano_parser::tokenize_markdown(contents);

    let _ = use_eval(cx)("renderMathInElement(document.getElementById(note-view));");

    cx.render(rsx!(
        div { id: "note-view",
            tokens.iter().map(|t| {
                match &t.ty {
                    TokenType::H1 => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            h1 { "{sub}" }
                        )
                    }
                    TokenType::H2 => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            h2 { "{sub}" }
                        )
                    }
                    TokenType::H3 => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            h3 { "{sub}" }
                        )
                    }
                    TokenType::H4 => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            h4 { "{sub}" }
                        )
                    }
                    TokenType::H5 => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            h5 { "{sub}" }
                        )
                    }
                    TokenType::Bold => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            b { "{sub}" }
                        )
                    }
                    TokenType::Italic => {
                        let sub: &'a str = &contents[t.span.0..t.span.1];
                        rsx!(
                            i { "{sub}" }
                        )
                    }
                    TokenType::Inline(tokens) => {
                        info!("Found inline token");
                        rsx!(
                            p {
                                tokens.iter().map(|t| {
                                    match t.ty {
                                        TextType::Bold => {
                                            rsx!(
                                                b { &contents[t.span.0..t.span.1] },
                                                " "
                                            )
                                        }
                                        TextType::Italic => {
                                            rsx!(
                                                i { &contents[t.span.0..t.span.1] },
                                                " "
                                            )
                                        }
                                        TextType::Plain => {
                                            rsx!(
                                                &contents[t.span.0..t.span.1],
                                                " "
                                            )
                                        }
                                    }
                                })
                            }
                        )
                    }
                    TokenType::Newline => rsx!(br {}),
                    _ => rsx!(
                        b { "Invalid token!" }
                    ),
                }
    })
        }
    ))
}
