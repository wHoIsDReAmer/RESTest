use std::{backtrace::Backtrace, error::Error, fmt};

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub backtrace: Backtrace,
}

impl PartialEq for ParseError {
    fn eq(&self, other: &Self) -> bool {
        self.message == other.message
    }
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
        write!(f, "parse error: {}\nbacktrace: {}",
            self.message, self.backtrace)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_error_message() {
        let error = ParseError::new("test error");
        assert_eq!(error.message, "test error");
    }

    #[test]
    fn test_parse_error_display() {
        let error = ParseError::new("display test");
        let display_string = format!("{}", error);

        println!("{}", display_string);
        assert!(display_string.starts_with("parse error: display test\nbacktrace:"));
    }

    #[test]
    fn test_parse_error_equality() {
        let error1 = ParseError::new("same message");
        let error2 = ParseError::new("same message");
        let error3 = ParseError::new("different message");
        
        assert_eq!(error1, error2);
        assert_ne!(error1, error3);
    }

    #[test]
    fn test_parse_error_debug() {
        let error = ParseError::new("debug test");
        let debug_string = format!("{:?}", error);
        assert!(debug_string.contains("debug test"));
    }
}
