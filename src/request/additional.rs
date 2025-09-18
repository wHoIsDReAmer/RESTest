use crate::dsl::tokens::HttpMethod;
use crate::request::{HTTPResponse, Requester, errors::RequestError};

pub struct ProxyRequester {
    proxy: String,
    requester: Box<dyn Requester>,
}

#[async_trait::async_trait]
impl Requester for ProxyRequester {
    async fn send_request(
        &self,
        url: String,
        method: HttpMethod,
    ) -> Result<HTTPResponse, RequestError> {
        // TODO: Implement proxy support

        self.requester.send_request(url, method).await
    }
}
