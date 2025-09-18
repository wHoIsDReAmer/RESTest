pub mod additional;
pub mod errors;
pub mod simple_requester;

use crate::{dsl::tokens::HttpMethod, request::errors::RequestError};

pub struct HTTPResponse {
    pub(crate) status: u16,
    pub(crate) raw_body: String,
}

#[async_trait::async_trait]
pub trait Requester: Send + Sync {
    async fn send_request(
        &self,
        url: String,
        method: HttpMethod,
    ) -> Result<HTTPResponse, RequestError>;
}
