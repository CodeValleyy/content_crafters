use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use bson::{oid::ObjectId, Document};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use log::info;
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    Database,
};
use reqwest::Client;
use serde_json::json;
use shared::database::api_response::ApiResponse;
use std::time::SystemTime;

use utils::error::UploadError;

use crate::utils::{self, firebase::delete_file_from_firebase};

#[utoipa::path(
    post,
    path = "/content/upload",
    tag = "content",
    responses(
        (status = 201, description = "Content uploaded successfully", body = String),
        (status = 400, description = "Bad Request"),
    ),
    request_body(
        content_type = "multipart/form-data",
        content = UploadFile
    ),
)]
pub async fn upload(
    db: web::Data<Database>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let firebase_bucket =
        std::env::var("FIREBASE_STORAGE_BUCKET").expect("FIREBASE_STORAGE_BUCKET must be set");
    let client = Client::new();

    let mut owner_id: Option<i32> = None;
    let mut file_data: Option<Vec<u8>> = None;
    let mut filename: Option<String> = None;
    let mut content_type: Option<String> = None;

    while let Some(item) = payload.next().await {
        let field = item?;
        match field.name() {
            "file" => {
                let (name, content_type_str, data) = process_file_field(field).await?;
                filename = Some(name);
                content_type = Some(content_type_str);
                file_data = Some(data);
            }
            "owner_id" => {
                owner_id = Some(parse_owner_id(field).await?);
            }
            _ => {}
        }
    }

    if let (Some(file_data), Some(owner_id), Some(filename), Some(content_type)) =
        (file_data, owner_id, filename, content_type)
    {
        return update(
            owner_id,
            file_data,
            &filename,
            &content_type,
            db,
            &client,
            &firebase_bucket,
        )
        .await;
    }

    Ok(HttpResponse::BadRequest().json(json!({"message": "No files or owner_id were provided."})))
}

