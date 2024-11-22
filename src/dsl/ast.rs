use super::tokens::HttpMethod;

/*
test "Foo"
endpoint "https://foofoofoo.com"
method GET
timeout 1000
headers
    Authorization "Bearer test"
body "{\"id\": 123}"
expect
    status 200
    body equals "{\"message\": \"Hello, world!\"}"
*/

#[derive(Debug)]
pub(crate) struct TestDefinition {
    endpoint: String,
    method: HttpMethod,
    headers: Vec<HeaderNode>,
    body: Option<String>,
    query: Option<String>,
    expect: Vec<ExpectNode>,
    timeout: Option<u16>,
}

impl Default for TestDefinition {
    fn default() -> Self {
        Self {
            endpoint: "".to_string(),
            method: HttpMethod::NONE,
            headers: vec![],
            body: None,
            query: None,
            expect: vec![],
            timeout: None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ASTNode {
    // name, test definition
    TestDefinition(String, TestDefinition),
}

#[derive(Debug)]
struct HeaderNode {
    key: String,
    value: String,
}

#[derive(Debug)]
enum ExpectNode {
    Status(u16),
    Body(BodyExpectation),
}

#[derive(Debug)]
enum BodyExpectation {
    Equals(String),
    Contains(String),
}

#[derive(Debug)]
pub struct TestFile {
    pub tests: Vec<ASTNode>,
}