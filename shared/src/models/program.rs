use crate::serializers::bson_datetime_serializer;
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Program {
    #[serde(rename = "_id")]
    #[schema(example = "60f7b3b3d4b3f3b3f3b3f3b3")]
    pub id: ObjectId,
    #[serde(rename = "owner_id")]
    #[schema(example = "60f7b3b3d4b3f3b3f3b3f3b3")]
    pub owner_id: String,
    #[serde(rename = "filename")]
    #[schema(example = "example.py")]
    pub filename: String,
    #[serde(rename = "code_url")]
    #[schema(example = "https://example.com/example.py")]
    pub code_url: String,
    #[serde(rename = "content_type")]
    #[schema(example = "text/plain")]
    pub content_type: String,
    #[serde(rename = "file_size")]
    #[schema(example = "1024")]
    pub file_size: i64,
    #[serde(rename = "input_type")]
    #[schema(example = "text/plain")]
    pub input_type: String,
    #[serde(rename = "output_type")]
    #[schema(example = "text/plain")]
    pub output_type: String,
    #[serde(rename = "upload_time", with = "bson_datetime_serializer")]
    #[schema(example = "2024-08-01T12:34:56Z")]
    pub upload_time: DateTime<Utc>,
    #[serde(rename = "update_time", with = "bson_datetime_serializer")]
    #[schema(example = "2024-08-01T12:34:56Z")]
    pub update_time: DateTime<Utc>,
    #[schema(example = "/path/to/save/example.py")]
    pub file_path: String,
    #[serde(rename = "file_hash")]
    #[schema(example = "example_hash")]
    pub file_hash: String,
}

// TODO: other models (Pipeline, ExecutionRecord, etc)
