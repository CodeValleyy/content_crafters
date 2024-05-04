use std::env;

use anyhow::{Error, Result};
use mongodb::{bson::Document, options::SelectionCriteria};

use super::{db::Db, mock_db::MockDb};

pub trait DatabaseInterface {
    fn init() -> impl std::future::Future<Output = Result<Self, Error>> + Send
    where
        Self: Sized;
    fn run_command(
        &self,
        cmd: Document,
        selection_criteria: Option<SelectionCriteria>,
    ) -> impl std::future::Future<Output = Result<Document, Error>> + Send;
}

#[derive(Clone)]
pub enum DatabaseConnection {
    Real(Db),
    Mock(MockDb),
}

impl DatabaseInterface for DatabaseConnection {
    async fn init() -> Result<Self> {
        if env::var("USE_MOCK_DB").unwrap_or_else(|_| "0".to_string()) == "1" {
            Ok(DatabaseConnection::Mock(MockDb::init().await?))
        } else {
            Ok(DatabaseConnection::Real(Db::init().await?))
        }
    }

    async fn run_command(
        &self,
        cmd: Document,
        selection_criteria: Option<SelectionCriteria>,
    ) -> Result<Document, Error> {
        match self {
            DatabaseConnection::Real(db) => db.run_command(cmd, selection_criteria).await,
            DatabaseConnection::Mock(mock) => mock.run_command(cmd, selection_criteria).await,
        }
    }
}
