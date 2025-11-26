use gloo_storage::{LocalStorage, Storage, errors::StorageError};

#[derive(Default)]
pub struct AuthService {
    storage_location_name: String,
}

impl AuthService {
    pub fn new() -> Self {
        Self {
            storage_location_name: "hello_blog_token".into(),
        }
    }

    pub fn set_token(&self, token: String) -> Result<(), StorageError> {
        LocalStorage::set(&self.storage_location_name, token)
    }

    pub fn get_token(&self) -> Result<Option<String>, StorageError> {
        match LocalStorage::get(&self.storage_location_name) {
            Err(StorageError::KeyNotFound(_)) => Ok(None),
            other => other,
        }
    }

    pub fn delete_token(&self) {
        LocalStorage::delete(&self.storage_location_name)
    }
}
