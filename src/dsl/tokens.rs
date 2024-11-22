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

    // 다른 모든 토큰들은 이 토큰으로 취급됨
    Item(String),

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
    NONE,
}
