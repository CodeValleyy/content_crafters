use actix_web::web;

use super::{metadata, upload, version};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/content")
            .route("/upload", web::post().to(upload::upload))
            .route(
                "/owner/{id}",
                web::get().to(metadata::get_contents_by_owner),
            )
            .route("/{id}", web::delete().to(metadata::delete))
            .route("/{id}", web::get().to(metadata::get_details))
            .route("/{id}", web::put().to(metadata::update_metadata))
            .route("/{id}", web::delete().to(metadata::delete))
            .route("/versions", web::get().to(version::list_versions))
            .route("/versions", web::post().to(version::create_version))
            .route("/versions/{id}", web::get().to(version::get_version)),
    );
}
