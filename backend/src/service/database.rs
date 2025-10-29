use crate::{
    DatabaseResult,
    service::database::{document::DatabaseDocumentTrait, transaction::DatabaseTransaction},
};

mod document;
mod service;
mod smart_document;
mod transaction;

use bson::{Document, oid::ObjectId};
use serde::{Deserialize, Serialize};
pub use service::DatabaseService;

/// Trait to define the database service behavior
pub trait DatabaseServiceTrait: Send + Sync + Default {
    fn connect(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>>;

    fn shutdown(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>>;

    fn get_db_name(&self) -> &str;

    fn new_transaction(
        &self,
    ) -> impl std::future::Future<Output = DatabaseResult<DatabaseTransaction>>;

    /// Insert the mongodb document in the collection specified by T
    /// and return the instance of the inserted document
    fn insert_one<T>(
        &self,
        document: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<ObjectId>>
    where
        T: DatabaseDocumentTrait + Send + Sync + Serialize;

    fn find_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<T>>>
    where
        T: DatabaseDocumentTrait + Send + Sync;
}
