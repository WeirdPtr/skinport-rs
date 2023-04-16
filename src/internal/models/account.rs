use serde::{Deserialize, Serialize};

use crate::util::{currency::Currency, locale::Locale, ordering::Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub page: u64,
    pub pages: u64,
    pub limit: u8,
    #[serde(
        serialize_with = "Ordering::serialize",
        deserialize_with = "Ordering::deserialize"
    )]
    pub order: Ordering,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub sale_id: u64,
    pub market_hash_name: String,
    #[serde(
        serialize_with = "Locale::serialize",
        deserialize_with = "Locale::deserialize"
    )]
    pub seller_country: Locale,
    #[serde(
        serialize_with = "Locale::serialize",
        deserialize_with = "Locale::deserialize"
    )]
    pub buyer_country: Locale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: u64,
    #[serde(rename = "type")]
    pub order_type: String,
    pub sub_type: Option<String>,
    pub status: String,
    pub amount: f32,
    pub fee: Option<f32>,
    #[serde(
        serialize_with = "Currency::serialize",
        deserialize_with = "Currency::deserialize"
    )]
    pub currency: Currency,
    pub items: Option<Vec<OrderItem>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemResponse {
    pub pagination: Pagination,
    pub data: Vec<OrderData>,
}
