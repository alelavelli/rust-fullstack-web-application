use mongodb::ClientSession;

use crate::DatabaseResult;

/// Wraps database operations inside the transaction allowing to commit or abort everything
pub struct DatabaseTransaction {
    session: ClientSession,
}

impl DatabaseTransaction {
    /// Creates a new instance of database transaction and starts it
    pub async fn new(mut session: ClientSession) -> DatabaseResult<DatabaseTransaction> {
        session.start_transaction().await?;
        Ok(DatabaseTransaction { session })
    }

    /// Abort the transaction
    pub async fn abort_transaction(&mut self) -> DatabaseResult<()> {
        self.session.abort_transaction().await?;
        Ok(())
    }

    /// Commit the transaction
    pub async fn commit_transaction(&mut self) -> DatabaseResult<()> {
        self.session.commit_transaction().await?;
        Ok(())
    }

    /// Return a mutable reference of the session
    pub fn get_mut_session(&mut self) -> &mut ClientSession {
        &mut self.session
    }
}
