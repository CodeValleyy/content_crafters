use chrono::{DateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serialize, Serializer};

pub mod bson_datetime_serializer {
    use super::*;

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let bson_date = mongodb::bson::DateTime::from_chrono(*date);
        bson_date.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bson_date = mongodb::bson::DateTime::deserialize(deserializer)?;
        Ok(bson_date.to_chrono())
    }
}
