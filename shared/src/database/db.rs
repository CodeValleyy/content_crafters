use anyhow::{Error, Result};
use bson::doc;
use log::info;
use mongodb::bson::Document;
use mongodb::options::SelectionCriteria;
use mongodb::{options::ClientOptions, Client, Database};
use std::env;

use super::db_interface::DatabaseInterface;

#[derive(Clone)]
pub struct Db {
    pub client: Database,
}

impl DatabaseInterface for Db {
    async fn init() -> Result<Self, Error> {
        let db_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
        let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set");
        info!("Connecting to MongoDB: {}", db_uri);

        let client_options = ClientOptions::parse(&db_uri).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&db_name);
        let db_instance = Db { client: database };

        db_instance.run_command(doc! {"ping": 0}, None).await?;

        Ok(db_instance)
    }

    async fn run_command(
        &self,
        cmd: Document,
        selection_criteria: Option<SelectionCriteria>,
    ) -> Result<Document, Error> {
        let result = self.client.run_command(cmd, selection_criteria).await?;
        Ok(result)
    }
}
