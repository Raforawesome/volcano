use dioxus::prelude::*;
use dioxus_web::use_eval;
#[allow(unused)]
use log::{error, info};
use volcano_parser::{TextType, TokenType, LatexType};

#[inline_props]
#[allow(non_snake_case)]
pub fn NoteView<'a>(cx: Scope, contents: &'a str) -> Element {
    let tokens = volcano_parser::tokenize_markdown(contents);

    cx.render(rsx!(
        div { id: "note-view", class: "noteview",
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
					TokenType::Latex => {
						let html: String;
						let substr: &str = &contents[t.span.0..t.span.1];
						let opts = katex::Opts::builder().display_mode(true).build().unwrap();
						html = katex::render_with_opts(&substr.replace('$', ""), &opts).unwrap();
						rsx!(
							p {
								dangerous_inner_html: "{html}"
							}
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
										TextType::Latex => {
											let html: String;
											let substr: &str = &contents[t.span.0..t.span.1];
											let opts = katex::Opts::builder().display_mode(false).build().unwrap();
											html = katex::render_with_opts(&substr.replace('$', ""), &opts).unwrap();
											rsx!(
												span {
													dangerous_inner_html: "{html}"
												}
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


					// TokenType::Latex(latex_type) => {
					// 	let s: String;
					// 	if *latex_type == LatexType::Display {
					// 		if let Value(st) = 
					// 		scripts(&format!("katex.render({})", &contents[t.span.0..t.span.1])).get().unwrap()
					// 		{}
					// 	}
					// 	rsx!(
					// 		p {
					// 			dangerous_inner_html: "{s}"
					// 		}
					// 	)
					// }