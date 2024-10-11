#[allow(dead_code)]

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Test,
    Endpoint,
    Method,
    Headers,
    Expect,
    Body,
    Status,
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

#[derive(Debug, PartialEq, Eq)]
pub enum BodyExpectation {
    Equals(String),
    Contains(String),
    // TODO: Matches(String), For regex matching
    // Matches(String),
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