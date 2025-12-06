use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use jsonwebtoken::dangerous::insecure_decode;
use log::error;
use yew::UseStateHandle;

use crate::{
    enums::HttpStatus,
    environment::EnvironmentService,
    model::{JWTAuthClaim, LoggedUserInfo},
    service::api::ApiService,
    types::{ApiResponse, AppContext, AppResult},
};

/// Service responsible to manage the logged user session
/// and its authentication information
///
/// It updates the application context with the logged user
/// and manage the local storage for the token
#[derive(Clone)]
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
        // TODO: add a condition that if the app context already has the user info then don't do anything
        if let Some(token) = self.load_token().unwrap_or(None) {
            // verify the token in not expired
            let insecure_decoded_claims = insecure_decode::<JWTAuthClaim>(&token).unwrap().claims;

            let now = chrono::offset::Local::now().timestamp() as u32;

            if insecure_decoded_claims.exp >= now {
                if self.app_context.user_info.is_some() {
                    // if the user is already in the context then we just return avoiding making an additional request
                    return;
                }
                // We make an api request to get the actual user information to validate the
                // ones read from the local storage. Moreover, this api request returns the
                // information if the user is admin
                let environment_service = EnvironmentService::new();
                let api_service = ApiService::new(
                    environment_service.api_url,
                    environment_service.mock,
                    Some(token.clone()),
                );

                let app_context = self.app_context.clone();
                let self_clone = self.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let response = api_service.get_user_info().await;
                    if let Ok(ApiResponse { body, status }) = response {
                        match status {
                            HttpStatus::Success(_) => {
                                let body = body.expect("Body must be present when it is success");
                                let logged_user_info = LoggedUserInfo {
                                    token,
                                    user_id: body.user_id,
                                    username: body.username,
                                    admin: body.admin,
                                };
                                app_context.set(AppContext::new(Some(logged_user_info)));
                            }
                            _ => {
                                self_clone.remove_logged_user();
                                error!("Response returned status {status}", status = status);
                            }
                        }
                    } else {
                        self_clone.remove_logged_user();
                        error!(
                            "Encountered an error in get user info request. Error {err}",
                            err = response.err().unwrap()
                        );
                    }
                });
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
