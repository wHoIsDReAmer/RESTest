use std::{backtrace::Backtrace, error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub backtrace: Backtrace,
}

impl ParseError {
    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
            backtrace: Backtrace::capture(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error: {}", self.message)
    }
}

impl Error for ParseError {}
