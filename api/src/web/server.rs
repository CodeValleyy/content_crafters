use actix_cors::Cors;
use actix_web::http;
use actix_web::web::{Data, JsonConfig};
use actix_web::{middleware::Logger, web, App, HttpServer};
use log::info;
use shared::models::pipeline::{CreatePipeline, Pipeline, UpdatePipeline};
use shared::{
    database::db_interface::DatabaseConnection,
    models::{program::Program, upload_file::UploadFile},
};
use std::env;
use std::net::Ipv4Addr;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::endpoints::content::{
    routes::config as content_config, update_program_dto::UpdateProgramDto,
};

use crate::endpoints::pipeline::routes::config as pipeline_config;

const DEFAULT_PORT: u16 = 8080;

/// This is an asynchronous function `run_server` that starts a server and binds it to a specified port.
/// It accepts a generic parameter `T` that implements the `DatabaseInterface` trait.
/// The function takes a `db` parameter of type `T` which represents the database interface.
/// The function returns a `std::io::Result<()>` indicating whether the server started successfully or encountered an error.
///
/// # Arguments
///
/// * `db` - A generic parameter `T` that implements the `DatabaseInterface` trait. It represents the database interface.
///
/// # Panics
///
/// The function may panic if there is an error binding the server to the specified port.
///
/// # Errors
///
/// The function returns a `std::io::Result<()>` indicating whether the server started successfully or encountered an error.
///
/// # Safety
///
/// The function is safe to call as long as the provided `db` parameter implements the `DatabaseInterface` trait correctly.
///
/// # Notes
///
/// - The function uses the `get_server_port` function to determine the port to bind the server to.
/// - It creates a server address tuple `(Ipv4Addr, u16)` with the unspecified IP address and the determined port.
/// - The function generates a Swagger UI URL based on the server address.
/// - It creates an `HttpServer` instance and configures it with the provided `db` parameter, logger middleware, Swagger UI, and content routes.
/// - Finally, it binds the server to the server address and runs it asynchronously.
///
/// # Returns
///
/// The function returns a `std::io::Result<()>` indicating whether the server started successfully or encountered an error.
///
pub async fn run_server(db: DatabaseConnection) -> std::io::Result<()> {
    let web_db = match db {
        DatabaseConnection::Real(real_db) => real_db.client,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unsupported database connection",
            ))
        }
    };

    let port = get_server_port();
    let server_address: (Ipv4Addr, u16) = (Ipv4Addr::UNSPECIFIED, port);
    let swagger_url = format!(
        "http://{}:{}/swagger-ui/",
        server_address.0, server_address.1
    );

    info!("Starting server on port {}", port);
    info!("Swagger UI available at {}", swagger_url);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                if let Some(origin_str) = origin.to_str().ok() {
                    origin_str.ends_with(":3000") || origin_str.contains("code-valley.xyz")
                } else {
                    false
                }
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .app_data(Data::new(web_db.clone()))
            .app_data(JsonConfig::default())
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", generate_openapi()),
            )
            .service(web::resource("/health").to(|| async { "OK" }))
            .service(
                web::scope("/v1")
                    .configure(content_config)
                    .configure(pipeline_config),
            )
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
        crate::endpoints::content::upload::upload,
        crate::endpoints::content::metadata::get_contents_by_owner,
        crate::endpoints::content::metadata::get_details,
        crate::endpoints::content::metadata::update_metadata,
        crate::endpoints::content::metadata::delete,
        crate::endpoints::pipeline::metadata::get_pipelines_by_owner,
        crate::endpoints::pipeline::metadata::get_pipeline,
        crate::endpoints::pipeline::metadata::list_pipelines,
        crate::endpoints::pipeline::metadata::create_pipeline,
        crate::endpoints::pipeline::metadata::delete_pipeline,
        crate::endpoints::pipeline::metadata::update_pipeline,
    ),
    components(
        schemas(
            UpdateProgramDto,
            UploadFile,
            Program,
            Pipeline,
            CreatePipeline,
            UpdatePipeline
        ),
    ),

    tags(
            (name = "content", description = "Content related operations"),
            (name = "pipeline", description = "Pipeline related operations")
    ),
    servers(
        (url = "/v1", description = "Base URL for all API endpoints")
    )
)]
struct ApiDoc;

fn generate_openapi() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}
