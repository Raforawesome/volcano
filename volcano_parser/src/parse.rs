#![allow(unused)]

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    H1,
    H2,
    H3,
    H4,
    H5,
    Bold,
    Italic,
	Plain,
    Inline(Vec<TextToken>),
    Newline,
    Latex(LatexType),
    List(ListType, Vec<Span>),
    Invalid,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TextType {
    Bold,
    Italic,
	Plain,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MdToken {
    pub ty: TokenType,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextToken {
    pub ty: TextType,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LatexType {
    Inline,
    Display,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListType {
    Ord,
    Unord,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct DebounceCounter {
    d1: bool,
    d2: bool,
}

impl DebounceCounter {
    pub fn new() -> Self {
        Self {
            d1: false,
            d2: false,
        }
    }

    pub fn reset(&mut self) {
        self.d1 = false;
        self.d2 = false;
    }

    pub fn blank_line(&mut self) {
        if !self.d1 {
            self.d1 = true;
        } else if self.d1 && !self.d2 {
            self.d2 = true;
        }
    }

    pub fn both(&mut self) -> bool {
        if self.d1 && !self.d2 {
            self.d2 = true;
            true
        } else {
            false
        }
    }
}

pub fn tokenize_markdown(md: &str) -> Vec<MdToken> {
    let mut buffer: Vec<MdToken> = vec![];
    let mut char_pos: usize = 0;
    let mut blank_counter = DebounceCounter::new();

    for line in md.lines() {
        if line.trim() == "" {
            char_pos += 1;
            blank_counter.blank_line();
            if blank_counter.both() {
                buffer.push(MdToken {
                    ty: TokenType::Newline,
                    span: Span::default(),
                });
            }
            continue;
        } else {
            blank_counter.reset();
        }
        if line.starts_with("# ") {
            buffer.push(MdToken {
                ty: TokenType::H1,
                span: Span(char_pos + 2, char_pos + line.len()),
            });
        } else if line.starts_with("## ") {
            buffer.push(MdToken {
                ty: TokenType::H2,
                span: Span(char_pos + 3, char_pos + line.len()),
            });
        } else if line.starts_with("### ") {
            buffer.push(MdToken {
                ty: TokenType::H3,
                span: Span(char_pos + 4, char_pos + line.len()),
            });
        } else if line.starts_with("#### ") {
            buffer.push(MdToken {
                ty: TokenType::H4,
                span: Span(char_pos + 5, char_pos + line.len()),
            });
        } else if line.starts_with("##### ") {
            buffer.push(MdToken {
                ty: TokenType::H5,
                span: Span(char_pos + 6, char_pos + line.len()),
            });
        } else if line.starts_with("**") && line.ends_with("**") {
            buffer.push(MdToken {
                ty: TokenType::Bold,
                span: Span(char_pos + 2, char_pos + line.len() - 2),
            });
        } else if line.starts_with('*') && line.ends_with('*') {
            buffer.push(MdToken {
                ty: TokenType::Italic,
                span: Span(char_pos + 1, char_pos + line.len() - 1),
            });
        } else {
			let mut buffer2: Vec<TextToken> = vec![];
			for s in line.split(' ') {
				if s.starts_with("**") && s.ends_with("**") {
					buffer2.push(TextToken {
						ty: TextType::Bold,
						span: Span(char_pos + 2, char_pos + s.len() - 2),
					});
				} else if s.starts_with('*') && s.ends_with('*') {
					buffer2.push(TextToken {
						ty: TextType::Italic,
						span: Span(char_pos + 1, char_pos + s.len() - 1),
					});
				} else {
					buffer2.push(TextToken {
						ty: TextType::Plain,
						span: Span(char_pos, char_pos + s.len())
					});
				}
				char_pos += s.len() + 1;
			}
			buffer.push(MdToken { ty: TokenType::Inline(buffer2), span: Span::default() });
			continue;
		}
        char_pos += line.len() + 1;
    }

    buffer
}
