use bson::Document;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateProgramDto {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "example.py")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "https://example.com/example.py")]
    pub code_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "text/plain")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "1024")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "text/plain")]
    pub input_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "text/plain")]
    pub output_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(example = "https://example.com/example.py")]
    pub file_path: Option<String>,
}

impl UpdateProgramDto {
    pub fn build_update_document(&self) -> Document {
        let mut update_document = Document::new();

        if let Some(filename) = &self.filename {
            update_document.insert("filename", filename);
        }

        if let Some(code_url) = &self.code_url {
            update_document.insert("code_url", code_url);
        }

        if let Some(content_type) = &self.content_type {
            update_document.insert("content_type", content_type);
        }

        if let Some(file_size) = &self.file_size {
            update_document.insert("file_size", file_size);
        }

        if let Some(input_type) = &self.input_type {
            update_document.insert("input_type", input_type);
        }

        if let Some(output_type) = &self.output_type {
            update_document.insert("output_type", output_type);
        }

        if let Some(file_path) = &self.file_path {
            update_document.insert("file_path", file_path);
        }

        update_document
    }

}
