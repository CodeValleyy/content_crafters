use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse};
use futures::StreamExt;
use log::info;
use mongodb::bson::doc;
use reqwest::Client;
use serde_json::json;
use shared::database::api_response::ApiResponse;

use utils::error::UploadError;

use crate::utils::{self, field_parser::parse_id};

#[utoipa::path(
    post,
    path = "/group/upload",
    tag = "group",
    responses(
        (status = 201, description = "Group Avatar uploaded successfully", body = String),
        (status = 400, description = "Bad Request"),
    ),
    request_body(
        content_type = "multipart/form-data",
        content = UploadGroupFile
    ),
)]
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let firebase_bucket =
        std::env::var("FIREBASE_STORAGE_BUCKET").expect("FIREBASE_STORAGE_BUCKET must be set");
    let client = Client::new();

    let mut group_id: Option<i32> = None;
    let mut owner_id: Option<i32> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;

    while let Some(item) = payload.next().await {
        let field = item?;
        let field_name = field.name().to_string();
        match field.name() {
            "file" => {
                let (name, content_type_str, data) = process_file_field(field).await?;
                filename = Some(name);
                content_type = Some(content_type_str);
                file_data = Some(data);
            }
            "group_id" => group_id = Some(parse_id(&field_name, field).await?),
            "owner_id" => owner_id = Some(parse_id(&field_name, field).await?),
            _ => {}
        }
    }

    if let (Some(file_data), Some(owner_id), Some(group_id), Some(filename), Some(content_type)) =
        (file_data, owner_id, group_id, filename, content_type)
    {
        return update(
            owner_id,
            group_id,
            file_data,
            &filename,
            &content_type,
            &client,
            &firebase_bucket,
        )
        .await;
    }

    Ok(HttpResponse::BadRequest()
        .json(json!({"message": "No files, group_id or owner_id were provided."})))
}

async fn update(
    owner_id: i32,
    group_id: i32,
    file_data: Vec<u8>,
    filename: &str,
    content_type: &str,
    client: &Client,
    firebase_bucket: &str,
) -> Result<HttpResponse, Error> {
    let (base_filename, extension) = match filename.rsplit_once('.') {
        Some((_, ext)) => (format!("{}-{}", group_id, owner_id), ext),
        None => (filename.to_string(), ""),
    };

    let filename_with_extension = if extension.is_empty() {
        format!("{}", base_filename)
    } else {
        format!("{}.{}", base_filename, extension)
    };

    let file_path: String = format!("group%2F{}%2F{}", owner_id, filename_with_extension);
    let upload_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o?name={}",
        firebase_bucket, file_path
    );

    info!("Uploading file: {:?}", filename_with_extension);

    let response = client
        .post(&upload_url)
        .header("Content-Type", content_type)
        .body(file_data)
        .send()
        .await;

    let file_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o/{}?alt=media",
        firebase_bucket, file_path
    );

    match response {
        Ok(res) if res.status().is_success() => {
            let response_data = ApiResponse::new(
                "File uploaded successfully",
                Some("".to_string()),
                Some(file_url),
            );
            Ok(HttpResponse::Created().json(response_data))
        }
        Ok(res) => {
            let error_message = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Ok(HttpResponse::BadRequest().json(json!({
                "message": format!("Error uploading to Firebase: {}", error_message)
            })))
        }
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "message": format!("Error uploading to Firebase: {}", e)
        }))),
    }
}

async fn process_file_field(
    field: actix_multipart::Field,
) -> Result<(String, String, Vec<u8>), Error> {
    let content_disposition = field.content_disposition().clone();
    let filename = match content_disposition
        .get_filename()
        .map(|name| name.to_string())
    {
        Some(name) if !name.is_empty() => name,
        _ => {
            return Err(UploadError::BadRequest("No filename provided".into()).into());
        }
    };

    let content_type = field
        .content_type()
        .map(|mime| mime.to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    let mut data = Vec::new();
    let mut field = field;
    while let Some(chunk) = field.next().await {
        data.extend_from_slice(&chunk?);
    }

    Ok((filename, content_type, data))
}
