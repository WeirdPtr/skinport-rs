use std::collections::HashMap;

use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct SalesHistory(pub HashMap<&'static str, Value>);

impl SalesHistory {
    pub fn market_hash_name(&mut self, market_hash_name: impl ToString) -> &mut Self {
        self.0.insert(
            "market_hash_name",
            Value::from(market_hash_name.to_string()),
        );
        self
    }

    pub fn app_id(&mut self, app_id: u32) -> &mut Self {
        self.0.insert("app_id", Value::from(app_id));
        self
    }

    pub fn currency(&mut self, currency: impl ToString) -> &mut Self {
        self.0.insert("currency", Value::from(currency.to_string()));
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct SalesOutOfStock(pub HashMap<&'static str, Value>);

impl SalesOutOfStock {
    pub fn app_id(&mut self, app_id: u32) -> &mut Self {
        self.0.insert("app_id", Value::from(app_id));
        self
    }

    pub fn currency(&mut self, currency: impl ToString) -> &mut Self {
        self.0.insert("currency", Value::from(currency.to_string()));
        self
    }
}
