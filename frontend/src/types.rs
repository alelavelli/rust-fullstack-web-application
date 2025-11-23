use crate::{environment::EnvironmentService, service::api::ApiService};

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub environment_service: EnvironmentService,
    pub api_service: ApiService,
}

impl AppContext {
    pub fn new(environment_service: EnvironmentService, api_service: ApiService) -> AppContext {
        AppContext {
            environment_service,
            api_service,
        }
    }
}
