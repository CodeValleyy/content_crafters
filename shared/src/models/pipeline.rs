use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Pipeline {
    #[serde(rename = "_id")]
    #[schema(example = "60f7b3b3d4b3f3b3f3b3f3b3")]
    pub id: ObjectId,

    #[serde(rename = "owner_id")]
    #[schema(example = "121")]
    pub owner_id: i32,

    #[serde(rename = "name")]
    #[schema(example = "example_pipeline")]
    pub name: String,

    #[serde(rename = "description")]
    #[schema(example = "example_description")]
    pub description: String,

    #[serde(rename = "steps")]
    #[schema(example = json!(vec![ObjectId::new().to_string(), ObjectId::new().to_string()]))]
    pub steps: Vec<String>,

    #[serde(rename = "created_date")]
    #[schema(example = json!(DateTime::<Utc>::from(Utc::now())))]
    pub created_date: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePipeline {
    #[serde(rename = "owner_id")]
    #[schema(example = "121")]
    pub owner_id: i32,

    #[serde(rename = "name")]
    #[schema(example = "example_pipeline")]
    pub name: String,

    #[serde(rename = "description")]
    #[schema(example = "example_description")]
    pub description: String,

    #[serde(rename = "steps")]
    #[schema(example = json!(vec![ObjectId::new().to_string(), ObjectId::new().to_string()]))]
    pub steps: Vec<String>,
}

impl From<CreatePipeline> for Pipeline {
    fn from(create: CreatePipeline) -> Self {
        Pipeline {
            id: ObjectId::new(),
            owner_id: create.owner_id,
            name: create.name,
            description: create.description,
            steps: create.steps,
            created_date: Utc::now().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePipeline {
    #[serde(rename = "name")]
    #[schema(example = "example_pipeline")]
    pub name: String,

    #[serde(rename = "description")]
    #[schema(example = "example_description")]
    pub description: String,

    #[serde(rename = "steps")]
    #[schema(example = json!(vec![ObjectId::new().to_string(), ObjectId::new().to_string()]))]
    pub steps: Vec<String>,
}

impl UpdatePipeline {
    pub fn build_update_document(&self) -> bson::Document {
        let mut update_document = bson::Document::new();

        if !self.name.is_empty() {
            update_document.insert("name", self.name.clone());
        }

        if !self.description.is_empty() {
            update_document.insert("description", self.description.clone());
        }

        if !self.steps.is_empty() {
            update_document.insert("steps", self.steps.clone());
        }

        update_document
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ExecutionRecord {
    #[serde(rename = "_id")]
    #[schema(example = "60f7b3b3d4b3f3b3f3b3f3b3")]
    pub id: ObjectId,

    #[serde(rename = "pipeline_id")]
    #[schema(example = "60f7b3b3d4b3f3b3f3b3f3b3")]
    pub pipeline_id: ObjectId,

    #[serde(rename = "execution_time")]
    #[schema(example = json!(DateTime::<Utc>::from(Utc::now())))]
    pub execution_time: DateTime<Utc>,

    #[serde(rename = "status")]
    #[schema(example = "success")]
    pub status: String,

    #[serde(rename = "output")]
    #[schema(example = "output")]
    pub output: String,
}
