//! Environment module provides a trait the defines the behavior of
//! the EnvironemntService and defines the struct containing application
//! environment variables.

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnvironmentService {
    pub api_url: String,
    pub mock: bool,
    pub token_storage_location_name: String,
}

impl EnvironmentService {
    pub fn new() -> EnvironmentService {
        Self {
            api_url: std::env::var("API_URL").unwrap_or("http://localhost:3000".into()),
            mock: std::env::var("MOCK_API")
                .unwrap_or("true".into())
                .to_lowercase()
                == "true".to_string(),
            token_storage_location_name: std::env::var("TOKEN_STORAGE_LOCATION_NAME")
                .unwrap_or("hello_blog_token".into()),
        }
    }
}
