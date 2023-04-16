use serde::{Deserialize, Deserializer, Serializer};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Locale {
    Unknown,
    EN,
    DE,
    RU,
    FR,
    ZH,
    NL,
    FI,
    ES,
    TR,
}

impl std::fmt::Display for Locale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_fmt = match self {
            Locale::EN => "en",
            Locale::DE => "de",
            Locale::RU => "ru",
            Locale::FR => "fr",
            Locale::ZH => "zh",
            Locale::NL => "nl",
            Locale::FI => "fi",
            Locale::ES => "es",
            Locale::TR => "tr",
            Self::Unknown => "Unknown",
        };

        write!(f, "{}", str_fmt)
    }
}

impl Locale {
    pub fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let locale_str = String::deserialize(deserializer)?;

        let locale = match locale_str.to_lowercase().as_str() {
            "en" => Locale::EN,
            "de" => Locale::DE,
            "ru" => Locale::RU,
            "fr" => Locale::FR,
            "zh" => Locale::ZH,
            "nl" => Locale::NL,
            "fi" => Locale::FI,
            "es" => Locale::ES,
            "tr" => Locale::TR,
            _ => Locale::Unknown,
        };

        Ok(locale)
    }
}
