use actix_web::web;

use super::handlers::{
    add_comment, add_like, create_version, delete, delete_comment, get_details, get_version,
    list_versions, remove_like, update_comment, update_metadata, upload,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/content")
            .route("/upload", web::post().to(upload))
            .route("/{id}", web::get().to(get_details))
            .route("/{id}", web::put().to(update_metadata))
            .route("/{id}", web::delete().to(delete))
            .service(
                web::scope("/{id}/comment")
                    .route("", web::post().to(add_comment))
                    .route("/{comment_id}", web::put().to(update_comment))
                    .route("/{comment_id}", web::delete().to(delete_comment)),
            )
            .route("/{id}/like", web::post().to(add_like))
            .route("/{id}/like", web::delete().to(remove_like))
            .service(
                web::scope("/{id}/versions")
                    .route("", web::get().to(list_versions))
                    .route("", web::post().to(create_version))
                    .route("/{version}", web::get().to(get_version)),
            ),
    );
}
