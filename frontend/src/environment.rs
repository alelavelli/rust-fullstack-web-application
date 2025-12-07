//! Environment module provides a trait the defines the behavior of
//! the EnvironmentService and defines the struct containing application
//! environment variables.

use dotenv_codegen::dotenv;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnvironmentService {
    pub api_url: String,
    pub mock: bool,
    pub token_storage_location_name: String,
}

impl EnvironmentService {
    pub fn new() -> EnvironmentService {
        Self {
            api_url: dotenv!("FE_API_URL").to_string(),
            mock: dotenv!("FE_MOCK").to_lowercase() == "true",
            token_storage_location_name: dotenv!("FE_TOKEN_STORAGE_LOCATION_NAME").to_string(),
        }
    }
}
