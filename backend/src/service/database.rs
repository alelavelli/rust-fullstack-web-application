//! Database mod defines the DatabaseServiceTrait and other modules required
//! to handle database operations.
//!
//! Modules:
//!
//! - `document`: defines DatabaseDocumentTrait and macros to create documents
//! - `smart_document`: defines the SmartDocumentReference enum used to cache document instance
//! - `transaction`: defines DatabaseTransactionTrait and implementations
//! - `mongodb_service`: implementation of DatabaseServiceTrait that interacts with MongoDB cluster
//! - `memory_service`: implementation of DatabaseServiceTrait for in memory database, used for testing

use std::sync::Arc;

use crate::{
    DatabaseResult,
    service::database::{
        document::DecoratedDatabaseDocumentTrait, transaction::DatabaseTransactionTrait,
    },
};

pub mod document;
pub mod memory_service;
mod mongodb_service;
pub mod smart_document;
pub mod transaction;

use bson::{Document, oid::ObjectId};
pub use mongodb_service::MongoDBDatabaseService;
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::RwLock;

/// Trait to define the database service behavior
///
/// The first two methods allows to open and close the connection with the database,
/// while the other methods allows to start a new transaction and perform any other
/// database operations.
///
/// Any database operation method requires generic T which implements
/// DecoratedDatabaseDocumentTrait that is used to get collection name and return
/// the specific document struct.
///
/// If transaction is provided then the operations will be done inside its context.
/// Transaction is provided as Arc<RwLock<>> so that it can be shared among threads
/// safely.
///
/// The return error type is DatabaseError which contains all the possible
/// error outcomes. It does not use ServiceAppError because it is a second
/// level service that is used by other services.
pub trait DatabaseServiceTrait: Default {
    type Transaction: DatabaseTransactionTrait + Send + Sync + 'static;

    fn connect(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>>;

    fn shutdown(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>>;

    fn get_db_name(&self) -> &str;

    fn new_transaction(
        &self,
    ) -> impl std::future::Future<Output = DatabaseResult<Self::Transaction>> + Send;

    /// Inserts the mongodb document in the collection specified by T
    /// and returns the id of the inserted document.
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn insert_one<T>(
        &self,
        document: Document,
        transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> impl std::future::Future<Output = DatabaseResult<ObjectId>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Inserts multiple mongodb documents in the collection specified by T
    /// and returns the vector of inserted ids.
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn insert_many<T>(
        &self,
        documents: Vec<Document>,
        transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<ObjectId>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Finds the first document that matches the query or None if it does not exist
    fn find_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<T>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Finds the documents that match the query or None if it does not exist
    fn find_many<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<T>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Finds the first document that matches the query or None if it does not exist
    ///
    /// The returned type is P that is a document with a subset of fields of original
    /// collection.
    fn find_one_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<P>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
        P: Send + Sync + Serialize + DeserializeOwned;

    /// Finds the documents that match the query or None if it does not exist
    ///
    /// The returned type is P that is a document with a subset of fields of original
    /// collection.
    fn find_many_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<P>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
        P: Send + Sync + Serialize + DeserializeOwned;

    /// Counts the number of documents in the collection that match the query
    fn count_documents<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<u64>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Updates the first document that matches the query with the given
    /// document containing attributes to set.
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn update_one<T>(
        &self,
        query: Document,
        update: Document,
        transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Updates the documents that matches the query with the given
    /// document containing attributes to set.
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn update_many<T>(
        &self,
        query: Document,
        update: Document,
        transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Delete the first document that matches the query
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn delete_one<T>(
        &self,
        query: Document,
        transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Delete the documents that matches the query
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn delete_many<T>(
        &self,
        query: Document,
        transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;

    /// Apply the operations in the pipeline and returns the specified document.
    ///
    /// If transaction is provided then the operation will be done inside its context
    fn aggreagte<T>(
        &self,
        pipeline: Vec<Document>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<Document>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait;
}
