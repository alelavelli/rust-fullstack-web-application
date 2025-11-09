use std::sync::Arc;

use crate::{
    auth::AuthInfo,
    error::{AppError, FacadeResult, ServiceAppError},
    model::{User, UserBuilder},
    service::{
        access_control::AccessControl,
        database::{DatabaseServiceTrait, document::DatabaseDocumentTrait},
    },
    utils::hash_password,
};
use bson::{doc, oid::ObjectId};
use tokio::sync::RwLock;

pub struct AdminFacade<D>
where
    D: DatabaseServiceTrait,
{
    database_service: Arc<D>,
}

impl<D> AdminFacade<D>
where
    D: DatabaseServiceTrait,
{
    /// Verify that the auth info corresponds to a platform admin
    /// and then returns an instance of the facade
    pub async fn new<T: AuthInfo>(auth_info: T, database_service: Arc<D>) -> FacadeResult<Self> {
        AccessControl::new(auth_info, database_service.clone())
            .await
            .map_err(|err| match err {
                ServiceAppError::AccessControlError(msg) => AppError::AccessControlError(msg),
                other => AppError::InternalServerError {
                    msg: other.to_string(),
                    source_error: other,
                },
            })?
            .is_platform_admin()
            .await
            .map_err(|err| match err {
                ServiceAppError::AccessControlError(msg) => AppError::AccessControlError(msg),
                other => AppError::InternalServerError {
                    msg: other.to_string(),
                    source_error: other,
                },
            })?;
        Ok(Self { database_service })
    }

    /// Return the list of users in the application
    pub async fn get_users(&self) -> FacadeResult<Vec<User>> {
        self.database_service
            .find_many::<User>(doc! {})
            .await
            .map_err(|err| AppError::InternalServerError {
                msg: "Error in retrieving users from database".into(),
                source_error: ServiceAppError::from(err),
            })
    }

    /// Create a new user in the application
    pub async fn create_user(
        &self,
        transaction: Arc<RwLock<D::Transaction>>,
        first_name: String,
        last_name: String,
        username: String,
        password: String,
        admin: bool,
    ) -> FacadeResult<ObjectId> {
        let password_hash =
            hash_password(&password).map_err(|err| AppError::InternalServerError {
                msg: err.to_string(),
                source_error: err,
            })?;

        let result = UserBuilder::new(self.database_service.clone())
            .first_name(first_name)
            .last_name(last_name)
            .username(username)
            .password_hash(password_hash)
            .admin(admin)
            .build(Some(transaction))
            .await
            .map_err(|err| AppError::InternalServerError {
                msg: err.to_string(),
                source_error: ServiceAppError::from(err),
            })?;
        Ok(result.get_id().clone())
    }
}
