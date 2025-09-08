use crate::{
    dsl::tokens::HttpMethod,
    request::{Requester, Response, errors::RequestError},
};

pub struct SimpleRequester {}

#[async_trait::async_trait]
impl Requester for SimpleRequester {
    async fn send_request(url: String, method: HttpMethod) -> Result<Response, RequestError> {
        let client = reqwest::Client::builder().build()?;

        let request = match method {
            HttpMethod::Get => client.get(url),
            HttpMethod::Post => client.post(url),
            _ => client.get(url),
        };

        todo!()
    }
}
