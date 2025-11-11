use std::collections::HashMap;
use std::sync::Arc;

use bson::oid::ObjectId;
use bson::{Bson, Document, from_document};
use tokio::sync::RwLock;

use crate::error::DatabaseError;
use crate::service::database::DatabaseServiceTrait;

use crate::service::database::transaction::MemoryDatabaseTransaction;
use crate::{DatabaseResult, service::database::document::DecoratedDatabaseDocumentTrait};

/// Memory database service stores data in memory in Vec objects
///
/// It is used only for testing purposes without interacting with
/// an actual database
#[derive(Default)]
pub struct MemoryDatabaseService {
    collections: RwLock<HashMap<String, Vec<Document>>>,
}

impl MemoryDatabaseService {
    /// Utility function that matches a query with a document
    ///
    /// For each item in the query, we check if it is present
    /// in our document and if the values match as requested
    fn match_document(document: &Document, query: &Document) -> bool {
        // return true only if all the query constraints are validated to true
        let mut match_result = false;
        for (key, value) in query.iter() {
            // value can be a "scalar" or another document
            // if it is another document then we have an operator
            // like "$in"
            match value {
                Bson::Document(operator_document) => {
                    // we expect `operator_document` to be like { field_name: {"$in": [value1, value2, value3] }}
                    if let Some(Bson::Array(query_array)) = operator_document.get("$in") {
                        // if the key is not present then we return false
                        // otherwise we check for the value according to the operator
                        // in this case, given the array of values we check if the document
                        // element is contained
                        if let Some(field_content) = document.get(key) {
                            match_result = !query_array
                                .iter()
                                .any(|candidate| candidate == field_content);
                        } else {
                            match_result = false;
                        }
                    } else {
                        // add implementation for other operators
                        todo!()
                    }
                }
                query_content => {
                    if let Some(field_document) = document.get(key) {
                        match_result = field_document == query_content;
                    } else {
                        match_result = false;
                    }
                }
            }
            if !match_result {
                // if we get a false then we break the loop
                break;
            }
        }
        match_result
    }

    fn apply_projection(document: &Document, projection: &Document) -> Document {
        if projection.is_empty() {
            document.clone()
        } else {
            let mut projected_document = Document::new();
            for (key, value) in projection.iter() {
                if matches!(value, Bson::Int32(1) | Bson::Int64(1) | Bson::Boolean(true))
                    && let Some(value_to_project) = document.get(key)
                {
                    projected_document.insert(key.clone(), value_to_project.clone());
                }
            }
            projected_document
        }
    }
}

impl DatabaseServiceTrait for MemoryDatabaseService {
    type Transaction = MemoryDatabaseTransaction;

