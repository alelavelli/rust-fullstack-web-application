use thiserror::Error;

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Error, Debug)]
pub enum ApiError {
    /// Equivalent to 422
    #[error("The request body contains invalid JSON: {0}")]
    JsonRejection(String),
    /// Equivalent to 500
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    /// Equivalent to 401
    #[error("Authorization Error: {0}")]
    AuthorizationError(String),
    /// Equivalent to 404
    #[error("The requested resource does not exist: {0}")]
    DoesNotExist(String),
    /// Equivalent to 403
    #[error("Missing required permissions: {0}")]
    AccessControlError(String),
    /// Equivalent to 400
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    /// Generic request error for all other cases
    #[error("Generic request error: {0}")]
    GenericRequestError(String),
    /// Request error from the gloo library
    #[error("Request error")]
    RequestError(#[from] gloo_net::Error),
}

impl ApiError {
    pub fn from_status(status: u16, msg: String) -> Option<Self> {
        match status {
            400 => Some(ApiError::AccessControlError(msg)),
            401 => Some(ApiError::AuthorizationError(msg)),
            403 => Some(ApiError::AccessControlError(msg)),
            404 => Some(ApiError::DoesNotExist(msg)),
            422 => Some(ApiError::JsonRejection(msg)),
            500..600 => Some(ApiError::InternalServerError(msg)),
            200..300 => None,
            _ => Some(ApiError::GenericRequestError(msg)),
        }
    }
}
