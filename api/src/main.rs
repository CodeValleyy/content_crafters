use api::web::run_server;
use cli::parser::{CliApiArgs, Parser};
use log::info;
use logger::init_logger;
use anyhow::Result;
use dotenv::dotenv;
use shared::db::Db;

#[tokio::main]
async fn main() -> Result<()> {
    let cli_args: CliApiArgs = CliApiArgs::parse();
    dotenv().ok();
    init_logger(cli_args.verbose, cli_args.debug, cli_args.trace)?;
    let db = Db::init().await?;
    info!("Database connection established.");
    run_server(db).await?;

    Ok(())
}
