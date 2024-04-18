use actix_web::{middleware::Logger, App, HttpServer};
use std::net::Ipv4Addr;
use utoipa::OpenApi;

use crate::endpoints::hello;
use hello::HelloWithName;

use utoipa_swagger_ui::SwaggerUi;

pub async fn run_server() -> std::io::Result<()> {
    env_logger::init();

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
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .service(hello::hello_my_g)
            .service(hello::hello_with_name)
    })
    .bind((Ipv4Addr::UNSPECIFIED, 8080))?
    .run()
    .await
}
