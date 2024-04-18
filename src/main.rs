use content_crafters::web::run_server;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    run_server().await
}
