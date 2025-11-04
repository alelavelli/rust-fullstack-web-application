use bson::{Document, oid::ObjectId};
use futures::TryStreamExt;
use mongodb::{
    Client, Database,
    options::{ClientOptions, FindOneOptions, FindOptions},
};
use serde::Serialize;
use tokio::sync::RwLock;

use crate::{
    DatabaseResult,
    error::DatabaseError,
    service::database::{
        DatabaseServiceTrait, document::DecoratedDatabaseDocumentTrait,
        transaction::MongoDBDatabaseTransaction,
    },
};

/// Database service struct using mongdb crate
///
/// It connects to the database, creates session objects to perform transactions
/// and provides collection objects.
///
/// According to MongoDB documentation https://www.mongodb.com/docs/drivers/rust/current/fundamentals/performance/
/// the client instance must be shared among threads, therefore, the database service
/// will be part of the app state.
#[derive(Debug, Clone)]
pub struct MongoDBDatabaseService {
    client: Option<Client>,
    database_name: String,
    connection_string: String,
}

impl MongoDBDatabaseService {
    pub fn new(database_name: String, connection_string: String) -> MongoDBDatabaseService {
        Self {
            client: None,
            database_name,
            connection_string,
        }
    }

    fn get_database(&self) -> DatabaseResult<Database> {
        if let Some(client) = &self.client {
            Ok(client.database(&self.database_name))
        } else {
            Err(DatabaseError::ClientNotConnected)
        }
    }
}

impl Default for MongoDBDatabaseService {
    fn default() -> Self {
        /*
        The default implementation works on localhost and takes
        the current thread id to create unique database name.

        This is particularly usefule when testing the database interactions
        with multiple threads avoiding contamination between tests.
        */
        let id = format!("{:?}", std::thread::current().id());
        let mut database_name = String::from("app-test-db-");
        database_name.push_str(&id);
        let connection_string = format!(
            "mongodb://localhost:27117/{}?replicaSet=rs0&directConnection=true",
            database_name
        );
        Self {
            client: None,
            database_name,
            connection_string,
        }
    }
}

impl DatabaseServiceTrait for MongoDBDatabaseService {
    type Transaction = MongoDBDatabaseTransaction;

    fn connect(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>> {
        async {
            if self.client.is_none() {
                let client_options = ClientOptions::parse(self.connection_string.clone()).await?;
                self.client = Some(Client::with_options(client_options)?);
            }
            Ok(())
        }
    }

    fn shutdown(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>> {
        async {
            if let Some(client) = self.client.take() {
                Client::shutdown(client).await;
            }
            Ok(())
        }
    }

    fn get_db_name(&self) -> &str {
        &self.database_name
    }

    fn new_transaction(
        &self,
    ) -> impl std::future::Future<Output = DatabaseResult<MongoDBDatabaseTransaction>> + Send {
        async {
            if let Some(client) = &self.client {
                MongoDBDatabaseTransaction::new(client.start_session().await?).await
            } else {
                Err(DatabaseError::ClientNotConnected)
            }
        }
    }

    fn insert_one<T>(
        &self,
        document: Document,
        transaction: Option<RwLock<Self::Transaction>>,
    ) -> impl std::future::Future<Output = DatabaseResult<ObjectId>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection(T::collection_name());
            let operation = collection.insert_one(document);
            let query_result = if let Some(transaction) = transaction {
                let mut transaction_guard = transaction
                    .try_write()
                    .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;
                let session = transaction_guard.get_mut_session();
                operation.session(session).await
            } else {
                operation.await
            };
            query_result.map(|inner| {
                inner
                    .inserted_id
                    .as_object_id()
                    .ok_or(DatabaseError::InvalidObjectId)
            })?
        }
    }

    fn insert_many<T>(
        &self,
        documents: Vec<Document>,
        transaction: Option<RwLock<Self::Transaction>>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<ObjectId>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection(T::collection_name());
            let operation = collection.insert_many(documents);
            let query_result = if let Some(transaction) = transaction {
                let mut transaction_guard = transaction
                    .try_write()
                    .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;
                let session = transaction_guard.get_mut_session();
                operation.session(session).await
            } else {
                operation.await
            };
            query_result.map(|inner| {
                inner
                    .inserted_ids
                    .values()
                    .map(|elem| elem.as_object_id().ok_or(DatabaseError::InvalidObjectId))
                    .collect::<DatabaseResult<Vec<ObjectId>>>()
            })?
        }
    }

    fn find_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<T>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            Ok(collection.find_one(query).await?)
        }
    }

    fn find_many<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<T>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            Ok(collection.find(query).await?.try_collect().await?)
        }
    }

    fn find_one_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<P>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
        P: Send + Sync + Serialize + serde::de::DeserializeOwned,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let query_options = FindOneOptions::builder().projection(projection).build();
            let result: Option<P> = collection
                .clone_with_type::<P>()
                .find_one(query)
                .with_options(query_options)
                .await?;
            Ok(result)
        }
    }

    fn find_many_projection<T, P>(
        &self,
        query: Document,
        projection: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<P>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
        P: Send + Sync + Serialize + serde::de::DeserializeOwned,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let query_options = FindOptions::builder().projection(projection).build();
            let result: Vec<P> = collection
                .clone_with_type::<P>()
                .find(query)
                .with_options(query_options)
                .await?
                .try_collect()
                .await?;
            Ok(result)
        }
    }

    fn count_documents<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<u64>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let result: u64 = collection.count_documents(query).await?;
            Ok(result)
        }
    }

    fn update_one<T>(
        &self,
        query: Document,
        update: Document,
        transaction: Option<RwLock<Self::Transaction>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let operation = collection.update_one(query, update);
            if let Some(transaction) = transaction {
                let mut transaction_guard = transaction
                    .try_write()
                    .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;
                let session = transaction_guard.get_mut_session();
                operation.session(session).await?
            } else {
                operation.await?
            };
            Ok(())
        }
    }

    fn update_many<T>(
        &self,
        query: Document,
        update: Document,
        transaction: Option<RwLock<Self::Transaction>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let operation = collection.update_many(query, update);
            if let Some(transaction) = transaction {
                let mut transaction_guard = transaction
                    .try_write()
                    .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;
                let session = transaction_guard.get_mut_session();
                operation.session(session).await?
            } else {
                operation.await?
            };
            Ok(())
        }
    }

    fn delete_one<T>(
        &self,
        query: Document,
        transaction: Option<RwLock<Self::Transaction>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let operation = collection.delete_one(query);
            if let Some(transaction) = transaction {
                let mut transaction_guard = transaction
                    .try_write()
                    .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;
                let session = transaction_guard.get_mut_session();
                operation.session(session).await?
            } else {
                operation.await?
            };
            Ok(())
        }
    }

    fn delete_many<T>(
        &self,
        query: Document,
        transaction: Option<RwLock<Self::Transaction>>,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let operation = collection.delete_many(query);
            if let Some(transaction) = transaction {
                let mut transaction_guard = transaction
                    .try_write()
                    .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;
                let session = transaction_guard.get_mut_session();
                operation.session(session).await?
            } else {
                operation.await?
            };
            Ok(())
        }
    }

    fn aggreagte<T>(
        &self,
        pipeline: Vec<Document>,
    ) -> impl std::future::Future<Output = DatabaseResult<Vec<Document>>> + Send
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        async {
            let collection = self.get_database()?.collection::<T>(T::collection_name());
            let result = collection.aggregate(pipeline).await?.try_collect().await?;
            Ok(result)
        }
    }
}
