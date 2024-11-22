use super::tokens::HttpMethod;

/*
DSL 예시

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
enum ASTNode {
    Test(String),
    Endpoint(String),
    Method(HttpMethod),
    Headers(Vec<HeaderNode>),
    Body(String),
    Query(String),
    Expect(Vec<ExpectNode>),
    Timeout(u16),
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