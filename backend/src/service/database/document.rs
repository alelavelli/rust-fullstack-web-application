use bson::{Document, oid::ObjectId};
use serde::{Serialize, de::DeserializeOwned};

use crate::DatabaseResult;

/// Trait that defines the behavior for each collection in database.
///
/// Operations divide in methods and functions.
/// Methods are save and delete and refers only to the current instance.
///
/// Functions are general and operate outside the instance
pub trait DatabaseDocument: Sized + Send + Sync + Serialize + DeserializeOwned {
    fn get_id(&self) -> &ObjectId;
    fn collection_name() -> &'static str;

    fn find_one(
        query: Document,
    ) -> impl std::future::Future<Output = DatabaseResult<Option<Self>>> + Send;
}
