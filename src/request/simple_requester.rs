use crate::{
    dsl::tokens::HttpMethod,
    request::{HTTPResponse, Requester, errors::RequestError},
};

pub struct SimpleRequester {}

#[async_trait::async_trait]
impl Requester for SimpleRequester {
    async fn send_request(
        &self,
        url: String,
        method: HttpMethod,
    ) -> Result<HTTPResponse, RequestError> {
        let client = reqwest::Client::builder().build()?;

        let request = match method {
            HttpMethod::Get => client.get(url),
            HttpMethod::Post => client.post(url),
            _ => client.get(url),
        };

        let response = request.send().await?;
        let status = response.status();
        let raw_body = response.text().await?;

        Ok(HTTPResponse {
            status: status.as_u16(),
            raw_body: raw_body,
        })
    }
}
