use serde::{Deserialize, Serialize};

use crate::util::currency::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResponse {
    pub market_hash_name: String,
    #[serde(
        serialize_with = "Currency::serialize",
        deserialize_with = "Currency::deserialize"
    )]
    pub currency: Currency,
    pub suggested_price: Option<f64>,
    pub item_page: String,
    pub market_page: String,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
    pub mean_price: Option<f64>,
    pub quantity: u32,
    pub created_at: i64,
    pub updated_at: i64,
}
