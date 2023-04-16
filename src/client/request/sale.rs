use crate::{
    builder::request::sales::{SalesHistory, SalesOutOfStock},
    client::Client,
    http::error::ApiErrorResponse,
    internal::models::sale::{SalesHistoryResponse, SalesOutOfStockResponse},
    util::{currency::Currency, game::Game},
};

impl Client {
    pub async fn get_sales_history(
        &self,
        market_hash_name: impl ToString,
        game: Option<Game>,
        currency: Option<Currency>,
    ) -> Result<Vec<SalesHistoryResponse>, ApiErrorResponse> {
        let mut request = SalesHistory::default();

        request.market_hash_name(market_hash_name);

        if let Some(game) = game {
            request.app_id(game.identifier());
        }

        if let Some(currency) = currency {
            request.currency(currency);
        }

        let url = api_url!("sales/history");

        let response = self.get(url, &request.0).await?;

        let items = response.text_with_charset("utf-8").await;

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        let items = serde_json::from_str::<Vec<SalesHistoryResponse>>(&items.unwrap());

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        Ok(items.unwrap())
    }

    pub async fn get_sales_out_of_stock(
        &self,
        game: Option<Game>,
        currency: Option<Currency>,
    ) -> Result<Vec<SalesOutOfStockResponse>, ApiErrorResponse> {
        let mut request = SalesOutOfStock::default();

        if let Some(game) = game {
            request.app_id(game.identifier());
        }

        if let Some(currency) = currency {
            request.currency(currency);
        }

        let url = api_url!("sales/out-of-stock");

        let response = self.get(url, &request.0).await?;

        let items = response.text_with_charset("utf-8").await;

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        let items = serde_json::from_str::<Vec<SalesOutOfStockResponse>>(&items.unwrap());

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        Ok(items.unwrap())
    }
}
