//! Environment module provides a trait the defines the behavior of
//! the EnvironemntService and defines the struct containing application
//! environment variables.

use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::enums::{FrontendMode, ObjectSourceType};

mod service;
pub use service::EnvironmentService;

/// Trait that defines the behavior of an Environment Service
///
/// It defines methods to get the information the application may need
/// and that needs to be defined in one place during the application
/// initialization.
pub trait EnvironmentServiceTrait {
    fn get_database_connection_string(&self) -> &str;

    fn get_database_db_name(&self) -> &str;

    fn get_authentication_jwt_expiration(&self) -> usize;

    fn get_authentication_jwt_encoding(&self) -> &EncodingKey;

    fn get_authentication_jwt_decoding(&self) -> &DecodingKey;

    fn get_logging_include_headers(&self) -> bool;

    fn get_logging_level(&self) -> tracing::Level;

    fn get_object_storage_source_type(&self) -> &ObjectSourceType;

    fn get_object_storage_prefix_path(&self) -> &str;

    fn get_frontend_mode(&self) -> &FrontendMode;
}

/// Logging configuration variables
#[derive(Debug, Clone)]
struct LoggingVariables {
    level: tracing::Level,
    include_headers: bool,
}

/// Authentication configuration variables used for JWT
#[derive(Clone)]
struct AuthenticationVariables {
    jwt_expiration: usize,
    jwt_encoding: EncodingKey,
    jwt_decoding: DecodingKey,
}

/// Configuration variables used to application's objects
#[derive(Clone)]
struct ObjectStorageVariables {
    storage_backend: ObjectSourceType,
    prefix_path: String,
}

/// Frontend configuration variabels
///
/// Frontend can be served as static content from the web server or
/// as an external entity. These variables defines which type of
/// configuration is used.
///
/// FrontendMode::Internal contains a string variable that represents
/// the path of the root folder containing static files
#[derive(Debug, Clone)]
struct FrontendVariables {
    frontend_mode: FrontendMode,
}

/// Database configuration with connection string and database name
#[derive(Debug, Clone)]
struct DatabaseVariables {
    connection_string: String,
    db_name: String,
}
