use crate::{
    builder::request::items::Items,
    client::Client,
    http::error::ApiErrorResponse,
    internal::models::item::ItemResponse,
    util::{currency::Currency, game::Game},
};

impl Client {
    pub async fn get_items(
        &self,
        game: Option<Game>,
        currency: Option<Currency>,
        tradable: Option<bool>,
    ) -> Result<Vec<ItemResponse>, ApiErrorResponse> {
        let mut request = Items::default();

        if let Some(game) = game {
            request.app_id(game.identifier());
        }

        if let Some(currency) = currency {
            request.currency(currency);
        }

        if let Some(tradable) = tradable {
            request.is_tradable(tradable);
        }

        let url = api_url!("items");

        let response = self.get(url, &request.0).await?;

        let items = response.text_with_charset("utf-8").await;

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        let items = serde_json::from_str::<Vec<ItemResponse>>(&items.unwrap());

        if items.is_err() {
            return Err(ApiErrorResponse::parse_items());
        }

        Ok(items.unwrap())
    }
}
