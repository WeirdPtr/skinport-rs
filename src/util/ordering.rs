use serde::{Deserialize, Deserializer, Serializer};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ordering {
    Ascending,
    Descending,
}

impl std::fmt::Display for Ordering {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_fmt = match self {
            Ordering::Ascending => "asc",
            Ordering::Descending => "desc",
        };

        write!(f, "{}", str_fmt)
    }
}

impl Ordering {
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
        let ordering_str = String::deserialize(deserializer)?;

        let ordering = match ordering_str.as_str() {
            "asc" => Ordering::Ascending,
            "desc" => Ordering::Descending,
            _ => Ordering::Descending,
        };

        Ok(ordering)
    }
}
