use super::tokens::HttpMethod;

#[derive(Debug)]
pub struct TestDefinition {
    pub(crate) endpoint: String,
    pub(crate) method: HttpMethod,
    pub(crate) headers: Vec<HeaderNode>,
    pub(crate) body: Option<String>,
    pub(crate) query: Option<String>,
    pub(crate) expect: Vec<ExpectNode>,
    pub(crate) timeout: Option<u16>,
}

impl Default for TestDefinition {
    fn default() -> Self {
        Self {
            endpoint: "".to_string(),
            method: HttpMethod::None,
            headers: vec![],
            body: None,
            query: None,
            expect: vec![],
            timeout: None,
        }
    }
}

#[derive(Debug)]
pub enum ASTNode {
    // name, test definition
    TestDefinition(String, TestDefinition),
}

#[derive(Debug)]
pub(crate) struct HeaderNode {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum ExpectNode {
    Status(u16),
    Body(BodyExpectation),
}

#[derive(Debug)]
pub enum BodyExpectation {
    Equals(String),
    Contains(String),
}

#[derive(Debug, Default)]
pub struct TestFile {
    pub tests: Vec<ASTNode>,
}
