use actix_web::web;

use super::upload;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/group").route("/upload", web::post().to(upload::upload)));
}