    async fn connect(&mut self) -> DatabaseResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self) -> DatabaseResult<()> {
        Ok(())
    }

    fn get_db_name(&self) -> &str {
        "database"
    }

    async fn new_transaction(&self) -> DatabaseResult<Self::Transaction> {
        Ok(MemoryDatabaseTransaction::new())
    }

    async fn insert_one<T>(
        &self,
        document: bson::Document,
        _transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> DatabaseResult<ObjectId>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let object_id = ObjectId::new();
        let mut document = document.clone();
        document.insert("_id", object_id);

        let collection = T::collection_name();

        self.collections
            .try_write()
            .map_err(|err| DatabaseError::TransactionError(err.to_string()))?
            .entry(collection.into())
            .or_default()
            .push(document);
        Ok(object_id)
    }

    async fn insert_many<T>(
        &self,
        documents: Vec<bson::Document>,
        _transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> DatabaseResult<Vec<ObjectId>>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let mut inserted_ids = vec![];

        for document in documents {
            inserted_ids.push(self.insert_one::<T>(document, None).await?);
        }
        Ok(inserted_ids)
    }

    async fn find_one<T>(&self, query: bson::Document) -> DatabaseResult<Option<T>>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let collection = T::collection_name();
        if let Some(documents) = self.collections.read().await.get(collection) {
            for document in documents.iter() {
                if Self::match_document(document, &query) {
                    return from_document(document.clone())
                        .map(Some)
                        .map_err(|e| DatabaseError::DocumentNotValid(e.to_string()));
                }
            }
            Ok(None)
        } else {
            Ok(None)
        }
    }

    async fn find_many<T>(&self, query: bson::Document) -> DatabaseResult<Vec<T>>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let mut documents_to_return = vec![];
        let collection = T::collection_name();
        if let Some(documents) = self.collections.read().await.get(collection) {
            for document in documents.iter() {
                if Self::match_document(document, &query) {
                    documents_to_return.push(
                        from_document(document.clone())
                            .map_err(|e| DatabaseError::DocumentNotValid(e.to_string()))?,
                    );
                }
            }
        }
        Ok(documents_to_return)
    }

    async fn find_one_projection<T, P>(
        &self,
        query: bson::Document,
        projection: bson::Document,
    ) -> DatabaseResult<Option<P>>
    where
        T: DecoratedDatabaseDocumentTrait,
        P: Send + Sync + serde::Serialize + serde::de::DeserializeOwned,
    {
        let collection = T::collection_name();
        if let Some(documents) = self.collections.read().await.get(collection) {
            for document in documents.iter() {
                if Self::match_document(document, &query) {
                    let projected = Self::apply_projection(document, &projection);
                    return from_document(projected)
                        .map(Some)
                        .map_err(|e| DatabaseError::DocumentNotValid(e.to_string()));
                }
            }
        }
        Ok(None)
    }

    async fn find_many_projection<T, P>(
        &self,
        query: bson::Document,
        projection: bson::Document,
    ) -> DatabaseResult<Vec<P>>
    where
        T: DecoratedDatabaseDocumentTrait,
        P: Send + Sync + serde::Serialize + serde::de::DeserializeOwned,
    {
        let mut documents_to_return = vec![];
        let collection = T::collection_name();
        if let Some(documents) = self.collections.read().await.get(collection) {
            for document in documents.iter() {
                if Self::match_document(document, &query) {
                    documents_to_return.push(
                        from_document(Self::apply_projection(document, &projection))
                            .map_err(|e| DatabaseError::DocumentNotValid(e.to_string()))?,
                    );
                }
            }
        }
        Ok(documents_to_return)
    }

    async fn count_documents<T>(&self, query: bson::Document) -> DatabaseResult<u64>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let mut count = 0;
        let collection = T::collection_name();
        if let Some(documents) = self.collections.read().await.get(collection) {
            for document in documents.iter() {
                if Self::match_document(document, &query) {
                    count += 1;
                }
            }
        }
        Ok(count)
    }

    async fn update_one<T>(
        &self,
        query: bson::Document,
        update: bson::Document,
        _transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> DatabaseResult<()>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let collection = T::collection_name();

        let mut guard = self
            .collections
            .try_write()
            .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;

        if let Some(documents) = guard.get_mut(collection) {
            for document in documents.iter_mut() {
                if Self::match_document(document, &query) {
                    if let Some(Bson::Document(set_document)) = update.get("$set") {
                        for (key, value) in set_document.iter() {
                            document.insert(key.clone(), value.clone());
                        }
                    }
                    break;
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    async fn update_many<T>(
        &self,
        query: bson::Document,
        update: bson::Document,
        _transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> DatabaseResult<()>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let collection = T::collection_name();

        let mut guard = self
            .collections
            .try_write()
            .map_err(|err| DatabaseError::TransactionError(err.to_string()))?;

        if let Some(documents) = guard.get_mut(collection) {
            for document in documents.iter_mut() {
                if Self::match_document(document, &query)
                    && let Some(Bson::Document(set_document)) = update.get("$set")
                {
                    for (key, value) in set_document.iter() {
                        document.insert(key.clone(), value.clone());
                    }
                }
            }
            Ok(())
        } else {
            Ok(())
        }
    }

    async fn delete_one<T>(
        &self,
        query: bson::Document,
        _transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> DatabaseResult<()>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let collection = T::collection_name();
        if let Some(documents) = self.collections.write().await.get_mut(collection)
            && let Some(document_position) = documents
                .iter()
                .position(|doc| Self::match_document(doc, &query))
        {
            documents.remove(document_position);
        }
        Ok(())
    }

    async fn delete_many<T>(
        &self,
        query: bson::Document,
        _transaction: Option<Arc<RwLock<Self::Transaction>>>,
    ) -> DatabaseResult<()>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        let collection = T::collection_name();
        if let Some(documents) = self.collections.write().await.get_mut(collection) {
            documents.retain(|doc| !Self::match_document(doc, &query));
        }
        Ok(())
    }

    async fn aggreagte<T>(
        &self,
        _pipeline: Vec<bson::Document>,
    ) -> DatabaseResult<Vec<bson::Document>>
    where
        T: DecoratedDatabaseDocumentTrait,
    {
        todo!()
    }
}
