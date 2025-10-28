use crate::{
    DatabaseResult,
    service::database::{document::DatabaseDocumentTrait, transaction::DatabaseTransaction},
};

mod document;
mod service;
mod smart_document;
mod transaction;

use async_trait::async_trait;
use bson::Document;
use mongodb::Database;
use serde::Serialize;
pub use service::DatabaseService;

/// Trait to define the database service behavior
#[async_trait]
pub trait DatabaseServiceTrait: Send + Sync + Default {
    async fn connect(&mut self) -> DatabaseResult<()>;

    async fn shutdown(&mut self) -> DatabaseResult<()>;

    fn get_db_name(&self) -> &str;

    fn get_database(&self) -> DatabaseResult<Database>;

    async fn new_transaction(&self) -> DatabaseResult<DatabaseTransaction>;

    async fn save_document<T>(&self, document: Document) -> DatabaseResult<T>
    where
        T: DatabaseDocumentTrait + Send + Sync + Serialize,
    {
        todo!()
    }
}
