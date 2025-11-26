//! Environment module provides a trait the defines the behavior of
//! the EnvironemntService and defines the struct containing application
//! environment variables.

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnvironmentService {
    api_url: String,
}

impl EnvironmentService {
    pub fn new() -> EnvironmentService {
        Self {
            api_url: std::env::var("API_URL").unwrap_or("http://localhost:3000".into()),
        }
    }

    pub fn get_api_url(&self) -> String {
        self.api_url.clone()
    }
}
