use std::sync::Arc;

use bson::oid::ObjectId;
use tokio::sync::RwLock;

use crate::{
    auth::AuthInfo,
    dtos::guest_response::LoggedUserInfoResponse,
    error::{AppError, DatabaseError, FacadeResult, ServiceAppError},
    model::{BlogPost, User},
    service::{
        access_control::AccessControl,
        blog::BlogService,
        database::{
            DatabaseServiceTrait, document::DatabaseDocumentTrait,
            smart_document::SmartDocumentReference,
        },
        user::UserService,
    },
};

pub struct UserFacade<D>
where
    D: DatabaseServiceTrait,
{
    access_control: AccessControl<D>,
    user: User,
    database_service: Arc<D>,
}

impl<D> UserFacade<D>
where
    D: DatabaseServiceTrait,
{
    pub async fn new<T: AuthInfo>(auth_info: T, database_service: Arc<D>) -> FacadeResult<Self> {
        let user_reference = Arc::new(RwLock::new(SmartDocumentReference::Id(
            *auth_info.user_id(),
        )));

        let access_control = AccessControl::new(user_reference.clone(), database_service.clone())
            .await
            .map_err(|err| match err {
                ServiceAppError::AccessControlError(msg) => AppError::AccessControlError(msg),
                other => AppError::InternalServerError {
                    msg: other.to_string(),
                    source_error: other,
                },
            })?;

        let mut user_service = UserService::new(
            Arc::new(RwLock::new(SmartDocumentReference::<User>::from(
                auth_info.user_id(),
            ))),
            database_service.clone(),
        );
        let user = user_service.get().await.map_err(|err| match err {
            ServiceAppError::DatabaseError(DatabaseError::DocumentDoesNotExist(object_id)) => {
                AppError::AccessControlError(format!("User with id {object_id} is not valid"))
            }
            other => AppError::InternalServerError {
                msg: format!(
                    "Error encountered in UserService::get where id is {}",
                    auth_info.user_id()
                ),
                source_error: other,
            },
        })?;

        Ok(Self {
            access_control,
            user: user.clone(),
            database_service,
        })
    }

    pub async fn get_info(&self) -> FacadeResult<LoggedUserInfoResponse> {
        Ok(LoggedUserInfoResponse {
            token: None,
            user_id: self.user.get_id().to_string(),
            username: self.user.username().clone(),
            admin: *self.user.admin(),
        })
    }

    pub async fn publish_post(
        &self,
        transaction: Arc<RwLock<D::Transaction>>,
        title: String,
        content: String,
    ) -> FacadeResult<String> {
        self.access_control
            .is_publisher_ref()
            .await
            .map_err(|err| match err {
                ServiceAppError::AccessControlError(msg) => AppError::AccessControlError(msg),
                other => AppError::InternalServerError {
                    msg: other.to_string(),
                    source_error: other,
                },
            })?;

        let blog_service = BlogService::new(self.database_service.clone(), Some(transaction));
        let blog_post = blog_service
            .publish_post(
                title,
                content,
                *self.user.get_id(),
                self.user.username().to_string(),
                chrono::offset::Utc::now(),
            )
            .await
            .map_err(|err| match err {
                ServiceAppError::InvalidRequest(msg) => AppError::InvalidRequest(msg),
                other => AppError::InternalServerError {
                    msg: "Error in building BlogPost document".into(),
                    source_error: other,
                },
            })?;

        Ok(blog_post.get_id().to_hex())
    }

    pub async fn get_posts(&self, user_id: Option<ObjectId>) -> FacadeResult<Vec<BlogPost>> {
        let blog_service = BlogService::new(self.database_service.clone(), None);
        blog_service
            .get_posts(user_id)
            .await
            .map_err(|err| match err {
                ServiceAppError::DoesNotExist(msg) => AppError::DoesNotExist(msg),
                other => AppError::InternalServerError {
                    msg: "Error in retrieving posts".into(),
                    source_error: other,
                },
            })
    }
}
