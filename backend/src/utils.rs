use crate::{ServiceResult, error::ServiceAppError};

pub fn hash_password(password: &str) -> ServiceResult<String> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(|e| {
        ServiceAppError::InternalServerError(format!("Error in hashing password. Got {e}"))
    })
}
