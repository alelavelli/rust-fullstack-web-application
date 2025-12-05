use std::str::FromStr;

/// Logging configuration variables
#[derive(Debug, Clone)]
struct LoggingVariables {
    level: tracing::Level,
    include_headers: bool,
}

#[derive(Clone)]
pub struct EnvironmentService {
    logging: LoggingVariables,
    files_path: String,
}

impl Default for EnvironmentService {
    fn default() -> Self {
        Self::new()
    }
}

impl EnvironmentService {
    pub fn new() -> Self {
        EnvironmentService {
            logging: Self::build_logging(),
            files_path: std::env::var("FILES_PATH").expect("FILES_PATH env variable is missing"),
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

    pub fn get_logging_include_headers(&self) -> bool {
        self.logging.include_headers
    }

    pub fn get_logging_level(&self) -> tracing::Level {
        self.logging.level
    }

    pub fn get_files_path(&self) -> &str {
        &self.files_path
    }
}
