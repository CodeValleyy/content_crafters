use actix_web::{middleware::Logger, web, App, HttpServer};
use shared::db::Db;
use std::env;
use std::net::Ipv4Addr;
use utoipa::OpenApi;

use crate::endpoints::content;
use crate::endpoints::content::models::{ContentDetails, UploadFile};
use crate::endpoints::content::routes::config as content_config;

use log::info;
use utoipa_swagger_ui::SwaggerUi;

const DEFAULT_PORT: u16 = 8080;

pub async fn run_server(db: Db) -> std::io::Result<()> {
    let port = get_server_port();
    let server_address = (Ipv4Addr::UNSPECIFIED, port);
    let swagger_url = format!(
        "http://{}:{}/swagger-ui/",
        server_address.0, server_address.1
    );

    info!("Starting server on port {}", port);
    info!("Swagger UI available at {}", swagger_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", generate_openapi()),
            )
            .service(web::scope("/v1").configure(content_config))
    })
    .bind(server_address)?
    .run()
    .await
}

fn get_server_port() -> u16 {
    env::var("APP_PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse()
        .unwrap_or(DEFAULT_PORT)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        content::handlers::upload,
        content::handlers::get_details,
    ),
    components(
        schemas(ContentDetails, UploadFile/*, TODO: Add other schemas here */)
    ),

    tags(
            (name = "content", description = "Content related operations")
    ),
    servers(
        (url = "/v1", description = "Base URL for all API endpoints")
    )
)]
struct ApiDoc;

fn generate_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
