use std::collections::HashMap;

use serde_json::Value;

#[derive(Clone, Debug, Default)]
pub struct Items(pub HashMap<&'static str, Value>);

impl Items {
    pub fn app_id(&mut self, app_id: u32) -> &mut Self {
        self.0.insert("app_id", Value::from(app_id));
        self
    }

    pub fn currency(&mut self, currency: impl ToString) -> &mut Self {
        self.0.insert("currency", Value::from(currency.to_string()));
        self
    }

    pub fn is_tradable(&mut self, tradable: bool) -> &mut Self {
        self.0.insert("tradable", Value::from(tradable));
        self
    }
}
