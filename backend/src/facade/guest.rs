use std::sync::Arc;

use jsonwebtoken::Header;
use tokio::sync::RwLock;

use crate::{
    AppResult, AppState,
    auth::JWTAuthClaim,
    dtos::guest_response::{self, JWTAuthResponse},
    error::{AppError, ServiceAppError},
    model::{User, UserBuilder},
    service::{
        database::{DatabaseServiceTrait, document::DatabaseDocumentTrait},
        user::UserService,
    },
    types::AppJson,
    utils::hash_password,
};

pub struct GuestFacade {
    state: Arc<AppState>,
}

impl GuestFacade {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }

    fn create_jwt(&self, user: &User) -> Result<String, AppError> {
        let exp = self
            .state
            .environment_service
            .get_authentication_jwt_expiration();
        let now = chrono::offset::Local::now().timestamp();
        let claims: JWTAuthClaim = JWTAuthClaim::new(
            now as u32 + exp as u32,
            *user.get_id(),
            user.username().clone(),
        );
        let token = claims.build_token(
            &Header::default(),
            self.state
                .environment_service
                .get_authentication_jwt_encoding(),
        )?;

        Ok(token)
    }

    pub async fn register_user<D: DatabaseServiceTrait>(
        &self,
        database_service: Arc<D>,
        transaction: Arc<RwLock<D::Transaction>>,
        first_name: String,
        last_name: String,
        username: String,
        password: String,
    ) -> AppResult<guest_response::JWTAuthResponse> {
        let password_hash =
            hash_password(&password).map_err(|err| AppError::InternalServerError {
                msg: err.to_string(),
                source_error: err,
            })?;

        let user = UserBuilder::new(database_service.clone())
            .first_name(first_name)
            .last_name(last_name)
            .username(username)
            .password_hash(password_hash)
            .admin(false)
            .publisher(true)
            .build(Some(transaction))
            .await
            .map_err(|err| AppError::InternalServerError {
                msg: err.to_string(),
                source_error: ServiceAppError::from(err),
            })?;

        let token = self.create_jwt(&user)?;

        Ok(AppJson(JWTAuthResponse {
            token,
            user_id: user.get_id().to_string(),
            username: user.username().to_string(),
            admin: Some(*user.admin()),
        }))
    }

    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> AppResult<guest_response::JWTAuthResponse> {
        let user = UserService::login(self.state.database_service.clone(), username, password)
            .await
            .map_err(|err| match err {
                ServiceAppError::AuthorizationError(auth_error) => {
                    AppError::AuthorizationError(auth_error)
                }
                any_other => AppError::InternalServerError {
                    msg: any_other.to_string(),
                    source_error: any_other,
                },
            })?;
        let token = self.create_jwt(&user)?;
        Ok(AppJson(JWTAuthResponse {
            token,
            user_id: user.get_id().to_string(),
            username: user.username().to_string(),
            admin: Some(*user.admin()),
        }))
    }
}
