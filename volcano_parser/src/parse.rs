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
    Latex(LatexType, Span),
    List(ListType, Vec<Span>),
    Invalid,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MdToken {
    pub ty: TokenType,
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

pub fn tokenize_markdown(md: &str) -> Vec<MdToken> {
    let mut buffer: Vec<MdToken> = vec![];
    let mut char_pos: usize = 0;
    for line in md.lines() {
        if line.trim() == "" {
            continue;
        }
        if line.starts_with("# ") {
            buffer.push(MdToken {
                ty: TokenType::H1,
                span: Span(char_pos + 2, char_pos + line.len()),
            });
        }
        char_pos += line.len();
    }
    buffer
}
