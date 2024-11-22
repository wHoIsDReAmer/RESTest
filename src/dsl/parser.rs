use super::tokens::Token;

pub struct Parser {
    tokens: Vec<Token>,
}

// impl Parser {
//     pub fn parse(&self) -> Result<ASTNode, String> {
//         Ok(ASTNode::Test("".to_string()))
//     }
// }