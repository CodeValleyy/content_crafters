use std::time::SystemTime;

use crate::endpoints::content::models::ContentDetails;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Responder};
use chrono::{DateTime, Utc};
use futures::StreamExt;
use log::info;
use mongodb::{
    bson::{doc, DateTime as BsonDateTime},
    Database,
};

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
    while let Some(item) = payload.next().await {
        let field = item?;
        info!("Field: {:?}", field);

        if field.name() == "file" {
            let content_disposition = field.content_disposition();
            let filename = content_disposition.get_filename().unwrap_or_default();
            let file_size = 0; // TODO: get an algorithm to calculate the file size

            let upload_time: DateTime<Utc> = Utc::now();
            let system_time: SystemTime = SystemTime::from(upload_time);
            let bson_upload_time = BsonDateTime::from(system_time);

            info!("Uploading file: {}", filename);

            let content_type = field.content_type().expect("dripping in dior").to_string();
            let metadata = doc! {
                "owner_id": "123456789", // TODO: get the owner id from the request
                "filename": filename,
                "code_url": "https://codevalley.com/filename?owner_id=123456789", // TODO: get the code url
                "content_type": content_type,
                "file_size": file_size,
                "input_type": "image/png",
                "output_type": "image/png",
                "upload_time": bson_upload_time,
                "update_time": bson_upload_time,
                "file_path": format!("/path/to/save/{}", filename), // TODO: get the path to save the file (local storage, GCP, etc.)
                "file_hash": "example_hash", // TODO: get an algorithm to calculate the file hash (MD5, SHA256, etc.)
            };
            info!("Metadata: {:?}", metadata);

            /* TODO: File Saving example (commented out for now)
            let mut file_bytes = web::BytesMut::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_bytes.extend_from_slice(&data);


            let mut file = tokio::fs::File::create(&_filepath).await?;
            file.write_all(&file_bytes).await?;
            info!("File saved to: {}", _filepath);
            */

            // TODO: Implement upload_to_gcp_storage
            //upload_to_gcp_storage(file_bytes.freeze()).await?;

            // TODO rn use the mongodb (run command) to store the metadata of the file
            let collection = db.collection("programs");
            match collection.insert_one(metadata, None).await {
                Ok(result) => result,
                Err(e) => {
                    return Ok(
                        HttpResponse::BadRequest().body(format!("Error inserting document: {}", e))
                    );
                }
            };

            info!("File uploaded and metadata saved: {}", filename);
        }
    }

    Ok(HttpResponse::Created().body("Upload successful"))
}

#[utoipa::path(
    get,
    path = "/content/{id}",
    tag = "content",
    params(("id"=String, Path, description = "Content database id")),

    responses(
        (status = 200, description = "Content details", body = ContentDetails),
        (status = 404, description = "Content not found"),
    )
)]
pub(super) async fn get_details(name: web::Path<String>) -> impl Responder {
    let result = ContentDetails {
        id: name.into_inner(),
        title: "Sample title".to_string(),
        description: "Sample description".to_string(),
        output_type: "text/html".to_string(),
        input_type: "text/plain".to_string(),
        author: "John Doe".to_string(),
        tags: vec![
            "utility".to_string(),
            "transformation".to_string(),
            "text processing".to_string(),
        ],
        version: "v1.2.0".to_string(),
    };

    HttpResponse::Ok().json(result)
}

pub(super) async fn update_metadata() -> impl Responder {
    HttpResponse::Ok().body("update_metadata")
}
pub(super) async fn delete() -> impl Responder {
    HttpResponse::Ok().body("delete")
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
