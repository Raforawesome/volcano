use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: &str) -> Self {
        Self {
            msg: msg.to_string(),
        }
    }

    pub fn new_from_string(msg: String) -> Self {
        Self { msg }
    }
}

#[macro_export]
macro_rules! parse_error {
    ($x:literal, $($arg:literal),+) => {
        ParseError::new_from_string(format!($x, $($arg),+))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ParseError: {}", self.msg)
    }
}

impl Error for ParseError {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {}

#[derive(Debug, Clone)]
pub struct TokenStream {
    internal: Vec<Token>,
    pos: usize,
}

impl Default for TokenStream {
    fn default() -> Self {
        Self {
            internal: Vec::new(),
            pos: 0,
        }
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.internal.len() {
            None
        } else {
            let token = self.internal[self.pos];
            self.pos += 1;
            Some(token)
        }
    }
}
