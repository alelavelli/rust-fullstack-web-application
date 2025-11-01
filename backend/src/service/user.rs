use bson::doc;

use crate::{
    ServiceResult,
    error::{AuthError, ServiceAppError},
    model::User,
    service::database::DatabaseServiceTrait,
};

pub struct UserService {}

impl UserService {
    /// Retrieve from the database the document with the given username and
    /// verify the password hash
    ///
    /// Returned Error
    /// --------------
    ///
    /// InternalServerError: when bcrypt fails
    /// DatabaseError: when an operation over database fails
    /// WrongCredentials: when the username does not exist or password is wrong
    pub async fn login<T: DatabaseServiceTrait>(
        database_service: &T,
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
