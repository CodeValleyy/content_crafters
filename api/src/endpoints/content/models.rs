use actix_multipart::Multipart;
use utoipa::ToSchema;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(ToSchema)]
pub struct UploadFile {
    pub file: Multipart,
}

#[derive(serde::Deserialize)]
pub struct ContentId {
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ContentDetails {
    #[schema(example = "\"1234abcd\"")]
    pub id: String,

    #[schema(example = "\"Super Cool Program\"")]
    pub title: String,

    #[schema(example = "\"This program transforms text files into HTML.\"")]
    pub description: String,

    #[schema(example = "\"text/html\"")]
    pub output_type: String,

    #[schema(example = "\"text/plain\"")]
    pub input_type: String,

    #[schema(example = "\"John Doe\"")]
    pub author: String,

    #[schema(example = "vec![\"utility\", \"transformation\", \"text processing\"]")]
    pub tags: Vec<String>,

    #[schema(example = "\"v1.2.0\"")]
    pub version: String,

    #[schema(example = "vec![\"comment1\", \"comment2\"]")]
    pub comments: Vec<String>,

    #[schema(example = "42")]
    pub likes: i32,

    #[schema(example = "true")]
    pub editable: bool,
/* 
    #[schema(example = "2021-08-01T12:34:56Z")]
    pub created_at: DateTime<chrono::Utc>,

    #[schema(example = "2021-08-01T12:34:56Z")]
    pub updated_at: DateTime<chrono::Utc>,
*/
}

impl ContentDetails {


    pub fn new(id: String, title: String, description: String, output_type: String, input_type: String, 
        author: String, tags: Vec<String>, version: String, comments: Vec<String>, likes: i32, 
        editable: bool) -> ContentDetails {
        ContentDetails {
            id,
            title,
            description,
            output_type,
            input_type,
            author,
            tags,
            version,
            comments,
            likes,
            editable
        }

        /* TODO: Add the following fields:
            created_at,
            updated_at,
        */
    }
}