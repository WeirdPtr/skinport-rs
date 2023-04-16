use serde::{Deserialize, Serialize};

use super::code::ApiResponseCode;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    #[serde(deserialize_with = "ApiResponseCode::deserialize")]
    pub code: ApiResponseCode,
    pub errors: Vec<ApiError>,
}

impl ApiErrorResponse {
    pub fn new(code: ApiResponseCode, errors: Vec<ApiError>) -> Self {
        Self { code, errors }
    }

    pub(crate) fn parse_items() -> Self {
        Self::new(
            ApiResponseCode::InternalServerError,
            vec![ApiError::new("parse:items", "Failed to parse items")],
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    #[serde(rename = "id")]
    pub identifier: String,
    pub message: String,
}

impl ApiError {
    pub fn new(identifier: impl ToString, message: impl ToString) -> Self {
        Self {
            identifier: identifier.to_string(),
            message: message.to_string(),
        }
    }
}
