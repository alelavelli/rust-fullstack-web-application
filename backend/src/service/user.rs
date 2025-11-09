use std::sync::Arc;

use bson::doc;

use crate::{
    ServiceResult,
    error::{AuthError, ServiceAppError},
    model::User,
    service::database::{DatabaseServiceTrait, smart_document::SmartDocumentReference},
};

/// UserService struct allows operations at user level.
///
/// Each instance is created for a specific user and operations
/// are done from its point of view.
///
/// A copy of `DatabaseService` is used to easly operate on database.
pub struct UserService<T>
where
    T: DatabaseServiceTrait,
{
    user: SmartDocumentReference<User>,
    database_service: Arc<T>,
}

impl<T: DatabaseServiceTrait> UserService<T> {
    pub fn new(user: SmartDocumentReference<User>, database_service: Arc<T>) -> Self {
        Self {
            user,
            database_service,
        }
    }

    /// Load the user from the database updating smart refence document
    pub async fn get(&mut self) -> ServiceResult<&User> {
        Ok(self
            .user
            .as_document_ref(self.database_service.clone())
            .await?)
    }

    /// Retrieve from the database the document with the given username and
    /// verify the password hash
    ///
    /// Returned Error
    /// --------------
    ///
    /// InternalServerError: when bcrypt fails
    /// DatabaseError: when an operation over database fails
    /// WrongCredentials: when the username does not exist or password is wrong
    pub async fn login(
        database_service: Arc<T>,
        username: &str,
        password: &str,
    ) -> ServiceResult<User> {
        if let Some(user_document) = database_service
            .find_one::<User>(doc! {"username": username})
            .await?
        {
            if bcrypt::verify(password, user_document.password_hash()).map_err(|e| {
                ServiceAppError::InternalServerError(format!(
                    "Error in password hash verification. Got {e}"
                ))
            })? {
                Ok(user_document)
            } else {
                Err(AuthError::WrongCredentials)?
            }
        } else {
            Err(ServiceAppError::AuthorizationError(
                AuthError::WrongCredentials,
            ))
        }
    }
}
