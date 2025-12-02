use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Request timed out")]
    Timeout,
    #[error("Reqwest module error")]
    ReqwestError(#[from] reqwest::Error),
}
