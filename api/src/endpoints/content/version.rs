use actix_web::{HttpResponse, Responder};

pub(super) async fn list_versions() -> impl Responder {
    HttpResponse::Ok().body("list_versions")
}

pub(super) async fn create_version() -> impl Responder {
    HttpResponse::Ok().body("create_version")
}

pub(super) async fn get_version() -> impl Responder {
    HttpResponse::Ok().body("get_version")
}
