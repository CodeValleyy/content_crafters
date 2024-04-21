use api::web::run_server;
use cli::parser::{CliApiArgs, Parser};
use std::error::Error;
use logger::init_logger;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_args: CliApiArgs = CliApiArgs::parse();
    init_logger(cli_args.verbose, cli_args.debug, cli_args.trace)?;
    run_server().await?;
    Ok(())
}