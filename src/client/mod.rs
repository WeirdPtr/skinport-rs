use serde_json::Value;
use std::collections::HashMap;

use crate::{
    auth_header,
    http::{
        code::ApiResponseCode,
        error::{ApiError, ApiErrorResponse},
    },
    user_agent_header,
};

pub(crate) mod request;

pub struct Client {
    pub(crate) identifier: String,
    pub(crate) secret: String,
    pub(crate) client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            identifier: String::new(),
            secret: String::new(),
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
    pub fn new(identifier: impl ToString, secret: impl ToString) -> Self {
        Self {
            identifier: identifier.to_string(),
            secret: secret.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub fn set_identifier(&mut self, identifier: impl ToString) -> &mut Self {
        self.identifier = identifier.to_string();
        self
    }

    pub fn set_secret(&mut self, secret: impl ToString) -> &mut Self {
        self.secret = secret.to_string();
        self
    }

    async fn authorized_get(
        &self,
        url: String,
        request: &HashMap<&'static str, Value>,
    ) -> Result<reqwest::Response, ApiErrorResponse> {
        let auth_header = auth_header!(&self.identifier, &self.secret);

        let response = self
            .client
            .get(url)
            .header("User-Agent", user_agent_header!())
            .header("Authorization", auth_header)
            .json(request)
            .send()
            .await;

        self.transform_response(response).await
    }

    async fn get(
        &self,
        url: String,
        request: &HashMap<&'static str, Value>,
    ) -> Result<reqwest::Response, ApiErrorResponse> {
        let response = self
            .client
            .get(url)
            .header("User-Agent", user_agent_header!())
            .json(request)
            .send()
            .await;

        self.transform_response(response).await
    }

    async fn transform_response(
        &self,
        response: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<reqwest::Response, ApiErrorResponse> {
        let response = response.unwrap();

        let status_code = response.status().as_u16();

        let code = ApiResponseCode::from_code(status_code);

        if code != ApiResponseCode::Ok {
            let errors = response.json::<Vec<ApiError>>().await.unwrap_or(vec![]);
            let error = ApiErrorResponse::new(code, errors);
            return Err(error);
        }

        Ok(response)
    }
}
