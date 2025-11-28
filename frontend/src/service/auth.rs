use gloo_storage::{LocalStorage, Storage, errors::StorageError};
use jsonwebtoken::dangerous::insecure_decode;
use yew::UseStateHandle;

use crate::{
    model::{JWTAuthClaim, LoggedUserInfo},
    types::AppContext,
};

pub struct AuthService {
    app_context: UseStateHandle<AppContext>,
    storage_location_name: String,
}

impl AuthService {
    pub fn new(app_context: UseStateHandle<AppContext>) -> Self {
        Self {
            app_context,
            storage_location_name: "hello_blog_token".into(),
        }
    }

    fn load_token(&self) -> Result<Option<String>, StorageError> {
        match LocalStorage::get(&self.storage_location_name) {
            Err(StorageError::KeyNotFound(_)) => Ok(None),
            other => other,
        }
    }

    pub fn remove_logged_user(&self) {
        LocalStorage::delete(&self.storage_location_name);
        self.app_context.set(AppContext { user_info: None });
    }

    pub fn set_logged_user_info(&self, info: LoggedUserInfo) -> Result<(), StorageError> {
        LocalStorage::set(&self.storage_location_name, info.token.clone())?;
        self.app_context.set(AppContext {
            user_info: Some(info),
        });
        Ok(())
    }

    pub fn load_logged_user_info(&self) -> Option<LoggedUserInfo> {
        if let Some(token) = self.load_token().unwrap_or(None) {
            // verify the token in not expired
            let insecure_decoded_claims = insecure_decode::<JWTAuthClaim>(&token).unwrap().claims;

            let now = chrono::offset::Local::now().timestamp() as u32;

            if insecure_decoded_claims.expiration >= now {
                Some(LoggedUserInfo { token })
            } else {
                self.remove_logged_user();
                None
            }
        } else {
            None
        }
    }

    pub fn get_auth_token(&self) -> Option<String> {
        self.app_context
            .user_info
            .as_ref()
            .map(|info| info.token.clone())
    }
}
