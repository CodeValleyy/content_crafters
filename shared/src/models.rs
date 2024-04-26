use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};


#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub owner_id: ObjectId,
    pub name: String,
    pub description: String,
    pub code_url: String,
    pub input_type: String,
    pub output_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// TODO: other models (Pipeline, ExecutionRecord, etc)
