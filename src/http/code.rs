use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ApiResponseCode {
    Unknown = -1,
    Ok = 200,
    Created = 201,
    NoContent = 204,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    TooManyRequests = 429,
    InternalServerError = 500,
    ServiceUnavailable = 503,
}

impl ApiResponseCode {
    pub fn from_code(code: u16) -> Self {
        match code {
            200 => Self::Ok,
            201 => Self::Created,
            204 => Self::NoContent,
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            429 => Self::TooManyRequests,
            500 => Self::InternalServerError,
            503 => Self::ServiceUnavailable,
            _ => Self::Unknown,
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = u16::deserialize(deserializer)?;

        Ok(Self::from_code(code))
    }
}
