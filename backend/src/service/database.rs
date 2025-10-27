use crate::{
    DatabaseResult,
    service::database::{document::DatabaseDocument, transaction::DatabaseTransaction},
};

mod document;
mod service;
mod smart_document;
mod transaction;

use async_trait::async_trait;
use mongodb::Database;
pub use service::DatabaseService;

/// Trait to define the database service behavior
#[async_trait]
pub trait DatabaseServiceTrait: Send + Sync {
    async fn connect(&mut self) -> DatabaseResult<()>;

    async fn shutdown(&mut self) -> DatabaseResult<()>;

    fn get_db_name(&self) -> &str;

    fn get_database(&self) -> DatabaseResult<Database>;

    async fn new_transaction(&self) -> DatabaseResult<DatabaseTransaction>;
}
