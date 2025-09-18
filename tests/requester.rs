use restest::dsl::tokens::HttpMethod;
use restest::request::{Requester, RequestConfig, simple_requester::SimpleRequester};
use std::collections::HashMap;
use std::time::Duration;

#[tokio::test]
async fn test_simple_requester_get() {
    let requester = SimpleRequester {};
    let result = requester
        .send_request("https://httpbin.org/get".to_string(), HttpMethod::Get)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_requester_with_headers() {
    let requester = SimpleRequester {};
    let mut headers = HashMap::new();
    headers.insert("User-Agent".to_string(), "restest/1.0".to_string());
    headers.insert("Accept".to_string(), "application/json".to_string());

    let config = RequestConfig::new()
        .with_headers(headers);

    let result = requester
        .send_request_with_config(
            "https://httpbin.org/headers".to_string(),
            HttpMethod::Get,
            config
        )
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, 200);
}

#[tokio::test]
async fn test_requester_with_query() {
    let requester = SimpleRequester {};
    let mut query = HashMap::new();
    query.insert("page".to_string(), "1".to_string());
    query.insert("limit".to_string(), "10".to_string());

    let config = RequestConfig::new()
        .with_query(query);

    let result = requester
        .send_request_with_config(
            "https://httpbin.org/get".to_string(),
            HttpMethod::Get,
            config
        )
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.raw_body.contains("\"page\": \"1\""));
    assert!(response.raw_body.contains("\"limit\": \"10\""));
}

#[tokio::test]
async fn test_requester_with_body() {
    let requester = SimpleRequester {};
    let body = r#"{"name": "test", "value": 123}"#.to_string();

    let config = RequestConfig::new()
        .with_body(body.clone());

    let result = requester
        .send_request_with_config(
            "https://httpbin.org/post".to_string(),
            HttpMethod::Post,
            config
        )
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, 200);
    assert!(response.raw_body.contains(&body));
}

#[tokio::test]
async fn test_requester_with_timeout() {
    let requester = SimpleRequester {};
    let config = RequestConfig::new()
        .with_timeout(Duration::from_millis(1));

    let result = requester
        .send_request_with_config(
            "https://httpbin.org/delay/5".to_string(),
            HttpMethod::Get,
            config
        )
        .await;

    assert!(result.is_err());
}
