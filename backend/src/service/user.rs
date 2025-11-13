use std::sync::Arc;

use bson::doc;
use tokio::sync::RwLock;

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
    user: Arc<RwLock<SmartDocumentReference<User>>>,
    database_service: Arc<T>,
}

impl<T: DatabaseServiceTrait> UserService<T> {
    pub fn new(user: Arc<RwLock<SmartDocumentReference<User>>>, database_service: Arc<T>) -> Self {
        Self {
            user,
            database_service,
        }
    }

    /// Load the user from the database updating smart refence document
    pub async fn get(&mut self) -> ServiceResult<User> {
        Ok(self
            .user
            .write()
            .await
            .as_document_ref(self.database_service.clone())
            .await?
            .clone())
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use tokio::sync::RwLock;

    use crate::{
        model::{User, UserBuilder},
        service::{
            database::{
                document::DatabaseDocumentTrait, memory_service::MemoryDatabaseService,
                smart_document::SmartDocumentReference,
            },
            user::UserService,
        },
        utils::hash_password,
    };

    async fn create_user(
        database_service: Arc<MemoryDatabaseService>,
        username: &str,
        password: &str,
    ) -> User {
        let password_hash = hash_password(password).unwrap();

        UserBuilder::new(database_service.clone())
            .first_name("Marcel".into())
            .last_name("Proust".into())
            .admin(false)
            .username(username.into())
            .publisher(false)
            .password_hash(password_hash)
            .build(None)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn test_login() {
        let username = "username";
        let password = "password";

        let database_service = Arc::new(MemoryDatabaseService::default());

        create_user(database_service.clone(), "first_user", "first_user").await;
        let user = create_user(database_service.clone(), username, password).await;

        let logged_user = UserService::login(database_service.clone(), username, password)
            .await
            .unwrap();
        assert_eq!(logged_user.username(), &username);
        assert_eq!(logged_user.get_id(), user.get_id());
    }

    #[tokio::test]
    async fn test_get() {
        let username = "username";
        let password = "password";

        let database_service = Arc::new(MemoryDatabaseService::default());

        create_user(database_service.clone(), "first_user", "first_user").await;
        let user = create_user(database_service.clone(), username, password).await;

        let mut user_service = UserService::new(
            Arc::new(RwLock::new(SmartDocumentReference::Document(user.clone()))),
            database_service.clone(),
        );

        let read_user = user_service.get().await.unwrap();

        assert_eq!(read_user.username(), &username);
        assert_eq!(read_user.get_id(), user.get_id());
    }
}
