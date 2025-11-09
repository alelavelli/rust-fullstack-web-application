use std::sync::Arc;

use crate::{
    ServiceResult,
    auth::AuthInfo,
    error::{DatabaseError, ServiceAppError},
    model::User,
    service::{
        database::{DatabaseServiceTrait, smart_document::SmartDocumentReference},
        user::UserService,
    },
};

/// Access control service is a secondary service used to
/// verify user permissions.
///
/// It contains a `UserService` instance to operate on the user and retrieve
/// the required information.
pub struct AccessControl<D: DatabaseServiceTrait> {
    user_service: UserService<D>,
}

impl<D: DatabaseServiceTrait> AccessControl<D> {
    /// From the auth_info it retrieve the user id and verify that it exists.
    ///
    /// If the user does not exist then a `ServiceAppError::AccessControlError`
    /// is returned.
    pub async fn new<T: AuthInfo>(
        auth_info: T,
        database_service: Arc<D>,
    ) -> ServiceResult<AccessControl<D>> {
        let mut user_service = UserService::new(
            SmartDocumentReference::<User>::from(auth_info.user_id()),
            database_service,
        );

        user_service.get().await.map_err(|err| match err {
            ServiceAppError::DatabaseError(DatabaseError::DocumentDoesNotExist(object_id)) => {
                ServiceAppError::AccessControlError(format!(
                    "User with id {object_id} is not valid"
                ))
            }
            _ => ServiceAppError::InternalServerError(format!(
                "Error encountered in UserService::get where id is {}",
                auth_info.user_id()
            )),
        })?;

        Ok(Self { user_service })
    }

    /// If the user is platform admin it returns self otherwise it returns
    /// and error
    pub async fn is_platform_admin(mut self) -> ServiceResult<Self> {
        let user = self.user_service.get().await?;
        if !user.admin() {
            Err(ServiceAppError::AccessControlError(
                "User is not admin".into(),
            ))
        } else {
            Ok(self)
        }
    }
}
