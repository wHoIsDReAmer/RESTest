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

        while let Some(token) = self.peek() {
            match token {
                Token::Test => {
                    self.advance();

                    match self.peek() {
                        Some(Token::Literal(name)) => {
                            if let Some(test_def) = current_ast_node.take() {
                                test_file.tests.push(test_def);
                            }
                            current_ast_node = Some(ASTNode::TestDefinition(name.clone(), TestDefinition::default()));
                        }
                        _ => return Err(ParseError::new("expected test name")),
                    }
                },
                Token::Endpoint => {
                    self.advance();

                    match self.peek() {
                        Some(Token::Literal(name)) => {
                            match current_ast_node.as_mut() {
                                Some(ASTNode::TestDefinition(_, test_def)) => {
                                    test_def.endpoint = name.clone();
                                }
                                _ => return Err(ParseError::new("no test definition found")),
                            }
                        }
                        _ => return Err(ParseError::new("expected endpoint name")),
                    }
                },

                _ => { self.advance(); }
            }
        }

        // 마지막 current_ast_node가 있으면 추가
        if let Some(test_def) = current_ast_node.take() {
            test_file.tests.push(test_def);
        }

        Ok(test_file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![Token::Test, Token::Literal("test".to_string()), Token::Endpoint, Token::Literal("endpoint".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_no_test_name() {
        let tokens = vec![Token::Test, Token::Endpoint, Token::Literal("endpoint".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "expected test name");
    }

    #[test]
    fn test_parse_no_endpoint_name() {
        let tokens = vec![Token::Test, Token::Literal("test".to_string()), Token::Endpoint, Token::EOF];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "expected endpoint name");
    }

    #[test]
    fn test_parse_endpoint_without_test() {
        let tokens = vec![Token::Endpoint, Token::Literal("endpoint".to_string()), Token::EOF];
        let mut parser = Parser::new(tokens);
        let result = parser.parse();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "no test definition found");
    }
}