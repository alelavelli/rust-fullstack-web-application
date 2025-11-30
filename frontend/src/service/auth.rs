use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use jsonwebtoken::dangerous::insecure_decode;
use yew::UseStateHandle;

use crate::{
    model::{JWTAuthClaim, LoggedUserInfo},
    types::{AppContext, AppResult},
};

/// Service responsible to manage the logged user session
/// and its authentication information
///
/// It updates the application context with the logged user
/// and manage the local storage for the token
pub struct AuthService {
    app_context: UseStateHandle<AppContext>,
    token_storage_location_name: String,
}

impl AuthService {
    pub fn new(
        token_storage_location_name: String,
        app_context: UseStateHandle<AppContext>,
    ) -> Self {
        Self {
            app_context,
            token_storage_location_name,
        }
    }

    fn load_token(&self) -> AppResult<Option<String>> {
        match LocalStorage::get(&self.token_storage_location_name) {
            Err(StorageError::KeyNotFound(_)) => Ok(None),
            other => other.map_err(|e| e.into()),
        }
    }

    /// Delete the token from the local storage and the info from the
    /// application context
    pub fn remove_logged_user(&self) {
        LocalStorage::delete(&self.token_storage_location_name);
        self.app_context.set(AppContext { user_info: None });
    }

    /// Given the logged user info, it stores the token on the local storage
    /// and update the application context with them
    pub fn set_logged_user_info(&self, info: LoggedUserInfo) -> AppResult<()> {
        LocalStorage::set(&self.token_storage_location_name, info.token.clone())?;
        self.app_context.set(AppContext {
            user_info: Some(info),
        });
        Ok(())
    }

    /// Tries to retrieve the token from the local storage and update the
    /// application context.
    ///
    /// If the token is not present or it is expired then the user info are
    /// removed from the context
    pub fn set_logged_user_info_from_storage(&self) {
        if let Some(token) = self.load_token().unwrap_or(None) {
            // verify the token in not expired
            let insecure_decoded_claims = insecure_decode::<JWTAuthClaim>(&token).unwrap().claims;

            let now = chrono::offset::Local::now().timestamp() as u32;

            // TODO: do not use the insecure claim but make an API request
            if insecure_decoded_claims.expiration >= now {
                let logged_user_info = LoggedUserInfo {
                    token,
                    user_id: insecure_decoded_claims.user_id,
                    username: insecure_decoded_claims.username,
                };

                if let Some(context_user_info) = &self.app_context.user_info {
                    if logged_user_info.token != context_user_info.token {
                        self.app_context
                            .set(AppContext::new(Some(logged_user_info)));
                    }
                } else {
                    self.app_context
                        .set(AppContext::new(Some(logged_user_info)));
                }
            } else {
                self.remove_logged_user();
            }
        }
    }

    /// Returns the auth token from the application context
    ///
    /// Note that it does not load from the local storage any information
    pub fn get_auth_token(&self) -> Option<String> {
        self.app_context
            .user_info
            .as_ref()
            .map(|info| info.token.clone())
    }
}
