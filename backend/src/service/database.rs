use crate::{DatabaseResult, service::database::document::DatabaseDocumentTrait};

pub mod document;
mod service;
mod smart_document;
mod transaction;

use bson::{Document, oid::ObjectId};
use mongodb::ClientSession;
use serde::{Serialize, de::DeserializeOwned};
pub use service::DatabaseService;
pub use transaction::DatabaseTransaction;

/// Trait to define the database service behavior
///
/// The first two methods allows to open and close the connection with the database,
/// while the other methods allows to start a new transaction and perform any other
/// database operations.
///
/// Any database operation method requires generic T which implements
/// DatabaseDocumentTrait that is used to get collection name and return
/// the specific document struct.
///
/// The return error type is DatabaseError which contains all the possible
/// error outcomes. It does not use ServiceAppError because it is a second
/// level service that is used by other services.
pub trait DatabaseServiceTrait: Send + Sync + Default + Clone {
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
        transaction_session: Option<&mut ClientSession>,
    ) -> impl std::future::Future<Output = DatabaseResult<ObjectId>> + Send
    where
        T: DatabaseDocumentTrait;

    fn insert_many<T>(
        &self,
        documents: Vec<Document>,
        transaction_session: Option<&mut ClientSession>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<ObjectId>>> + Send
    where
        T: DatabaseDocumentTrait;

    fn find_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<T>>> + Send
    where
        T: DatabaseDocumentTrait;

    fn find_many<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<T>>> + Send
    where
        T: DatabaseDocumentTrait;

    fn find_one_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<P>>> + Send
    where
        T: DatabaseDocumentTrait,
        P: Send + Sync + Serialize + DeserializeOwned;

    fn find_many_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<P>>> + Send
    where
        T: DatabaseDocumentTrait,
        P: Send + Sync + Serialize + DeserializeOwned;

    fn count_documents<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<u64>> + Send
    where
        T: DatabaseDocumentTrait;

    fn update_one<T>(
        &self,
        query: Document,
        update: Document,
        transaction_session: Option<&mut ClientSession>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DatabaseDocumentTrait;

    fn update_many<T>(
        &self,
        query: Document,
        update: Document,
        transaction_session: Option<&mut ClientSession>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DatabaseDocumentTrait;

    fn delete_one<T>(
        &self,
        query: Document,
        transaction_session: Option<&mut ClientSession>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DatabaseDocumentTrait;

    fn delete_many<T>(
        &self,
        query: Document,
        transaction_session: Option<&mut ClientSession>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DatabaseDocumentTrait;

    fn aggreagte<T>(
        &self,

        pipeline: Vec<Document>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<Document>>> + Send
    where
        T: DatabaseDocumentTrait;
}
