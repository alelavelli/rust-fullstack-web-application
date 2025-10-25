use std::str::FromStr;

use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::{
    enums::{FrontendMode, ObjectSourceType},
    environment::{
        AuthenticationVariables, DatabaseVariables, EnvironmentServiceTrait, FrontendVariables,
        LoggingVariables, ObjectStorageVariables,
    },
};

/// Basic environment service that initialize all the application
/// environment variables from the system environment variables
///
/// Required variables are:
///
/// - LOGGING_LEVEL: the logging level to use in the application
/// - DEPLOY_ENVIRONMENT: in which context the application is running, it is used to specify some resources according to it
/// - JWT_SECRET: string used as secret to sign jwt
/// - JWT_EXPIRATION: time in seconds of the duration of a jwt
/// - MONGODB_CONNECTION_STRING: authenticated connection string to the mongodb cluster
/// - MONGODB_DB_NAME: name of mongodb database to use as prefix by adding deploy environment
/// - OBJECT_STORAGE_BACKEND: which type of backend to use as object storage
/// - OBJECT_STORAGE_PREFIX_PATH: prefix path to store objects. In case of remote object storage it contains also the bucket name
/// - FRONTEND_MODE: whether to work with integrated frontend or as external service
#[derive(Clone)]
pub struct EnvironmentService {
    logging: LoggingVariables,
    authentication: AuthenticationVariables,
    database: DatabaseVariables,
    storage: ObjectStorageVariables,
    frontend: FrontendVariables,
}

impl Default for EnvironmentService {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentService {
    pub fn new() -> Self {
        let deploy_environment = if cfg!(test) {
            "test".into()
        } else {
            std::env::var("DEPLOY_ENVIRONMENT").expect("DEPLOY_ENVIRONMENT must be set")
        };
        EnvironmentService {
            logging: Self::build_logging(),
            authentication: Self::build_authentication(),
            database: Self::build_database(&deploy_environment),
            storage: Self::build_storage(),
            frontend: Self::build_frontend(),
        }
    }

    fn build_logging() -> LoggingVariables {
        let level =
            tracing::Level::from_str(&std::env::var("LOGGING_LEVEL").unwrap_or("INFO".into()))
                .expect("Encountered an error while retrieving the logging level");

        let include_headers = std::env::var("LOGGING_INCLUDE_HEADERS")
            .unwrap_or("false".into())
            .to_lowercase()
            == "true";

        LoggingVariables {
            level,
            include_headers,
        }
    }

    fn build_authentication() -> AuthenticationVariables {
        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expiration = std::env::var("JWT_EXPIRATION").map_or(60 * 60 * 24, |s| {
            usize::from_str(&s).expect("JWT_EXPIRATION is not valid")
        });

        AuthenticationVariables {
            jwt_expiration,
            jwt_encoding: EncodingKey::from_secret(secret.as_bytes()),
            jwt_decoding: DecodingKey::from_secret(secret.as_bytes()),
        }
    }

    fn build_database(deploy_environment: &str) -> DatabaseVariables {
        let (connection_string, db_name) = if cfg!(test) {
            let db_name = format!("application-database-{}", deploy_environment);
            (
                format!(
                    "mongodb://localhost:27117/{}?replicaSet=rs0&directConnection=true",
                    db_name
                ),
                db_name,
            )
        } else {
            (
                std::env::var("MONGODB_CONNECTION_STRING")
                    .expect("MONGODB_CONNECTION_STRING must be set"),
                std::env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME must be set"),
            )
        };

        DatabaseVariables {
            connection_string,
            db_name,
        }
    }

    fn build_storage() -> ObjectStorageVariables {
        if cfg!(test) {
            ObjectStorageVariables {
                storage_backend: ObjectSourceType::LocalFileSystem,
                prefix_path: "../app-objects".into(),
            }
        } else {
            let storage_backend_var = std::env::var("OBJECT_STORAGE_BACKEND")
                .expect("OBJECT_STORAGE_BACKEND must be set.");
            ObjectStorageVariables {
                storage_backend: ObjectSourceType::try_from(storage_backend_var.as_str())
                    .unwrap_or_else(|_| {
                        panic!(
                            "Invalid OBJECT_STORAGE_BACKEND value. Got {}",
                            storage_backend_var
                        )
                    }),
                prefix_path: std::env::var("OBJECT_STORAGE_PREFIX_PATH")
                    .expect("OBJECT_STORAGE_PREFIX_PATH must be set"),
            }
        }
    }

    fn build_frontend() -> FrontendVariables {
        if cfg!(test) {
            FrontendVariables {
                frontend_mode: FrontendMode::External,
            }
        } else {
            let frontend_mode = std::env::var("FRONTEND_MODE").expect("FRONTEND_MODE must be set.");
            FrontendVariables {
                frontend_mode: FrontendMode::try_from(frontend_mode.as_str()).unwrap_or_else(
                    |_| panic!("Invalid FRONTEND_MODE value. Got `{frontend_mode}`"),
                ),
            }
        }
    }
}

impl EnvironmentServiceTrait for EnvironmentService {
    fn get_database_connection_string(&self) -> &str {
        &self.database.connection_string
    }

    fn get_database_db_name(&self) -> &str {
        &self.database.db_name
    }

    fn get_authentication_jwt_expiration(&self) -> usize {
        self.authentication.jwt_expiration
    }

    fn get_authentication_jwt_encoding(&self) -> &EncodingKey {
        &self.authentication.jwt_encoding
    }

    fn get_authentication_jwt_decoding(&self) -> &DecodingKey {
        &self.authentication.jwt_decoding
    }

    fn get_logging_include_headers(&self) -> bool {
        self.logging.include_headers
    }

    fn get_logging_level(&self) -> tracing::Level {
        self.logging.level
    }

    fn get_object_storage_source_type(&self) -> &ObjectSourceType {
        &self.storage.storage_backend
    }

    fn get_object_storage_prefix_path(&self) -> &str {
        &self.storage.prefix_path
    }

    fn get_frontend_mode(&self) -> &FrontendMode {
        &self.frontend.frontend_mode
    }
}
