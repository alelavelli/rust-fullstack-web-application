use async_trait::async_trait;
use mongodb::{Client, Database, options::ClientOptions};

use crate::{
    DatabaseResult,
    error::DatabaseError,
    service::database::{DatabaseServiceTrait, transaction::DatabaseTransaction},
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

#[async_trait]
impl DatabaseServiceTrait for DatabaseService {
    async fn connect(&mut self) -> DatabaseResult<()> {
        if self.client.is_none() {
            let client_options = ClientOptions::parse(self.connection_string.clone()).await?;
            self.client = Some(Client::with_options(client_options)?);
        }
        Ok(())
    }

    async fn shutdown(&mut self) -> DatabaseResult<()> {
        if let Some(client) = self.client.take() {
            Client::shutdown(client).await;
        }
        Ok(())
    }

    fn get_db_name(&self) -> &str {
        &self.database_name
    }

    fn get_database(&self) -> DatabaseResult<Database> {
        if let Some(client) = &self.client {
            Ok(client.database(&self.database_name))
        } else {
            Err(DatabaseError::ClientNotConnected)
        }
    }

    async fn new_transaction(&self) -> DatabaseResult<DatabaseTransaction> {
        if let Some(client) = &self.client {
            DatabaseTransaction::new(client.start_session().await?, &self.database_name).await
        } else {
            Err(DatabaseError::ClientNotConnected)
        }
    }
}
