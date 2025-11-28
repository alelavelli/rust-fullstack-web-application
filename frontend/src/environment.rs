//! Environment module provides a trait the defines the behavior of
//! the EnvironemntService and defines the struct containing application
//! environment variables.

#[derive(Clone, Debug, PartialEq, Default)]
pub struct EnvironmentService {
    api_url: String,
    mock: bool,
}

impl EnvironmentService {
    pub fn new() -> EnvironmentService {
        Self {
            api_url: std::env::var("API_URL").unwrap_or("http://localhost:3000".into()),
            mock: std::env::var("MOCK_API")
                .unwrap_or("true".into())
                .to_lowercase()
                == "true".to_string(),
        }
    }

    pub fn get_api_url(&self) -> String {
        self.api_url.clone()
    }

    pub fn get_mock_api(&self) -> bool {
        self.mock
    }
}
