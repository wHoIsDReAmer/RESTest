use crate::dsl::tokens::HttpMethod;
use crate::request::{HTTPResponse, Requester, RequestConfig, errors::RequestError};

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
        let config = RequestConfig::new();
        self.send_request_with_config(url, method, config).await
    }

    async fn send_request_with_config(
        &self,
        url: String,
        method: HttpMethod,
        config: RequestConfig,
    ) -> Result<HTTPResponse, RequestError> {
        // TODO: Implement proxy support with config

        self.requester.send_request_with_config(url, method, config).await
    }
}
