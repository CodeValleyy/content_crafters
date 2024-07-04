use actix_web::{web, Error, HttpResponse};
use bson::oid::ObjectId;
use futures::StreamExt;
use log::{debug, info, warn};
use mongodb::{bson::doc, Collection, Database};
use shared::models::{
    pipeline::{CreatePipeline, Pipeline, UpdatePipeline},
    program::Program,
};
#[utoipa::path(
    get,
    path = "/pipeline/{id}",
    tag = "pipeline",
    params(("id"=String, Path, description = "Get Pipeline by id")),
    responses(
        (status = 200, description = "Pipeline details", body = Pipeline),
        (status = 404, description = "Pipeline not found"),
    )
)]
pub async fn get_pipeline(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Pipeline> = db.collection("pipelines");

    let id_str = id.as_ref().trim();
    info!("Raw ID string: {}", id_str);

    let object_id = match ObjectId::parse_str(id_str) {
        Ok(oid) => oid,
        Err(e) => {
            warn!("Invalid ID format: {}", e);
            return Err(actix_web::error::ErrorBadRequest("Invalid ID format"));
        }
    };

    info!("Parsed ObjectId: {}", object_id);

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
    path = "/pipeline/owner/{id}",
    tag = "pipeline",
    params(("id"=i32, Path, description = "Get Pipelines by owner id")),
    responses(
        (status = 200, description = "Pipeline details", body = Vec<Pipeline>),
        (status = 404, description = "Pipeline not found"),
    )
)]
pub async fn get_pipelines_by_owner(
    db: web::Data<Database>,
    owner_id: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Pipeline> = db.collection("pipelines");

    let owner_id_value = owner_id.into_inner();
    log::info!("Searching for pipelines with owner_id: {}", owner_id_value);

    let filter = doc! {"owner_id": owner_id_value};
    let result = collection.find(filter, None).await;

    match result {
        Ok(cursor) => {
            let programs: Vec<Pipeline> = cursor
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
                log::warn!("No pipelines found for owner_id: {}", owner_id_value);
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
    get,
    path = "/pipeline/list",
    tag = "pipeline",
    responses(
        (status = 200, description = "List of pipelines", body = Vec<Pipeline>),
        (status = 404, description = "No pipelines found"),
    )
)]
pub async fn list_pipelines(db: web::Data<Database>) -> Result<HttpResponse, Error> {
    let collection: Collection<Pipeline> = db.collection("pipelines");

    let result = collection.find(None, None).await;
    match result {
        Ok(cursor) => {
            let pipelines: Vec<Pipeline> = cursor
                .filter_map(|item| async move {
                    match item {
                        Ok(pipeline) => {
                            log::info!("Found pipeline: {:?}", pipeline);
                            Some(pipeline)
                        }
                        Err(e) => {
                            log::error!("Error reading pipeline: {}", e);
                            None
                        }
                    }
                })
                .collect()
                .await;

            if pipelines.is_empty() {
                log::warn!("No pipelines found");
            }
            Ok(HttpResponse::Ok().json(pipelines))
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
    post,
    path = "/pipeline/create",
    tag = "pipeline",
    responses(
        (status = 201, description = "Pipeline created successfully", body = Pipeline),
        (status = 400, description = "Bad Request"),
    ),
    request_body(
        content_type = "application/json",
        content = CreatePipeline
    ),
)]
pub async fn create_pipeline(
    db: web::Data<Database>,
    pipeline: web::Json<CreatePipeline>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Pipeline> = db.collection("pipelines");

    let create_pipeline = pipeline.into_inner();

    check_programs_exist(&db, &create_pipeline.steps).await?;

    let pipeline: Pipeline = create_pipeline.into();
    let result = collection.insert_one(&pipeline, None).await;

    match result {
        Ok(inserted) => {
            let pipeline_id = inserted.inserted_id.as_object_id().unwrap();
            let pipeline = collection.find_one(doc! {"_id": pipeline_id}, None).await;

            match pipeline {
                Ok(Some(pipeline)) => Ok(HttpResponse::Created().json(pipeline)),
                Ok(None) => Ok(HttpResponse::NotFound().finish()),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
                    "Database query failed: {}",
                    e
                ))),
            }
        }
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Database query failed: {}",
            e
        ))),
    }
}

