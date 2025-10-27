use bson::{Document, oid::ObjectId};
use mongodb::{ClientSession, Collection, Database};
use serde::Serialize;

use crate::{DatabaseResult, error::DatabaseError, service::database::document::DatabaseDocument};

/// Wraps database operations inside the transaction allowing to commit or abort everything
pub struct DatabaseTransaction {
    session: ClientSession,
    database: Database,
}

impl DatabaseTransaction {
    /// Creates a new instance of database transaction and starts it
    pub async fn new(
        mut session: ClientSession,
        database_name: &str,
    ) -> DatabaseResult<DatabaseTransaction> {
        session.start_transaction().await?;
        let database = session.client().database(database_name);
        Ok(DatabaseTransaction { session, database })
    }

    /// Consumes the object and abort the transaction
    pub async fn abort_transaction(mut self) -> DatabaseResult<()> {
        self.session.abort_transaction().await?;
        Ok(())
    }

    /// Consumes the object and commit the transaction
    pub async fn commit_transaction(mut self) -> DatabaseResult<()> {
        self.session.commit_transaction().await?;
        Ok(())
    }

    /// Utility method to retrieve the Collection object from
    /// database object
    fn get_collection<T>(&self) -> Collection<T>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        self.database.collection::<T>(T::collection_name())
    }

    pub async fn insert_one<T>(&mut self, document: &T) -> DatabaseResult<ObjectId>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        let query_result = collection
            .insert_one(document)
            .session(&mut self.session)
            .await;

        if let Ok(outcome) = query_result {
            outcome
                .inserted_id
                .as_object_id()
                .ok_or(DatabaseError::TransactionError(
                    "Outcome of insert_one does not have an id".into(),
                ))
        } else {
            // TODO: Here we can abort the transaction but we would need to take ownership
            // of DatabaseTransaction. For the moment, we give this responsibility to the
            // database transaction middleware that when an error is returned then the transaction
            // is aborted.
            Err(DatabaseError::TransactionError(
                query_result
                    .err()
                    .ok_or(DatabaseError::TransactionError(
                        "outcome of insert_one is None but it does not contain Error".into(),
                    ))?
                    .to_string(),
            ))
        }
    }

    pub async fn insert_many<T>(&mut self, documents: Vec<&T>) -> DatabaseResult<Vec<ObjectId>>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        let query_result = collection
            .insert_many(documents)
            .session(&mut self.session)
            .await;

        if let Ok(outcome) = query_result {
            outcome
                .inserted_ids
                .values()
                .map(|elem| {
                    elem.as_object_id().ok_or(DatabaseError::TransactionError(
                        "Outcome of insert_many does not have an id".into(),
                    ))
                })
                .collect::<DatabaseResult<Vec<ObjectId>>>()
        } else {
            Err(DatabaseError::TransactionError(
                query_result
                    .err()
                    .ok_or(DatabaseError::TransactionError(
                        "outcome of insert_many is None but it does not contain error".into(),
                    ))?
                    .to_string(),
            ))
        }
    }

    pub async fn update_one<T>(&mut self, query: Document, update: Document) -> DatabaseResult<()>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        collection
            .update_one(query, update)
            .session(&mut self.session)
            .await?;
        Ok(())
    }

    pub async fn update_many<T>(&mut self, query: Document, update: Document) -> DatabaseResult<()>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        collection
            .update_many(query, update)
            .session(&mut self.session)
            .await?;
        Ok(())
    }

    pub async fn replace_one<T>(&mut self, query: Document, replacement: &T) -> DatabaseResult<()>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        collection
            .replace_one(query, replacement)
            .session(&mut self.session)
            .await?;
        Ok(())
    }

    pub async fn delete_one<T>(&mut self, filter: Document) -> DatabaseResult<()>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        collection
            .delete_one(filter)
            .session(&mut self.session)
            .await?;
        Ok(())
    }

    pub async fn delete_many<T>(&mut self, filter: Document) -> DatabaseResult<()>
    where
        T: DatabaseDocument + Send + Sync + Serialize,
    {
        let collection = self.get_collection::<T>();
        collection
            .delete_many(filter)
            .session(&mut self.session)
            .await?;
        Ok(())
    }
}
