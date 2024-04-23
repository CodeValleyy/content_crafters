use api::web::run_server;
use cli::parser::{CliApiArgs, Parser};
use dotenv::dotenv;
use logger::init_logger;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_args: CliApiArgs = CliApiArgs::parse();
    dotenv().ok();
    init_logger(cli_args.verbose, cli_args.debug, cli_args.trace)?;
    run_server().await?;
    Ok(())
}
