pub mod additional;
pub mod errors;
pub mod simple_requester;

use crate::{dsl::tokens::HttpMethod, request::errors::RequestError};
use std::collections::HashMap;
use std::time::Duration;

pub struct HTTPResponse {
    pub status: u16,
    pub raw_body: String,
}

#[derive(Debug, Clone, Default)]
pub struct RequestConfig {
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub query: Option<HashMap<String, String>>,
    pub timeout: Option<Duration>,
}

impl RequestConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }

    pub fn with_query(mut self, query: HashMap<String, String>) -> Self {
        self.query = Some(query);
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

#[async_trait::async_trait]
pub trait Requester: Send + Sync {
    async fn send_request(
        &self,
        url: String,
        method: HttpMethod,
    ) -> Result<HTTPResponse, RequestError>;

    async fn send_request_with_config(
        &self,
        url: String,
        method: HttpMethod,
        config: RequestConfig,
    ) -> Result<HTTPResponse, RequestError>;
}
