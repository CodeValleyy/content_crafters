use actix_web::{get, post, web::Json, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[utoipa::path(
    responses(
        (status = 201, description = "Created"),
    )
)]
#[get("/hello")]
pub(super) async fn hello_my_g() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub struct HelloWithName {
    #[schema(example = "Mario")]
    name: String,
}

#[utoipa::path(
    responses(
        (status = 201, description = "Created"),
    )
)]
#[post("/hello")]
pub(super) async fn hello_with_name(name: Json<HelloWithName>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name.name))
}
