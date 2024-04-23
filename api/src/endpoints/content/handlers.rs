use crate::endpoints::content::models::ContentDetails;
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse, Responder};
use futures::StreamExt;
use log::info;
use tokio::io::AsyncWriteExt;
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
pub async fn upload(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;
        info!("Field: {:?}", field);

        if field.name() == "file" {
            let content_disposition = field.content_disposition();
            let filename = content_disposition.get_filename().unwrap();
            info!("Uploading file: {}", filename);
            let _filepath = format!("{}", sanitize_filename::sanitize(filename));

            let mut file_bytes = web::BytesMut::new();
            while let Some(chunk) = field.next().await {
                let data = chunk?;
                file_bytes.extend_from_slice(&data);
            }
            /* TODO: File Saving example
            let mut file = tokio::fs::File::create(&_filepath).await?;
            file.write_all(&file_bytes).await?;
            info!("File saved to: {}", _filepath);
            */
            // TODO: Implement upload_to_gcp_storage
            //upload_to_gcp_storage(file_bytes.freeze()).await?;
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
        comments: vec!["comment1".to_string(), "comment2".to_string()],
        likes: 42,
        editable: true,
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
