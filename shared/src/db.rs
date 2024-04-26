use anyhow::{Result, Error};
use bson::doc;
use log::info;
use std::env;
use mongodb::{Client, options::ClientOptions, Database};

#[derive(Clone)]
pub struct Db {
    pub client: Database,
}

impl Db {
    pub async fn init() -> Result<Self, Error> {
        let db_uri: String = env::var("MONGO_URI").expect("MONGO_URI must be set");
        let db_name = env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set");
        info!("Connecting to MongoDB: {}", db_uri);

        let client_options = ClientOptions::parse(&db_uri).await?;
        let client = Client::with_options(client_options)?;
        let database = client.database(&db_name);

        database.run_command(doc! {"ping": 1}, None).await?;
        Ok(Db { client: database })
    }
}