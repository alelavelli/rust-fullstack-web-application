use crate::{DatabaseResult, service::database::transaction::DatabaseTransaction};

mod document;
mod service;
mod smart_document;
mod transaction;

/// Trait to define the database service behavior
pub trait DatabaseServiceTrait: Send + Sync {
    async fn connect(&mut self) -> DatabaseResult<()>;

    async fn shutdown(&mut self) -> DatabaseResult<()>;

    fn get_db_name(&self) -> &str;

    fn get_collection<T: Send + Sync>(&self, name: &str) -> DatabaseResult<mongodb::Collection<T>>;

    async fn new_transaction(&self) -> DatabaseResult<DatabaseTransaction>;
}
