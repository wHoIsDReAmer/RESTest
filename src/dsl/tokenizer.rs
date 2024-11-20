use thiserror::Error;

use super::tokens::{HttpMethod, Token};

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString {
        line: usize,
        column: usize,
    },
    #[error("Invalid token at line {line}, column {column}")]
    InvalidToken {
        last_token_string: String,
        line: usize,
        column: usize,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Tokenizer {
    buffer: Vec<char>,
    index: usize,

    last_char: char,

    row: usize,
    column: usize,
}

impl Tokenizer {
    pub fn new(buffer: Vec<char>) -> Self {
        let mut tokenizer = Self { buffer, ..Default::default() };
        tokenizer.next();
        tokenizer
    }

    pub fn is_newline(&self) -> bool {
        self.last_char == '\n'
    }

    pub fn is_whitespace(&self) -> bool {
        self.last_char == ' ' || self.last_char == '\t' || self.last_char == '\n'
    }

    pub fn is_first_column(&self) -> bool {
        self.column == 0
    }

    pub fn is_alphabet(&self) -> bool {
        self.last_char.is_alphabetic()
    }

    pub fn is_underscore(&self) -> bool {
        self.last_char == '_'
    }

    pub fn is_digit(&self) -> bool {
        self.last_char.is_ascii_digit()
    }

    pub fn is_quote(&self) -> bool {
        ['\'', '"'].contains(&self.last_char)
    }

    pub fn is_eof(&self) -> bool {
        self.index >= self.buffer.len()
    }

    fn next(&mut self) {
        if self.index >= self.buffer.len() {
            self.last_char = '\0';
            return
        }

        let ch = self.buffer.get(self.index).cloned().unwrap_or(' ');
        self.index += 1;
        self.last_char = ch;

        if self.is_newline() {
            self.row += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn undo(&mut self) {
        if self.index == 0 {
            self.last_char = '\0';
            return;
        }

        self.index -= 1;
        self.last_char = self.buffer.get(self.index-1).cloned().unwrap_or('\0');

        if self.is_newline() {
            self.row -= 1;
            self.column = 0;
        } else {
            self.column -= 1;
        }
    }

    pub fn tokenize(&mut self) -> Result<Token, TokenizerError> {
        // 공백 문자 처리 이전에 첫 컬럼에서 스페이스 2개를 만나면 indent 처리
        if self.is_first_column() {
            let mut count: i32 = 0;
            for _ in 0..2 {
                self.next();
                if self.last_char == ' ' {
                    count += 1;
                } else {
                    self.undo();
                    break;
                }
            }

            if count == 2 {
                return Ok(Token::Indent);
            }
        }

        // 공백 문자 처리 
        while !self.is_eof() && self.is_whitespace() {
            self.next();
        }

        // 알파벳이라면 키워드로 인식
        if self.is_alphabet() {
            let mut identifier = vec![self.last_char];

            self.next();
            while self.is_alphabet() || self.is_underscore() {
                identifier.push(self.last_char);
                self.next();
            }

            let identifier = String::from_iter(identifier).to_lowercase();
            let token = match identifier.as_str() {
                "test" => Token::Test,
                "endpoint" => Token::Endpoint,

                // methods
                "get" => Token::Method(HttpMethod::GET),
                "post" => Token::Method(HttpMethod::POST),
                "put" => Token::Method(HttpMethod::PUT),
                "delete" => Token::Method(HttpMethod::DELETE),
                "patch" => Token::Method(HttpMethod::PATCH),
                "options" => Token::Method(HttpMethod::OPTIONS),
                "head" => Token::Method(HttpMethod::HEAD),

                "headers" => Token::Headers,
                "expect" => Token::Expect,
                "body" => Token::Body,
                "status" => Token::Status,

                // comparing
                "contains" => Token::Contains,
                "equals" => Token::Equals,
                _ => return Err(TokenizerError::InvalidToken { last_token_string: identifier, line: self.row, column: self.column }),
            };

            return Ok(token);
        }
        // 숫자일 경우 정수로 인식
        else if self.is_digit() {
            let mut number = self.last_char.to_digit(10).unwrap();

            self.next();
            while self.is_digit() {
                number = number * 10 + self.last_char.to_digit(10).unwrap();
                self.next();
            }

            return Ok(Token::Number(number));
        }
        // 따옴표일 경우 문자열로 인식
        else if self.is_quote() {
            let seperator = self.last_char;

            let mut string = vec![];
            self.next();
            while self.last_char != seperator && !self.is_eof() {
                string.push(self.last_char);
                self.next();
            }

            if self.is_eof() {
                return Err(TokenizerError::UnterminatedString { line: self.row, column: self.column });
            }

            self.next();

            return Ok(Token::Literal(String::from_iter(string)));
        }
        
        Ok(Token::EOF)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_1() {
        let mut tokenizer = Tokenizer::new("123 \"test\"\n  ".chars().collect());

        assert_eq!(tokenizer.tokenize().unwrap(), Token::Number(123));
        assert_eq!(tokenizer.tokenize().unwrap(), Token::Literal("test".to_string()));
        assert_eq!(tokenizer.tokenize().unwrap(), Token::Indent);
        assert_eq!(tokenizer.tokenize().unwrap(), Token::EOF);
    }

    #[test]
    fn test_tokenize_2() {
        let mut tokenizer = Tokenizer::new("test 42 \"hello\" \n endpoint".chars().collect());

        assert_eq!(tokenizer.tokenize().unwrap(), Token::Test);
        assert_eq!(tokenizer.tokenize().unwrap(), Token::Number(42));
        assert_eq!(tokenizer.tokenize().unwrap(), Token::Literal("hello".to_string()));
        assert_eq!(tokenizer.tokenize().unwrap(), Token::Endpoint);
        assert_eq!(tokenizer.tokenize().unwrap(), Token::EOF);
    }
}
