use serde::{Deserialize, Deserializer, Serializer};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Currency {
    Unknown,
    AUD,
    BRL,
    CAD,
    CHF,
    CNY,
    CZK,
    DKK,
    EUR,
    GBP,
    HRK,
    NOK,
    PLN,
    RUB,
    SEK,
    TRY,
    USD,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_fmt = match self {
            Currency::AUD => "AUD",
            Currency::BRL => "BRL",
            Currency::CAD => "CAD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::CZK => "CZK",
            Currency::DKK => "DKK",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::HRK => "HRK",
            Currency::NOK => "NOK",
            Currency::PLN => "PLN",
            Currency::RUB => "RUB",
            Currency::SEK => "SEK",
            Currency::TRY => "TRY",
            Currency::USD => "USD",
            Self::Unknown => "Unknown",
        };

        write!(f, "{}", str_fmt)
    }
}

impl Currency {
    pub(crate) fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let currency_str = String::deserialize(deserializer)?;

        let currency = match currency_str.to_uppercase().as_str() {
            "AUD" => Currency::AUD,
            "BRL" => Currency::BRL,
            "CAD" => Currency::CAD,
            "CHF" => Currency::CHF,
            "CNY" => Currency::CNY,
            "CZK" => Currency::CZK,
            "DKK" => Currency::DKK,
            "EUR" => Currency::EUR,
            "GBP" => Currency::GBP,
            "HRK" => Currency::HRK,
            "NOK" => Currency::NOK,
            "PLN" => Currency::PLN,
            "RUB" => Currency::RUB,
            "SEK" => Currency::SEK,
            "TRY" => Currency::TRY,
            "USD" => Currency::USD,
            _ => Self::Unknown,
        };

        Ok(currency)
    }
}
