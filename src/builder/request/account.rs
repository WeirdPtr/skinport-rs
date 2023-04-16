use std::collections::HashMap;

use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct AccountTransaction(pub HashMap<&'static str, Value>);

impl AccountTransaction {
    pub fn page(&mut self, page: u64) -> &mut Self {
        self.0.insert("page", Value::from(page));
        self
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        self.0.insert("limit", Value::from(limit));
        self
    }

    pub fn order(&mut self, order: impl ToString) -> &mut Self {
        self.0.insert("order", Value::from(order.to_string()));
        self
    }
}
