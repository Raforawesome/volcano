#![allow(unused)]

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span(usize, usize);

#[derive(Clone, Debug, PartialEq)]
pub enum MdToken {
	H1(Span),
	H2(Span),
	H3(Span),
	H4(Span),
	H5(Span),
	Bold(Span),
	Italic(Span),
	Latex(LatexType, Span),
	List(ListType, Vec<Span>)
}

#[derive(Clone, Debug, PartialEq)]
pub enum LatexType {
	Inline,
	Display
}

#[derive(Clone, Debug, PartialEq)]
pub enum ListType {
	Ord,
	Unord
}

pub fn tokenize_markdown(md: &str) -> Vec<MdToken> {
	todo!()
}