async fn update(
    owner_id: i32,
    file_data: Vec<u8>,
    filename: &str,
    content_type: &str,
    db: web::Data<Database>,
    client: &Client,
    firebase_bucket: &str,
) -> Result<HttpResponse, Error> {
    let collection: mongodb::Collection<Document> = db.collection::<Document>("programs");

    let base_filename = match filename.rsplit_once('.') {
        Some((base, _)) => base,
        None => filename,
    };
    let regex_pattern = format!("^{}", regex::escape(base_filename));
    info!("Regex pattern: {:?}", regex_pattern);

    let existing_file: Option<bson::Document> = collection
        .find_one(
            doc! {
                "owner_id": owner_id,
                "filename": { "$regex": regex_pattern, "$options": "i" }
            },
            None,
        )
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    info!("Existing file: {:?}", existing_file);
    let file_size = file_data.len();
    let mut file_id = ObjectId::new();
    let upload_time: DateTime<Utc> = Utc::now();
    let bson_upload_time = BsonDateTime::from(SystemTime::from(upload_time));
    let timestamp = upload_time.timestamp_millis();
    let (base_filename, extension) = match filename.rsplit_once('.') {
        Some((base, ext)) => (base, ext),
        None => (filename, ""),
    };
    if let Some(existing_file) = existing_file {
        let existing_file_path = existing_file.get_str("file_path").map_err(|e| {
            log::error!("Error getting file_path: {:?}", e);
            actix_web::error::ErrorInternalServerError("Error getting file_path")
        })?;
        file_id = existing_file
            .get_object_id("_id")
            .map_err(|e| {
                log::error!("Error getting _id: {:?}", e);
                actix_web::error::ErrorInternalServerError("Error getting _id")
            })?
            .clone();
        delete_file_from_firebase(client, firebase_bucket, existing_file_path).await?;
    }

    let filename_with_timestamp = if extension.is_empty() {
        format!("{}-{}-{}", base_filename, file_id, timestamp)
    } else {
        format!("{}-{}-{}.{}", base_filename, file_id, timestamp, extension)
    };

    let file_path: String = format!("content%2F{}%2F{}", owner_id, filename_with_timestamp);
    let upload_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o?name={}",
        firebase_bucket, file_path
    );

    info!("Uploading file: {:?}", filename_with_timestamp);

    let response = client
        .post(&upload_url)
        .header("Content-Type", content_type)
        .body(file_data)
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => {
            save_metadata_to_db(
                db,
                owner_id,
                filename_with_timestamp.to_string(),
                firebase_bucket,
                file_path,
                content_type.to_string(),
                file_size as i64,
                bson_upload_time,
                file_id,
            )
            .await
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

async fn save_metadata_to_db(
    db: web::Data<Database>,
    owner_id: i32,
    filename: String,
    firebase_bucket: &str,
    file_path: String,
    content_type: String,
    file_size: i64,
    bson_upload_time: BsonDateTime,
    file_id: ObjectId,
) -> Result<HttpResponse, Error> {
    let code_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o/{}?alt=media",
        firebase_bucket, file_path
    );

    let collection = db.collection("programs");

    // Vérifier si le document existe déjà
    let existing_file = collection
        .find_one(doc! { "_id": file_id }, None)
        .await
        .map_err(|e| {
            log::error!("Database error: {:?}", e);
            actix_web::error::ErrorInternalServerError("Database error")
        })?;

    if let Some(_) = existing_file {
        let metadata = doc! {
            "owner_id": owner_id,
            "filename": filename,
            "code_url": &code_url,
            "content_type": content_type,
            "file_size": file_size,
            "update_time": bson_upload_time,
            "file_path": file_path,
            "file_hash": "example_hash", // TODO: get an algorithm to calculate the file hash (MD5, SHA256, etc.)
        };

        let update_result = collection
            .update_one(doc! { "_id": file_id }, doc! { "$set": metadata }, None)
            .await
            .map_err(|e| {
                log::error!("Update error: {:?}", e);
                actix_web::error::ErrorInternalServerError("Update error")
            })?;

        if update_result.matched_count > 0 {
            let response_data = ApiResponse::new(
                "File metadata updated",
                Some(file_id.to_hex()),
                Some(code_url),
            );
            Ok(HttpResponse::Ok().json(response_data))
        } else {
            Ok(HttpResponse::BadRequest().json(json!({"message": "Failed to update metadata"})))
        }
    } else {
        let metadata = doc! {
            "_id": file_id,
            "owner_id": owner_id,
            "filename": filename,
            "code_url": &code_url,
            "content_type": content_type,
            "file_size": file_size,
            "upload_time": bson_upload_time,
            "update_time": bson_upload_time,
            "file_path": file_path,
            "file_hash": "example_hash", // TODO: get an algorithm to calculate the file hash (MD5, SHA256, etc.)
        };

        let insert_result = collection.insert_one(metadata, None).await.map_err(|e| {
            log::error!("Insert error: {:?}", e);
            actix_web::error::ErrorInternalServerError("Insert error")
        })?;

        if insert_result.inserted_id.as_object_id() == Some(file_id) {
            let response_data = ApiResponse::new(
                "File uploaded and metadata saved",
                Some(file_id.to_hex()),
                Some(code_url),
            );
            Ok(HttpResponse::Created().json(response_data))
        } else {
            Ok(HttpResponse::BadRequest().json(json!({"message": "Failed to save metadata"})))
        }
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

async fn parse_owner_id(mut field: actix_multipart::Field) -> Result<i32, Error> {
    let mut data = Vec::new();
    while let Some(chunk) = field.next().await {
        data.extend_from_slice(&chunk?);
    }
    let owner_id_str = String::from_utf8(data)
        .map_err(|_| UploadError::BadRequest("Invalid UTF-8 sequence in owner_id".into()))?;
    owner_id_str
        .parse::<i32>()
        .map_err(|_| UploadError::BadRequest("Invalid owner_id format".into()).into())
}
