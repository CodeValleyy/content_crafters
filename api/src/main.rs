use anyhow::Result;
use api::web::run_server;
use cli::parser::{CliApiArgs, Parser};
use dotenv::dotenv;
use log::info;
use logger::init_logger;
use shared::database::db_interface::{DatabaseConnection, DatabaseInterface};

#[tokio::main]
async fn main() -> Result<()> {
    let cli_args: CliApiArgs = CliApiArgs::parse();
    dotenv().ok();
    init_logger(cli_args.verbose, cli_args.debug, cli_args.trace)?;
    let db = DatabaseConnection::init().await?;

    info!("Database connection established.");
    run_server(db).await?;

    Ok(())
}
