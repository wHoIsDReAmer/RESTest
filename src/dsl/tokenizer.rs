use thiserror::Error;

use super::tokens::Token;

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },
    #[error("Invalid token at line {line}, column {column}")]
    InvalidToken { line: usize, column: usize },
}

#[derive(Debug, Clone, Default)]
pub struct Tokenizer {
    input: String,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        Self { input, ..Default::default() }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        todo!()
    }
}
