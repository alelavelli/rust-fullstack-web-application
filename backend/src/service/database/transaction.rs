use mongodb::ClientSession;

use crate::DatabaseResult;

pub trait DatabaseTransactionTrait {
    fn abort_transaction(&mut self)
    -> impl std::future::Future<Output = DatabaseResult<()>> + Send;
    fn commit_transaction(
        &mut self,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send;
}

/// Wraps database operations inside the transaction allowing to commit or abort everything
pub struct MongoDBDatabaseTransaction {
    session: ClientSession,
}

impl MongoDBDatabaseTransaction {
    /// Creates a new instance of database transaction and starts it
    pub async fn new(mut session: ClientSession) -> DatabaseResult<MongoDBDatabaseTransaction> {
        session.start_transaction().await?;
        Ok(MongoDBDatabaseTransaction { session })
    }

    /// Return a mutable reference of the session
    pub fn get_mut_session(&mut self) -> &mut ClientSession {
        &mut self.session
    }
}

impl DatabaseTransactionTrait for MongoDBDatabaseTransaction {
    /// Abort the transaction
    async fn abort_transaction(&mut self) -> DatabaseResult<()> {
        self.session.abort_transaction().await?;
        Ok(())
    }

    /// Commit the transaction
    fn commit_transaction(
        &mut self,
    ) -> impl std::future::Future<Output = DatabaseResult<()>> + Send {
        async {
            self.session.commit_transaction().await?;
            Ok(())
        }
    }
}

pub struct MemoryDatabaseTransaction {}

impl MemoryDatabaseTransaction {
    pub fn new() -> Self {
        Self {}
    }
}

impl DatabaseTransactionTrait for MemoryDatabaseTransaction {
    fn abort_transaction(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>> {
        async { Ok(()) }
    }

    fn commit_transaction(&mut self) -> impl std::future::Future<Output = DatabaseResult<()>> {
        async { Ok(()) }
    }
}
