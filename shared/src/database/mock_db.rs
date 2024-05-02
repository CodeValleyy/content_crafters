use bson::{doc, Document};
use log::{debug, info};
use mongodb::options::SelectionCriteria;

use super::db_interface::DatabaseInterface;
use anyhow::Error;

#[derive(Clone)]
pub struct MockDb {}

impl DatabaseInterface for MockDb {
    async fn run_command(
        &self,
        cmd: Document,
        _selection_criteria: Option<SelectionCriteria>,
    ) -> Result<Document, Error> {
        if cmd.get("ping").is_some() {
            debug!("ping reply: {:?}", doc! {"ok": 1});
            Ok(doc! {"ok": 1})
        } else {
            Err(Error::msg("Invalid command"))
        }
    }

    async fn init() -> anyhow::Result<Self, anyhow::Error>
    where
        Self: Sized,
    {
        let db_instance = MockDb {};
        db_instance.run_command(doc! {"ping": 1}, None).await?;
        info!("Database mock connection established.");
        Ok(db_instance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bson::doc;
    use tokio::runtime::Runtime;

    #[test]
    fn test_run_command_with_valid_ping() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let db = MockDb {};
            let command = doc! {"ping": "1"};
            let result = db.run_command(command, None).await;
            assert_eq!(result.unwrap(), doc! {"ok": 1});
        });
    }

    #[test]
    fn test_run_command_with_invalid_command() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let db = MockDb {};
            let command = doc! {"select * from users": "0"};
            let result = db.run_command(command, None).await;
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_init_method_success() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let result = MockDb::init().await;
            assert!(result.is_ok());
        });
    }
}
