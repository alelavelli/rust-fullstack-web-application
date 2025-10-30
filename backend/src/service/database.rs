use crate::{DatabaseResult, service::database::document::DatabaseDocumentTrait};

mod document;
mod service;
mod smart_document;
mod transaction;

use bson::{Document, oid::ObjectId};
use serde::{Serialize, de::DeserializeOwned};
pub use service::DatabaseService;
pub use transaction::DatabaseTransaction;

/// Trait to define the database service behavior
pub trait DatabaseServiceTrait: Send + Sync + Default {
    fn connect(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>>;

    fn shutdown(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>>;

    fn get_db_name(&self) -> &str;

    fn new_transaction(
        &self,
    ) -> impl std::future::Future<Output = DatabaseResult<DatabaseTransaction>> + Send;

    /// Insert the mongodb document in the collection specified by T
    /// and return the id of the inserted document
    fn insert_one<T>(
        &self,
        document: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<ObjectId>>
    where
        T: DatabaseDocumentTrait;

    fn insert_many<T>(
        &self,
        documents: Vec<Document>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<ObjectId>>>
    where
        T: DatabaseDocumentTrait;

    fn find_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<T>>>
    where
        T: DatabaseDocumentTrait;

    fn find_many<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<T>>>
    where
        T: DatabaseDocumentTrait;

    fn find_one_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<P>>>
    where
        T: DatabaseDocumentTrait,
        P: Send + Sync + Serialize + DeserializeOwned;

    fn find_many_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<P>>>
    where
        T: DatabaseDocumentTrait,
        P: Send + Sync + Serialize + DeserializeOwned;

    fn count_documents<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<u64>>
    where
        T: DatabaseDocumentTrait;

    fn update_one<T>(
        &self,
        query: Document,
        update: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<()>>
    where
        T: DatabaseDocumentTrait;

    fn update_many<T>(
        &self,
        query: Document,
        update: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<()>>
    where
        T: DatabaseDocumentTrait;

    fn delete_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<()>>
    where
        T: DatabaseDocumentTrait;

    fn delete_many<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<()>>
    where
        T: DatabaseDocumentTrait;

    fn aggreagte<T>(
        &self,

        pipeline: Vec<Document>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<Document>>>
    where
        T: DatabaseDocumentTrait;
}
