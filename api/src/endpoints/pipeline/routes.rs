use actix_web::web;

use super::metadata::{
    create_pipeline, delete_pipeline, get_pipeline, get_pipelines_by_owner, list_pipelines,
    update_pipeline,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/pipeline")
            .route("/list", web::get().to(list_pipelines))
            .route("/create", web::post().to(create_pipeline))
            .route("/{id}", web::get().to(get_pipeline))
            .route("/{id}", web::delete().to(delete_pipeline))
            .route("/{id}", web::put().to(update_pipeline))
            .route("/owner/{id}", web::get().to(get_pipelines_by_owner)),
    );
}
