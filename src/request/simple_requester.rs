use crate::{
    dsl::tokens::HttpMethod,
    request::{HTTPResponse, Requester, RequestConfig, errors::RequestError},
};
use reqwest::Method;

pub struct SimpleRequester {}

#[async_trait::async_trait]
impl Requester for SimpleRequester {
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
        let mut client_builder = reqwest::Client::builder();

        if let Some(timeout) = config.timeout {
            client_builder = client_builder.timeout(timeout);
        }

        let client = client_builder.build()?;

        let mut request = match method {
            HttpMethod::Get => client.get(&url),
            HttpMethod::Post => client.post(&url),
            HttpMethod::Put => client.put(&url),
            HttpMethod::Delete => client.delete(&url),
            HttpMethod::Patch => client.patch(&url),
            HttpMethod::Options => client.request(Method::OPTIONS, &url),
            HttpMethod::Head => client.head(&url),
            _ => client.get(&url),
        };

        for (key, value) in config.headers {
            request = request.header(key, value);
        }

        if let Some(body) = config.body {
            request = request.body(body);
        }

        if let Some(query_params) = config.query {
            request = request.query(&query_params);
        }

        let response = request.send().await?;
        let status = response.status();
        let raw_body = response.text().await?;

        Ok(HTTPResponse {
            status: status.as_u16(),
            raw_body,
        })
    }
}
