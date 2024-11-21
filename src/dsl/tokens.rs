#[allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    // Keywords
    Test,
    Endpoint,
    Method(HttpMethod),

    Headers,

    Expect,
    Body,
    Status,
    
    // Comparing
    Contains,
    Equals,

    // Literals
    Indent,
    Dedent,
    Literal(String),
    Number(u32),

    EOF
}

#[derive(Debug, PartialEq, Eq)]
pub enum HttpMethod {
    UNKNOWN,
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    OPTIONS,
    HEAD,
}

/*
DSL 예시

test "Foo"
endpoint "https://foofoofoo.com"
method GET
headers
    Authorization "Bearer test"
body "{\"id\": 123}"
expect
    status 200
    body equals "{\"message\": \"Hello, world!\"}"
*/