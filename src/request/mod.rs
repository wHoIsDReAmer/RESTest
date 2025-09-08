pub mod errors;
pub mod simple_requester;

use crate::{dsl::tokens::HttpMethod, request::errors::RequestError};

pub(crate) struct Response {
    status: i32,
    raw_body: String,
}

#[async_trait::async_trait]
pub(crate) trait Requester {
    async fn send_request(url: String, method: HttpMethod) -> Result<Response, RequestError>;
}
