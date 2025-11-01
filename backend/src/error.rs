//! Error module defines the error types used by the application
//!
//! There are two categories of errors:
//!
//! `AppError` is used by facades and routers and implements `IntoResponse` trait
//! to be returned as HTTP response.
//!
//! `ServiceAppError` is the internal type of error and is used by services in
//! the application.
//!
//! `AuthError` is a variant of `ServiceAppError` specific for the authorization.
//!
//! Facades are responsible to translate `ServiceAppError` objects to `AppError` ones
//! according to the specific situation.
//!
//! Each enum variant has an associated Result type for syntactic sugar.  

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse};
use bson::oid::ObjectId;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

use crate::types::AppJson;

pub type AppResult<T> = Result<AppJson<T>, AppError>;
pub type ServiceResult<T> = Result<T, ServiceAppError>;
pub type AuthResult<T> = Result<T, AuthError>;
pub type DatabaseResult<T> = Result<T, DatabaseError>;

/// Enumeration of different error typologies that the application
/// can return to the client.
///
/// Each enum variant is associated with a reponse code.
#[derive(Error, Debug)]
pub enum AppError {
    /// Equivalent to 422
    #[error("The request body contains invalid JSON")]
    JsonRejection(#[from] JsonRejection),
    /// Equivalent to 500
    #[error("Internal server error: {msg}")]
    InternalServerError {
        msg: String,
        source_error: ServiceAppError,
    },
    /// Equivalent to 401
    #[error("Authorization Error: {0}")]
    AuthorizationError(#[from] AuthError),
    /// Equivalent to 404
    #[error("The requested resource does not exist: {0}")]
    DoesNotExist(String),
    /// Equivalent to 403
    #[error("Missing required permissions: {0}")]
    AccessControlError(String),
    /// Equivalent to 400
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        // Create a temporary support struct to be passed to AppJson
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        // For each enum variant we return the message and associated status code
        // Note that for some variants we ignore the message because we don't want
        // to provide it to the client.
        let (status, message) = match self {
            AppError::JsonRejection(rejection) => {
                // rejection object contains all the information about the
                // causes of this bad user input, so we translate it directly
                // to status code and message
                (rejection.status(), rejection.body_text())
            }
            AppError::InternalServerError { msg, source_error } => {
                // Here we don't return the message to the user but we log it
                error!(
                    msg,
                    source_error = source_error.to_string(),
                    error_type = "InternalServerError"
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".into(),
                )
            }
            AppError::AccessControlError(message) => (StatusCode::FORBIDDEN, message),
            AppError::DoesNotExist(message) => (StatusCode::NOT_FOUND, message),
            AppError::InvalidRequest(message) => (StatusCode::BAD_REQUEST, message),
            AppError::AuthorizationError(auth_error) => auth_error.to_status_message(),
        };
        (status, AppJson(ErrorResponse { message })).into_response()
    }
}

/// Enumeration of different internal error returned by application services
///
/// It is translated to an AppError according to the context, so that a DoesNotExist
/// does not necessarily corresponds to the equivalent AppError variant but can be
/// transalted into InternalServerError
#[derive(Error, Debug)]
pub enum ServiceAppError {
    /// Equivalent to 500
    #[error("Internal server error: {0}")]
    InternalServerError(String),
    /// Equivalent to 401
    #[error("Authorization error: {0}")]
    AuthorizationError(#[from] AuthError),
    /// Equivalent to 403
    #[error("Access control error: {0}")]
    AccessControlError(String), // TODO: expand the content of the error with user_id, required permission and current permission
    /// Error derived from interaction with database
    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
    /// Error occurred when building the reponse for the client
    #[error("Response build error: {0}")]
    ResponseBuildError(String), // TODO: expand the content with structured attributes
    /// Equivalent to 404
    #[error("Required resource does not exist: {0}")]
    DoesNotExist(String),
    /// Equivalent to 400
    /// It occurs when the request contains parameters that fail some service validation
    /// or when it cannot be done
    #[error("Request is not valid: {0}")]
    InvalidRequest(String),
    /// Error derived from the interaction with the object storage
    #[error("Object storage error: {0}")]
    ObjectStorageError(String),
    #[error("Application state error: {0}")]
    AppStateError(String),
}

#[derive(Error, Debug)]
pub enum AuthError {
    /// Equivalent to 500
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    /// Equivalent to 401
    #[error("Wrong credentials")]
    WrongCredentials,
    /// Equivalent to 400
    #[error("Missing credentials")]
    MissingCredentials,
    /// Error in jwt creation, so it is internal
    #[error("Error in JWT creation")]
    TokenCreation,
    /// Equivalent to 400
    #[error("Invalid JWT")]
    InvalidToken,
    /// Equivalent to 400
    #[error("Invalid API Key")]
    InvalidApiKey,
}

impl AuthError {
    fn to_status_message(&self) -> (StatusCode, String) {
        let (status, message) = match self {
            AuthError::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".into(),
            ),
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials".into()),
            AuthError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "Wrong credentials".into()),

            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, "Missing credentials".into())
            }
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Token creation error".into(),
            ),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token".into()),
        };
        (status, message)
    }
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("MongoDB API error: {0}")]
    MongoDBApiError(#[from] mongodb::error::Error),
    /// When an operation over a transaction fails
    #[error("Transaction is failed: {0}")]
    TransactionError(String),
    #[error("Document has already an associated id")]
    DocumentHasAlreadyAnId,
    #[error("Object id is invalid")]
    InvalidObjectId,
    #[error("Client is not connected to the cluster")]
    ClientNotConnected,
    #[error("Document with id {0} does not exist")]
    DocumentDoesNotExist(ObjectId),
    #[error("Document is not valid because it's missing some fields")]
    DocumentNotValid(String),
    #[error("DatabaseService is not available: {0}")]
    DatabaseServiceError(String),
    #[error("Error encountered during a database operation: {0}")]
    DatabaseOperationError(String),
}
