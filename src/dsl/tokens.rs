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

    // All else tokens are treated as items
    Item(String),

    EOF,
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
    NONE,
}
