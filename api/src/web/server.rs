use actix_web::{middleware::Logger, App, HttpServer};
use std::net::Ipv4Addr;
use utoipa::OpenApi;
use std::env;

use crate::endpoints::hello;
use hello::HelloWithName;

use utoipa_swagger_ui::SwaggerUi;
use log::info;

pub async fn run_server() -> std::io::Result<()> {
    #[derive(OpenApi)]
    #[openapi(
        paths(hello::hello_my_g, hello::hello_with_name
        ),
        components(
            schemas(HelloWithName)
        ),

        tags(
                (name = "hello", description = "Hello world!")
    )
    )]
    struct ApiDoc;
    let openapi = ApiDoc::openapi();
    let port = env::var("APP_PORT").unwrap_or_else(|_| "8080".to_string()).parse::<u16>().unwrap_or(8080);
    info!("Starting server on port {}", port);
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello::hello_my_g)
            .service(hello::hello_with_name)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}
