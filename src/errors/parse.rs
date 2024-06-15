use std::fmt::{Display, Formatter, write};

#[derive(Debug)]
pub struct ParseError {
    message: &'static str
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ParseError {
    pub fn from_str(s: &'static str) -> Self {
        ParseError{
            message: s,
        }
    }
}