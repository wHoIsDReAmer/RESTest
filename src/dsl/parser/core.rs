use super::errors::ParseError;
use crate::dsl::prelude::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.current);
        self.current += 1;
        token
    }

    fn expect(&mut self, token: Token) -> Result<&Token, ParseError> {
        match self.peek() {
            Some(t) if t == &token => {
                self.advance();
                Ok(self.peek().unwrap())
            }
            _ => Err(ParseError::new("token not found")),
        }
    }

    pub fn parse(&mut self) -> Result<TestFile, ParseError> {
        let mut test_file = TestFile::default();

        let mut current_ast_node: Option<ASTNode> = None;

        while let Some(token) = self.peek().cloned() {
            match token {
                Token::Test => {
                    self.advance();

                    match self.peek() {
                        Some(Token::Literal(name)) => {
                            if let Some(test_def) = current_ast_node.take() {
                                test_file.tests.push(test_def);
                            }
                            current_ast_node = Some(ASTNode::TestDefinition(
                                name.clone(),
                                TestDefinition::default(),
                            ));
                        }
                        _ => return Err(ParseError::new("expected test name")),
                    }
                }
                Token::Endpoint => {
                    self.advance();

                    match self.peek() {
                        Some(Token::Literal(name)) => {
                            let test_def = Self::current_test_def(&mut current_ast_node)?;
                            test_def.endpoint = name.clone();
                            self.advance();
                        }
                        _ => return Err(ParseError::new("expected endpoint name")),
                    }
                }
                Token::Method(method) => {
                    self.advance();
                    let test_def = Self::current_test_def(&mut current_ast_node)?;
                    test_def.method = method;
                }
                Token::Headers => {
                    self.advance();
                    let test_def = Self::current_test_def(&mut current_ast_node)?;
                    self.parse_headers(test_def)?;
                }
                Token::Body => {
                    self.advance();
                    let test_def = Self::current_test_def(&mut current_ast_node)?;
                    match self.peek() {
                        Some(Token::Literal(body)) => {
                            test_def.body = Some(body.clone());
                            self.advance();
                        }
                        _ => return Err(ParseError::new("expected body literal")),
                    }
                }
                Token::Expect => {
                    self.advance();
                    let test_def = Self::current_test_def(&mut current_ast_node)?;
                    self.parse_expectations(test_def)?;
                }

                _ => {
                    self.advance();
                }
            }
        }

        // 마지막 current_ast_node가 있으면 추가
        if let Some(test_def) = current_ast_node.take() {
            test_file.tests.push(test_def);
        }

        Ok(test_file)
    }

    fn current_test_def(
        current_ast_node: &mut Option<ASTNode>,
    ) -> Result<&mut TestDefinition, ParseError> {
        match current_ast_node.as_mut() {
            Some(ASTNode::TestDefinition(_, test_def)) => Ok(test_def),
            _ => Err(ParseError::new("no test definition found")),
        }
    }

    fn parse_headers(&mut self, test_def: &mut TestDefinition) -> Result<(), ParseError> {
        loop {
            match self.peek() {
                Some(Token::Indent) => {
                    self.advance();
                }
                Some(Token::Item(key)) => {
                    let key = key.clone();
                    self.advance();
                    match self.peek() {
                        Some(Token::Literal(value)) => {
                            test_def.headers.push(HeaderNode {
                                key,
                                value: value.clone(),
                            });
                            self.advance();
                        }
                        _ => return Err(ParseError::new("expected header value")),
                    }
                }
                Some(Token::Test)
                | Some(Token::Endpoint)
                | Some(Token::Expect)
                | Some(Token::Body)
                | Some(Token::Method(_))
                | Some(Token::Status)
                | Some(Token::Contains)
                | Some(Token::Equals)
                | Some(Token::EOF) => break,
                Some(_) => {
                    self.advance();
                }
                None => break,
            }
        }
        Ok(())
    }

    fn parse_expectations(&mut self, test_def: &mut TestDefinition) -> Result<(), ParseError> {
        loop {
            match self.peek() {
                Some(Token::Indent) => {
                    self.advance();
                }
                Some(Token::Status) => {
                    self.advance();
                    match self.peek() {
                        Some(Token::Number(code)) => {
                            test_def.expect.push(ExpectNode::Status(*code as u16));
                            self.advance();
                        }
                        _ => return Err(ParseError::new("expected status code")),
                    }
                }
                Some(Token::Body) => {
                    self.advance();
                    match self.peek() {
                        Some(Token::Contains) => {
                            self.advance();
                            match self.peek() {
                                Some(Token::Literal(text)) => {
                                    test_def.expect.push(ExpectNode::Body(
                                        BodyExpectation::Contains(text.clone()),
                                    ));
                                    self.advance();
                                }
                                _ => return Err(ParseError::new("expected body text")),
                            }
                        }
                        Some(Token::Equals) => {
                            self.advance();
                            match self.peek() {
                                Some(Token::Literal(text)) => {
                                    test_def.expect.push(ExpectNode::Body(
                                        BodyExpectation::Equals(text.clone()),
                                    ));
                                    self.advance();
                                }
                                _ => return Err(ParseError::new("expected body text")),
                            }
                        }
                        _ => return Err(ParseError::new("expected body comparator")),
                    }
                }
                Some(Token::Test)
                | Some(Token::Endpoint)
                | Some(Token::Headers)
                | Some(Token::Method(_))
                | Some(Token::EOF) => break,
                Some(_) => {
                    self.advance();
                }
                None => break,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::Test,
            Token::Literal("test".to_string()),
            Token::Endpoint,
            Token::Literal("endpoint".to_string()),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("{result:?}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_no_test_name() {
        let tokens = vec![
            Token::Test,
            Token::Endpoint,
            Token::Literal("endpoint".to_string()),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "expected test name");
    }

    #[test]
    fn test_parse_no_endpoint_name() {
        let tokens = vec![
            Token::Test,
            Token::Literal("test".to_string()),
            Token::Endpoint,
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "expected endpoint name");
    }

    #[test]
    fn test_parse_endpoint_without_test() {
        let tokens = vec![
            Token::Endpoint,
            Token::Literal("endpoint".to_string()),
            Token::EOF,
        ];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "no test definition found");
    }
}
