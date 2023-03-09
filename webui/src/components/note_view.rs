use dioxus::prelude::*;
use volcano_parser::{TokenType, TextType};
#[allow(unused)]
use log::{info, error};

#[inline_props]
#[allow(non_snake_case)]
pub fn NoteView<'a>(cx: Scope, contents: &'a str) -> Element {
    let tokens = volcano_parser::tokenize_markdown(contents);

    cx.render(rsx!(tokens.iter().map(|t| {
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
    })))
}
