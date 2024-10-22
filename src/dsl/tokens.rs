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
의사 코드: RESTest의 DSL 정의

test "테스트 이름"
endpoint "엔드포인트 이름"
method [메서드 이름]
headers
    Authorization "Bearer test"
body "{\"message\": \"Hello, world!\"}"
expect
    status 200
    body equals "{\"message\": \"Hello, world!\"}" // auto trimming
*/