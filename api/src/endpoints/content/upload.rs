use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use bson::oid::ObjectId;
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

    while let Some(item) = payload.next().await {
        let field = item?;
        if field.name() == "file" {
            return handle_file_upload(field, &firebase_bucket, db, &client).await;
        }
    }

    Ok(HttpResponse::Ok().json(json!({"message": "No files were uploaded."})))
}

async fn handle_file_upload(
    mut field: actix_multipart::Field,
    firebase_bucket: &str,
    db: web::Data<Database>,
    client: &Client,
) -> Result<HttpResponse, Error> {
    let content_disposition = field.content_disposition().clone();
    let filename = match content_disposition
        .get_filename()
        .map(|name| name.to_string())
    {
        Some(name) if !name.is_empty() => name,
        _ => {
            return Ok(HttpResponse::BadRequest().json(json!({"message": "No filename provided"})));
        }
    };

    let file_size = 0; // TODO: Implement file size calculation
    let upload_time: DateTime<Utc> = Utc::now();
    let bson_upload_time = BsonDateTime::from(SystemTime::from(upload_time));

    info!("Uploading file: {:?}", filename);

    let content_type = field
        .content_type()
        .expect("Content type is missing")
        .to_string();

    let mut file_bytes = web::BytesMut::new();
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        file_bytes.extend_from_slice(&data);
    }

    let file_path = format!("content%2F{}", filename);
    let upload_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o?name={}",
        firebase_bucket, file_path
    );

    let response = client
        .post(&upload_url)
        .header("Content-Type", &content_type)
        .body(file_bytes.to_vec())
        .send()
        .await;

    match response {
        Ok(res) if res.status().is_success() => {
            save_metadata_to_db(
                db,
                filename,
                firebase_bucket,
                file_path,
                content_type,
                file_size,
                bson_upload_time,
            )
            .await
        }
        Ok(res) => {
            let error_message = res
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let error_response = ApiResponse::new(
                format!("Error uploading to Firebase: {}", error_message),
                None,
                None,
            );
            Ok(HttpResponse::BadRequest().json(error_response))
        }
        Err(e) => {
            let error_response =
                ApiResponse::new(format!("Error uploading to Firebase: {}", e), None, None);
            Ok(HttpResponse::BadRequest().json(error_response))
        }
    }
}

async fn save_metadata_to_db(
    db: web::Data<Database>,
    filename: String,
    firebase_bucket: &str,
    file_path: String,
    content_type: String,
    file_size: i64,
    bson_upload_time: BsonDateTime,
) -> Result<HttpResponse, Error> {
    let code_url = format!(
        "https://firebasestorage.googleapis.com/v0/b/{}/o/{}?alt=media",
        firebase_bucket, file_path
    );

    let metadata = doc! {
        "owner_id": ObjectId::new(), // TODO: get the owner id
        "filename": filename,
        "code_url": &code_url,
        "content_type": content_type,
        "file_size": file_size,
        "upload_time": bson_upload_time,
        "update_time": bson_upload_time,
        "file_path": file_path,
        "file_hash": "example_hash", // TODO: get an algorithm to calculate the file hash (MD5, SHA256, etc.)
    };

    let collection = db.collection("programs");
    let insert_result = collection.insert_one(metadata, None).await;

    match insert_result {
        Ok(insert_response) => {
            let response_data = ApiResponse::new(
                "File uploaded and metadata saved",
                insert_response
                    .inserted_id
                    .as_object_id()
                    .map(|oid| oid.to_hex()),
                Some(code_url),
            );
            Ok(HttpResponse::Created().json(response_data))
        }
        Err(e) => {
            let error_response =
                ApiResponse::new(format!("Error inserting document: {}", e), None, None);
            Ok(HttpResponse::BadRequest().json(error_response))
        }
    }
}
