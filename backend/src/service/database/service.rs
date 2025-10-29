use async_trait::async_trait;
use bson::{Document, oid::ObjectId};
use mongodb::{Client, Collection, Database, options::ClientOptions};
use serde::Serialize;

use crate::{
    DatabaseResult,
    error::DatabaseError,
    service::database::{
        DatabaseServiceTrait, document::DatabaseDocumentTrait, transaction::DatabaseTransaction,
    },
};

/// Database service struct
///
/// It connects to the database, creates session objects to perform transactions
/// and provides collection objects.
///
/// According to MongoDB documentation https://www.mongodb.com/docs/drivers/rust/current/fundamentals/performance/
/// the client instance must be shared among threads, therefore, the database service
/// will be part of the app state.
#[derive(Debug, Clone)]
pub struct DatabaseService {
    client: Option<Client>,
    database_name: String,
    connection_string: String,
}

impl DatabaseService {
    pub fn new(database_name: String, connection_string: String) -> DatabaseService {
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

    fn get_collection<T>(&self) -> DatabaseResult<Collection<T>>
    where
        T: DatabaseDocumentTrait + Send + Sync + Serialize,
    {
        Ok(self.get_database()?.collection::<T>(T::collection_name()))
    }
}

impl Default for DatabaseService {
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

impl DatabaseServiceTrait for DatabaseService {
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
    ) -> impl std::future::Future<Output = DatabaseResult<DatabaseTransaction>> {
        async {
            if let Some(client) = &self.client {
                DatabaseTransaction::new(client.start_session().await?, &self.database_name).await
            } else {
                Err(DatabaseError::ClientNotConnected)
            }
        }
    }

    fn insert_one<T>(
        &self,
        document: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<ObjectId>>
    where
        T: DatabaseDocumentTrait + Send + Sync + Serialize,
    {
        async {
            let collection = self.get_database()?.collection(T::collection_name());
            let query_result = collection.insert_one(document).await;
            query_result.map(|inner| {
                inner
                    .inserted_id
                    .as_object_id()
                    .ok_or(DatabaseError::InvalidObjectId)
            })?
        }
    }

    fn find_one<T>(
        &self,
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<T>>>
    where
        T: DatabaseDocumentTrait + Send + Sync,
    {
        async { todo!() }
    }
}
