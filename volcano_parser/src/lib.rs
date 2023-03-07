//! # Volcano Markdown Parser
//! **A blazingly fast, zero-allocation minimal markdown parser.**
//! 
//! ---
//! 
//! This parser **does not** aim to implement every markdown feature.
//! It implements a minimal superset, based on what I use for my notes.
//! Notably, this includes headers, bold text, italic text, and most 
//! importantly LaTeX mode, with support for both inline ($expr$) and
//! display ($$expr$$) modes.

mod parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
