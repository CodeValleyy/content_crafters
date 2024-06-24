use crate::endpoints::content::update_program_dto::UpdateProgramDto;
use actix_web::{web, Error, HttpResponse};
use bson::oid::ObjectId;
use futures::StreamExt;
use mongodb::{bson::doc, Collection, Database};
use shared::models::program::Program;

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
        Err(_) => return Err(actix_web::error::ErrorBadRequest("Invalid ID format")),
    };

    let result = collection.find_one(doc! {"_id": object_id}, None).await;
    match result {
        Ok(Some(program)) => Ok(HttpResponse::Ok().json(program)),
        Ok(None) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Database query failed: {}",
            e
        ))),
    }
}

#[utoipa::path(
    get,
    path = "/content/owner/{id}",
    tag = "content",
    params(("id"=i32, Path, description = "Get Contents by owner id")),
    responses(
        (status = 200, description = "Content details", body = Vec<Program>),
        (status = 404, description = "Content not found"),
    )
)]
pub async fn get_contents_by_owner(
    db: web::Data<Database>,
    owner_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Program> = db.collection("programs");

    let owner_id_value = owner_id.into_inner();
    log::info!("Searching for programs with owner_id: {}", owner_id_value);

    let filter = doc! {"owner_id": owner_id_value};
    let result = collection.find(filter, None).await;

    match result {
        Ok(cursor) => {
            let programs: Vec<Program> = cursor
                .filter_map(|item| async move {
                    match item {
                        Ok(program) => {
                            log::info!("Found program: {:?}", program);
                            Some(program)
                        }
                        Err(e) => {
                            log::error!("Error reading program: {}", e);
                            None
                        }
                    }
                })
                .collect()
                .await;

            if programs.is_empty() {
                log::warn!("No programs found for owner_id: {}", owner_id_value);
            }
            Ok(HttpResponse::Ok().json(programs))
        }
        Err(e) => {
            log::error!("Database query failed: {}", e);
            Err(actix_web::error::ErrorInternalServerError(format!(
                "Database query failed: {}",
                e
            )))
        }
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
        Err(_) => return Err(actix_web::error::ErrorBadRequest("Invalid ID format")),
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
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
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
        Err(_) => return Err(actix_web::error::ErrorBadRequest("Invalid ID format")),
    };

    let delete_result = collection.delete_one(doc! {"_id": object_id}, None).await;
    match delete_result {
        Ok(delete) if delete.deleted_count == 1 => Ok(HttpResponse::Ok().body("Content deleted")),
        Ok(_) => Ok(HttpResponse::NotFound().body("Content not found")),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Database operation failed: {}",
            e
        ))),
    }
}
