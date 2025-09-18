use restest::dsl::tokens::HttpMethod;
use restest::request::{Requester, simple_requester::SimpleRequester};

#[tokio::test]
async fn test_simple_requester_get() {
    let requester = SimpleRequester {};
    let result = requester
        .send_request("https://httpbin.org/get".to_string(), HttpMethod::Get)
        .await;

    assert!(result.is_ok());
}
