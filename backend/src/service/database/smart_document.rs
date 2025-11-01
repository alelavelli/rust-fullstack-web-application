use std::sync::Arc;

use bson::oid::ObjectId;

use crate::{
    DatabaseResult,
    error::DatabaseError,
    service::database::{DatabaseServiceTrait, document::DatabaseDocumentTrait},
};
use mongodb::bson::doc;

/// Enum that caches a mongodb document so that it can be
/// accessed reading it repeatedly without doing an additional query
/// to the database
#[derive(Clone)]
pub enum SmartDocumentReference<T: DatabaseDocumentTrait> {
    Id(ObjectId),
    Document(T),
}

impl<T: DatabaseDocumentTrait> SmartDocumentReference<T> {
    /// Returns the document id without query the database
    pub fn as_ref_id(&self) -> &ObjectId {
        match self {
            SmartDocumentReference::Id(document_id) => document_id,
            SmartDocumentReference::Document(document) => document.get_id(),
        }
    }

    /// Returns a reference of the document
    ///
    /// if it is Document(T) then its reference is returned,
    /// otherwise, a query on the database is made
    pub async fn as_document_ref(
        &mut self,
        database_service: Arc<impl DatabaseServiceTrait>,
    ) -> DatabaseResult<&T> {
        match self {
            SmartDocumentReference::Id(document_id) => {
                let document = database_service
                    .find_one::<T>(doc! {"_id": *document_id})
                    .await
                    .and_then(|op| op.ok_or(DatabaseError::DocumentDoesNotExist(*document_id)))?;
                *self = SmartDocumentReference::Document(document);
                match self {
                    SmartDocumentReference::Id(_) => unreachable!(),
                    SmartDocumentReference::Document(document) => Ok(document),
                }
            }
            SmartDocumentReference::Document(document) => Ok(document),
        }
    }

    /// Returns a mutable reference of the document
    ///
    /// if it is Document(T) then its reference is returned,
    /// otherwise, a query on the database is made
    pub async fn as_document_ref_mut(
        &mut self,
        database_service: Arc<impl DatabaseServiceTrait>,
    ) -> DatabaseResult<&mut T> {
        match self {
            SmartDocumentReference::Id(document_id) => {
                let document = database_service
                    .find_one::<T>(doc! {"_id": *document_id})
                    .await
                    .and_then(|op| op.ok_or(DatabaseError::DocumentDoesNotExist(*document_id)))?;
                *self = SmartDocumentReference::Document(document);
                match self {
                    SmartDocumentReference::Id(_) => unreachable!(),
                    SmartDocumentReference::Document(document) => Ok(document),
                }
            }
            SmartDocumentReference::Document(document) => Ok(document),
        }
    }

    /// Consumes the object and returns the document
    pub async fn to_document(
        self,
        database_service: Arc<impl DatabaseServiceTrait>,
    ) -> DatabaseResult<T> {
        match self {
            SmartDocumentReference::Id(document_id) => {
                let document = database_service
                    .find_one::<T>(doc! {"_id": document_id})
                    .await
                    .and_then(|op| op.ok_or(DatabaseError::DocumentDoesNotExist(document_id)))?;
                Ok(document)
            }
            SmartDocumentReference::Document(document) => Ok(document),
        }
    }
}

impl<T: DatabaseDocumentTrait> From<ObjectId> for SmartDocumentReference<T> {
    fn from(value: ObjectId) -> Self {
        Self::Id(value)
    }
}

impl<T: DatabaseDocumentTrait> From<&ObjectId> for SmartDocumentReference<T> {
    fn from(value: &ObjectId) -> Self {
        Self::Id(*value)
    }
}

impl<T: DatabaseDocumentTrait> From<T> for SmartDocumentReference<T> {
    fn from(value: T) -> Self {
        Self::Document(value)
    }
}
