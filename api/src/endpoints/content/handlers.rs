use crate::endpoints::content::update_program_dto::UpdateProgramDto;
use actix_multipart::Multipart;
use actix_web::error;
use actix_web::{web, Error, HttpResponse, Responder};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use log::info;
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    Collection, Database,
};
use reqwest::Client;
use serde_json::json;
use shared::database::api_response::ApiResponse;
use shared::models::program::Program;
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
        let mut field = item?;
        info!("Field: {:?}", field);

        if field.name() == "file" {
            let content_disposition = field.content_disposition().clone();
            let filename = content_disposition
                .get_filename()
                .map(|name| name.to_string())
                .unwrap_or_default();
            if filename.is_empty() {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "message": "No filename provided"
                })));
            }

            let file_size = 0; // TODO: get an algorithm to calculate the file size

            let upload_time: DateTime<Utc> = Utc::now();
            let system_time: SystemTime = SystemTime::from(upload_time);
            let bson_upload_time = BsonDateTime::from(system_time);

            info!("Uploading file: {:?}", filename);

            let content_type = field.content_type().expect("").to_string();

            let mut file_bytes = web::BytesMut::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_bytes.extend_from_slice(&data);
            }

            let file_path = format!("content/{}", filename);
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
                    let metadata = doc! {
                        "owner_id": ObjectId::new(), // TODO: get the owner id
                        "filename": filename,
                        "code_url": format!("https://firebasestorage.googleapis.com/v0/b/{}/o/{}?alt=media", firebase_bucket, file_path),
                        "content_type": content_type,
                        "file_size": file_size,
                        "upload_time": bson_upload_time,
                        "update_time": bson_upload_time,
                        "file_path": file_path,
                        "file_hash": "example_hash", // TODO: get an algorithm to calculate the file hash (MD5, SHA256, etc.)
                    };

                    let collection = db.collection("programs");
                    let insert_result = collection.insert_one(metadata, None).await;
                    return match insert_result {
                        Ok(insert_response) => {
                            let response_data = ApiResponse::new(
                                "File uploaded and metadata saved",
                                insert_response
                                    .inserted_id
                                    .as_object_id()
                                    .map(|oid| oid.to_hex()),
                            );
                            Ok(HttpResponse::Created().json(response_data))
                        }
                        Err(e) => {
                            let error_response =
                                ApiResponse::new(format!("Error inserting document: {}", e), None);
                            Ok(HttpResponse::BadRequest().json(error_response))
                        }
                    };
                }
                Ok(res) => {
                    let error_message = res
                        .text()
                        .await
                        .unwrap_or_else(|_| "Unknown error".to_string());
                    let error_response = ApiResponse::new(
                        format!("Error uploading to Firebase: {}", error_message),
                        None,
                    );
                    return Ok(HttpResponse::BadRequest().json(error_response));
                }
                Err(e) => {
                    let error_response =
                        ApiResponse::new(format!("Error uploading to Firebase: {}", e), None);
                    return Ok(HttpResponse::BadRequest().json(error_response));
                }
            };
        }
    }

    Ok(HttpResponse::Ok().json(json!({"message": "No files were uploaded."})))
}

#[utoipa::path(
    get,
    path = "/content/{id}",
    tag = "content",
    params(("id"=String, Path, description = "Get Content by id")),

    responses(
        (status = 200, description = "Content details", body = Program),
        (status = 404, description = "Content not found"),
    )
)]
pub async fn get_details(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Program> = db.collection("programs");

    let object_id = match ObjectId::parse_str(&id.as_ref()) {
        Ok(oid) => oid,
        Err(_) => return Err(error::ErrorBadRequest("Invalid ID format")),
    };

    let result = collection.find_one(doc! {"_id": object_id}, None).await;
    match result {
        Ok(Some(program)) => Ok(HttpResponse::Ok().json(program)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(error::ErrorInternalServerError(format!(
            "Database query failed: {}",
            e
        ))),
    }
}

#[utoipa::path(
    put,
    path = "/content/{id}",
    tag = "content",
    params(("id"=String, Path, description = "Edit Content by id")),
    request_body(
        content_type = "application/json",
        content = UpdateProgramDto
    ),

responses(
        (status = 200, description = "Content metadata updated", body = String),
        (status = 404, description = "Content not found"),
    )
)]
pub async fn update_metadata(
    db: web::Data<Database>,
    id: web::Path<String>,
    update_dto: web::Json<UpdateProgramDto>,
) -> Result<HttpResponse, Error> {
    let collection = db.collection::<Program>("programs");
    let object_id = match ObjectId::parse_str(&id.as_ref()) {
        Ok(oid) => oid,
        Err(_) => return Err(error::ErrorBadRequest("Invalid ID format")),
    };
    let update_doc = update_dto.build_update_document();
    let update_command = doc! {
        "$set": update_doc,
        "$currentDate": {"update_time": true}
    };

    let update_result = collection
        .update_one(doc! {"_id": object_id}, update_command, None)
        .await;

    match update_result {
        Ok(update) if update.matched_count == 1 => {
            Ok(HttpResponse::Ok().body("Content metadata updated"))
        }
        Ok(_) => Ok(HttpResponse::NotFound().body("Content not found")),
        Err(e) => Err(error::ErrorInternalServerError(format!(
            "Database operation failed: {}",
            e
        ))),
    }
}

#[utoipa::path(
    delete,
    path = "/content/{id}",
    tag = "content",
    params(("id"=String, Path, description = "Delete Content by id")),
    responses(
        (status = 200, description = "Content deleted", body = String),
        (status = 404, description = "Content not found"),
    )
)]
pub async fn delete(db: web::Data<Database>, id: web::Path<String>) -> Result<HttpResponse, Error> {
    let collection = db.collection::<Program>("programs");
    let object_id = match ObjectId::parse_str(&id.as_ref()) {
        Ok(oid) => oid,
        Err(_) => return Err(error::ErrorBadRequest("Invalid ID format")),
    };

    let delete_result = collection.delete_one(doc! {"_id": object_id}, None).await;
    match delete_result {
        Ok(delete) if delete.deleted_count == 1 => Ok(HttpResponse::Ok().body("Content deleted")),
        Ok(_) => Ok(HttpResponse::NotFound().body("Content not found")),
        Err(e) => Err(error::ErrorInternalServerError(format!(
            "Database operation failed: {}",
            e
        ))),
    }
}

pub(super) async fn add_comment() -> impl Responder {
    HttpResponse::Ok().body("add_comment")
}

pub(super) async fn update_comment() -> impl Responder {
    HttpResponse::Ok().body("update_comment")
}

pub(super) async fn delete_comment() -> impl Responder {
    HttpResponse::Ok().body("delete_comment")
}

pub(super) async fn add_like() -> impl Responder {
    HttpResponse::Ok().body("add_like")
}

pub(super) async fn remove_like() -> impl Responder {
    HttpResponse::Ok().body("remove_like")
}

pub(super) async fn list_versions() -> impl Responder {
    HttpResponse::Ok().body("list_versions")
}

pub(super) async fn create_version() -> impl Responder {
    HttpResponse::Ok().body("create_version")
}
pub(super) async fn get_version() -> impl Responder {
    HttpResponse::Ok().body("get_version")
}
