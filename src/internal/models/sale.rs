use serde::{Deserialize, Serialize};

use crate::util::currency::Currency;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sale {
    pub price: f64,
    pub wear_value: Option<u32>,
    pub sold_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesSummary {
    pub min: Option<f64>,
    pub max: Option<f64>,
    #[serde(rename = "avg")]
    pub average: Option<f64>,
    pub volume: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesHistoryResponse {
    pub market_hash_name: String,
    #[serde(
        serialize_with = "Currency::serialize",
        deserialize_with = "Currency::deserialize"
    )]
    pub currency: Currency,
    pub item_page: String,
    pub market_page: String,
    pub sales: Option<Vec<Sale>>,
    #[serde(rename = "last_7_days")]
    pub last_week: SalesSummary,
    #[serde(rename = "last_30_days")]
    pub last_month: SalesSummary,
    #[serde(rename = "last_90_days")]
    pub last_quarter: SalesSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOutOfStockResponse {
    pub market_hash_name: String,
    pub version: Option<String>,
    #[serde(
        serialize_with = "Currency::serialize",
        deserialize_with = "Currency::deserialize"
    )]
    pub currency: Currency,
    pub suggested_price: Option<f64>,
    #[serde(rename = "avg_sale_price")]
    pub average_sale_price: Option<f64>,
    #[serde(rename = "sales_last_90d")]
    pub sales_last_quarter: u64,
}
