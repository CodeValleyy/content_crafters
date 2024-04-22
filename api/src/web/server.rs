use actix_web::{middleware::Logger, web, App, HttpServer};
use std::env;
use std::net::Ipv4Addr;
use utoipa::OpenApi;

use crate::endpoints::content;
use crate::endpoints::content::models::ContentDetails;
use crate::endpoints::content::routes::config as content_config;

use log::info;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run_server() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            content::handlers::upload,
            content::handlers::get_details,
        ),
        components(
            schemas(ContentDetails/*, TODO: Add other schemas here */)
        ),

        tags(
                (name = "content", description = "Content related operations")
        ),
        servers(
            (url = "/v1", description = "Base URL for all API endpoints")
        )
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();
    let port = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    info!("Starting server on port {}", port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(web::scope("/v1").configure(content_config))
            .configure(content_config)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}
