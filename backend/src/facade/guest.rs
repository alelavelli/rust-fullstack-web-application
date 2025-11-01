use std::sync::Arc;

use jsonwebtoken::Header;

use crate::{
    AppResult, AppState,
    auth::JWTAuthClaim,
    dtos::guest_response::{self, JWTAuthResponse},
    error::{AppError, ServiceAppError},
    service::{
        UserService,
        database::{DatabaseServiceTrait, document::DatabaseDocumentTrait},
    },
    types::AppJson,
};

pub struct GuestFacade<T: DatabaseServiceTrait> {
    state: Arc<AppState<T>>,
}

impl<T: DatabaseServiceTrait> GuestFacade<T> {
    pub fn new(state: Arc<AppState<T>>) -> Self {
        Self { state }
    }

    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
    ) -> AppResult<guest_response::JWTAuthResponse> {
        let user = UserService::login(&self.state.database_service, username, password)
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

        let exp = self
            .state
            .environment_service
            .get_authentication_jwt_expiration();
        let now = chrono::offset::Local::now().timestamp();
        let claims: JWTAuthClaim<T> = JWTAuthClaim::new(
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

        Ok(AppJson(JWTAuthResponse {
            token,
            token_type: "Bearer".into(),
        }))
    }
}
