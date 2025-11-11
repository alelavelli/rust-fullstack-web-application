use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    ServiceResult,
    error::{DatabaseError, ServiceAppError},
    model::User,
    service::database::{DatabaseServiceTrait, smart_document::SmartDocumentReference},
};

/// Access control service is a secondary service used to
/// verify user permissions.
///
/// It contains a `UserService` instance to operate on the user and retrieve
/// the required information.
pub struct AccessControl<D: DatabaseServiceTrait> {
    user: Arc<RwLock<SmartDocumentReference<User>>>,
    database_service: Arc<D>,
}

impl<D: DatabaseServiceTrait> AccessControl<D> {
    /// From the auth_info it retrieve the user id and verify that it exists.
    ///
    /// If the user does not exist then a `ServiceAppError::AccessControlError`
    /// is returned.
    pub async fn new(
        user: Arc<RwLock<SmartDocumentReference<User>>>,
        database_service: Arc<D>,
    ) -> ServiceResult<AccessControl<D>> {
        // first control to be done is to verify that the user exists, so
        // we use as_document_ref to query the database and verify it
        user.write()
            .await
            .as_document_ref(database_service.clone())
            .await
            .map_err(|err| match err {
                DatabaseError::DocumentDoesNotExist(object_id) => {
                    ServiceAppError::AccessControlError(format!(
                        "User with id {object_id} is not valid"
                    ))
                }
                other => ServiceAppError::DatabaseError(other),
            })?;

        Ok(Self {
            user,
            database_service,
        })
    }

    /// If the user is platform admin it returns self otherwise it returns
    /// an error
    ///
    /// It consumes the object and is used to chain operations
    pub async fn is_platform_admin(self) -> ServiceResult<Self> {
        self.is_platform_admin_ref().await?;
        Ok(self)
    }

    /// If the user is platform admin it returns self otherwise it returns
    /// an error.
    ///
    /// It perform access contro without consuming the object
    pub async fn is_platform_admin_ref(&self) -> ServiceResult<()> {
        if !self
            .user
            .write()
            .await
            .as_document_ref(self.database_service.clone())
            .await?
            .admin()
        {
            Err(ServiceAppError::AccessControlError(
                "User is not admin".into(),
            ))
        } else {
            Ok(())
        }
    }

    /// If the user has publisher role it returns self otherwise it returns
    /// an error
    pub async fn is_publisher(self) -> ServiceResult<Self> {
        self.is_publisher_ref().await?;
        Ok(self)
    }

    /// If the user has publisher role it returns self otherwise it returns
    /// an error
    pub async fn is_publisher_ref(&self) -> ServiceResult<()> {
        if !self
            .user
            .write()
            .await
            .as_document_ref(self.database_service.clone())
            .await?
            .publisher()
        {
            Err(ServiceAppError::AccessControlError(
                "User is not publisher".into(),
            ))
        } else {
            Ok(())
        }
    }
}