#[utoipa::path(
    delete,
    path = "/pipeline/{id}",
    tag = "pipeline",
    params(("id"=String, Path, description = "Delete Pipeline by id")),
    responses(
        (status = 204, description = "Pipeline deleted successfully"),
        (status = 404, description = "Pipeline not found"),
    )
)]
pub async fn delete_pipeline(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Pipeline> = db.collection("pipelines");

    let id_str = id.as_ref().trim();
    debug!("Raw ID string: {}", id_str);

    let object_id = match ObjectId::parse_str(id_str) {
        Ok(oid) => oid,
        Err(e) => {
            warn!("Invalid ID format: {}", e);
            return Err(actix_web::error::ErrorBadRequest("Invalid ID format"));
        }
    };

    debug!("Parsed ObjectId: {}", object_id);

    let result = collection.delete_one(doc! {"_id": object_id}, None).await;
    match result {
        Ok(deleted) if deleted.deleted_count == 1 => Ok(HttpResponse::NoContent().finish()),
        Ok(_) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Database query failed: {}",
            e
        ))),
    }
}

#[utoipa::path(
    put,
    path = "/pipeline/{id}",
    tag = "pipeline",
    params(("id"=String, Path, description = "Update Pipeline by id")),
    responses(
        (status = 200, description = "Pipeline updated successfully", body = Pipeline),
        (status = 400, description = "Bad Request"),
        (status = 404, description = "Pipeline not found"),
    ),
    request_body(
        content_type = "application/json",
        content = UpdatePipeline
    ),
)]
pub async fn update_pipeline(
    db: web::Data<Database>,
    id: web::Path<String>,
    update_pipeline: web::Json<UpdatePipeline>,
) -> Result<HttpResponse, Error> {
    let collection: Collection<Pipeline> = db.collection("pipelines");

    let id_str = id.as_ref().trim();
    debug!("Raw ID string: {}", id_str);

    let object_id = match ObjectId::parse_str(id_str) {
        Ok(oid) => oid,
        Err(e) => {
            warn!("Invalid ID format: {}", e);
            return Err(actix_web::error::ErrorBadRequest("Invalid ID format"));
        }
    };

    debug!("Parsed ObjectId: {}", object_id);

    let update_pipeline = update_pipeline.into_inner();
    check_programs_exist(&db, &update_pipeline.steps).await?;

    let update_doc = update_pipeline.build_update_document();
    let update_command = doc! {
        "$set": update_doc,
        "$currentDate": {"update_time": true}
    };

    let result = collection
        .update_one(doc! {"_id": object_id}, update_command, None)
        .await;

    match result {
        Ok(update) if update.matched_count == 1 => {
            let pipeline = collection.find_one(doc! {"_id": object_id}, None).await;

            match pipeline {
                Ok(Some(pipeline)) => Ok(HttpResponse::Ok().json(pipeline)),
                Ok(None) => Ok(HttpResponse::NotFound().finish()),
                Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
                    "Database query failed: {}",
                    e
                ))),
            }
        }
        Ok(_) => Ok(HttpResponse::NotFound().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Database query failed: {}",
            e
        ))),
    }
}

/* Private helper functions */
async fn check_programs_exist(db: &Database, steps: &Vec<String>) -> Result<(), Error> {
    let programs_collection: Collection<Program> = db.collection("programs");
    for step in steps {
        let program_id = match ObjectId::parse_str(step) {
            Ok(id) => id,
            Err(e) => {
                return Err(actix_web::error::ErrorInternalServerError(format!(
                    "Failed to parse program ID: {}",
                    e
                )));
            }
        };

        let program: Result<Option<Program>, mongodb::error::Error> = programs_collection
            .find_one(doc! {"_id": program_id}, None)
            .await;

        match program {
            Ok(Some(_)) => {}
            Ok(None) => {
                return Err(actix_web::error::ErrorBadRequest(format!(
                    "Program not found: {}",
                    program_id
                )));
            }
            Err(e) => {
                return Err(actix_web::error::ErrorInternalServerError(format!(
                    "Database query failed: {}",
                    e
                )));
            }
        }
    }
    Ok(())
}
