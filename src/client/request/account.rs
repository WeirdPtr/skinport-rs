use crate::{
    builder::request::account::AccountTransaction, client::Client, http::error::ApiErrorResponse,
    internal::models::account::ItemResponse, util::ordering::Ordering,
};

impl Client {
    pub async fn get_account_transactions(
        &self,
        page: Option<u64>,
        limit: Option<u8>,
        order: Option<Ordering>,
    ) -> Result<ItemResponse, ApiErrorResponse> {
        let mut request = AccountTransaction::default();

        if let Some(page) = page {
            request.page(page);
        }

        if let Some(limit) = limit {
            request.limit(limit);
        }

        if let Some(order) = order {
            request.order(order);
        }

        let url = api_url!("account/transactions");

        let response = self.authorized_get(url, &request.0).await?;

        let items = response.text_with_charset("utf-8").await;

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        let items = serde_json::from_str::<ItemResponse>(&items.unwrap());

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        Ok(items.unwrap())
    }
}
