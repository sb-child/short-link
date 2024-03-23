pub mod challenge;
pub mod short_link;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestError {
    #[serde(rename = "cause")]
    pub cause: Option<String>,

    #[serde(rename = "error")]
    pub error: String,

    #[serde(rename = "errorMessage")]
    pub error_message: String,

    #[serde(skip)]
    pub status_code: axum::http::StatusCode,
}

pub type ErrorResponse = (axum::http::StatusCode, axum::Json<RequestError>);

impl RequestError {
    pub fn to_response(self) -> ErrorResponse {
        (self.status_code, axum::Json::from(self))
    }
}

pub trait ToError {
    fn to_error(&self) -> RequestError;
}
