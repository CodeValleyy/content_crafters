use std::time::SystemTime;

use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Responder};
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use futures::StreamExt;
use log::info;
use mongodb::{
    bson::{doc, DateTime as BsonDateTime}, Collection, Database
};
use actix_web::error;
use shared::models::program::Program;
use crate::endpoints::content::update_program_dto::UpdateProgramDto;

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
        Err(e) => Err(error::ErrorInternalServerError(format!("Database query failed: {}", e))),
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
    update_dto: web::Json<UpdateProgramDto>
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

    let update_result = collection.update_one(
        doc! {"_id": object_id},
        update_command,
        None
    ).await;

    match update_result {
        Ok(update) if update.matched_count == 1 => Ok(HttpResponse::Ok().body("Content metadata updated")),
        Ok(_) => Ok(HttpResponse::NotFound().body("Content not found")),
        Err(e) => Err(error::ErrorInternalServerError(format!("Database operation failed: {}", e))),
    }
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
