use thiserror::Error;

use super::tokens::{HttpMethod, Token};

#[derive(Error, Debug)]
pub enum TokenError {
    #[error("Unterminated string at line {line}, column {column}")]
    UnterminatedString { line: usize, column: usize },
    #[error("Invalid token at line {line}, column {column}")]
    InvalidToken {
        last_token_string: String,
        line: usize,
        column: usize,
    },
}

#[derive(Debug, Clone, Default)]
pub struct Lexer {
    buffer: Vec<char>,
    index: usize,

    last_char: char,

    row: usize,
    column: usize,
}

impl Lexer {
    pub fn new(buffer: Vec<char>) -> Self {
        let mut tokenizer = Self {
            buffer,
            ..Default::default()
        };
        tokenizer.next();
        tokenizer
    }

    pub fn is_newline(&self) -> bool {
        self.last_char == '\n'
    }

    pub fn is_whitespace(&self) -> bool {
        self.last_char == ' '
            || self.last_char == '\t'
            || self.last_char == '\n'
            || self.last_char == '\r'
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

    pub fn is_dash(&self) -> bool {
        self.last_char == '-'
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
            return;
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
        self.last_char = self.buffer.get(self.index - 1).cloned().unwrap_or('\0');

        if self.is_newline() {
            self.row -= 1;
            // TODO: Column 처리 필요
            self.column = 0;
        } else {
            self.column -= 1;
        }
    }

    pub fn tokenize(&mut self) -> Result<Token, TokenError> {
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
            while self.is_alphabet() || self.is_underscore() || self.is_dash() {
                identifier.push(self.last_char);
                self.next();
            }

            let identifier = String::from_iter(identifier).to_uppercase();
            let token = match identifier.as_str() {
                "TEST" => Token::Test,
                "ENDPOINT" => Token::Endpoint,

                // methods
                "GET" => Token::Method(HttpMethod::GET),
                "POST" => Token::Method(HttpMethod::POST),
                "PUT" => Token::Method(HttpMethod::PUT),
                "DELETE" => Token::Method(HttpMethod::DELETE),
                "PATCH" => Token::Method(HttpMethod::PATCH),
                "OPTIONS" => Token::Method(HttpMethod::OPTIONS),
                "HEAD" => Token::Method(HttpMethod::HEAD),

                "HEADERS" => Token::Headers,
                "EXPECT" => Token::Expect,
                "BODY" => Token::Body,
                "STATUS" => Token::Status,

                // comparing
                "CONTAINS" => Token::Contains,
                "EQUALS" => Token::Equals,

                _ => Token::Item(identifier),
                // _ => return Err(TokenError::InvalidToken { last_token_string: identifier, line: self.row, column: self.column }),
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
            let mut escape = false;

            let mut string = vec![];

            self.next();
            while (self.last_char != seperator || escape) && !self.is_eof() {
                // 이스케이핑 처리
                if escape {
                    match self.last_char {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => string.push(self.last_char),
                    }
                    escape = false;
                } else if self.last_char == '\\' {
                    escape = true;
                } else {
                    string.push(self.last_char);
                }
                self.next();
            }

            if self.is_eof() {
                return Err(TokenError::UnterminatedString {
                    line: self.row,
                    column: self.column,
                });
            }

            self.next();

            return Ok(Token::Literal(String::from_iter(string)));
        }

        if self.is_eof() {
            return Ok(Token::EOF);
        }

        Err(TokenError::InvalidToken {
            last_token_string: self.last_char.to_string(),
            line: self.row,
            column: self.column,
        })
    }

    // 생성 없이 가져올 수 있는 유틸 함수
    pub fn string_to_tokens(string: &str) -> Result<Vec<Token>, TokenError> {
        let mut tokenizer = Lexer::new(string.chars().collect());
        let mut tokens = vec![];

        while !tokenizer.is_eof() {
            tokens.push(tokenizer.tokenize()?);
        }

        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenize_1() {
        let tokens = Lexer::string_to_tokens("123 \"test\"\n  ").unwrap();

        assert_eq!(tokens[0], Token::Number(123));
        assert_eq!(tokens[1], Token::Literal("test".to_string()));
        assert_eq!(tokens[2], Token::Indent);
    }

    #[test]
    fn test_simple_tokenize_2() {
        let tokens = Lexer::string_to_tokens("test 42 \"hello\" \n endpoint").unwrap();

        assert_eq!(tokens[0], Token::Test);
        assert_eq!(tokens[1], Token::Number(42));
        assert_eq!(tokens[2], Token::Literal("hello".to_string()));
        assert_eq!(tokens[3], Token::Endpoint);
    }

    #[test]
    fn test_verbose_tokenize() {
        let raw_tokens = r#"
test "Get User Info"
endpoint "https://api.example.com/users/123"    
headers
  Authorization "Bearer your-token-here"
  Content-Type "application/json"
expect
  status 200
  body contains "\"username\": \"john\""
"#;

        let tokens = Lexer::string_to_tokens(raw_tokens).unwrap();

        assert_eq!(tokens[0], Token::Test);
        assert_eq!(tokens[1], Token::Literal("Get User Info".to_string()));
        assert_eq!(tokens[2], Token::Endpoint);
        assert_eq!(tokens[3], Token::Literal("https://api.example.com/users/123".to_string()));
        assert_eq!(tokens[4], Token::Headers);
        assert_eq!(tokens[5], Token::Indent);
        assert_eq!(tokens[6], Token::Item("AUTHORIZATION".to_string()));
        assert_eq!(tokens[7], Token::Literal("Bearer your-token-here".to_string()));
        assert_eq!(tokens[8], Token::Indent);
        assert_eq!(tokens[9], Token::Item("CONTENT-TYPE".to_string()));
        assert_eq!(tokens[10], Token::Literal("application/json".to_string()));
        assert_eq!(tokens[11], Token::Expect);
        assert_eq!(tokens[12], Token::Indent);
        assert_eq!(tokens[13], Token::Status);
        assert_eq!(tokens[14], Token::Number(200));
        assert_eq!(tokens[15], Token::Indent);
        assert_eq!(tokens[16], Token::Body);
        assert_eq!(tokens[17], Token::Contains);
        assert_eq!(tokens[18], Token::Literal("\"username\": \"john\"".to_string()));
    }
}
