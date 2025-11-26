use crate::{
    environment::EnvironmentService, error::ApiError, model::LoggedUserInfo,
    service::api::ApiService,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub environment_service: EnvironmentService,
    pub api_service: ApiService,
    pub user_info: Option<LoggedUserInfo>,
}

impl AppContext {
    pub fn new(
        environment_service: EnvironmentService,
        api_service: ApiService,
        user_info: Option<LoggedUserInfo>,
    ) -> AppContext {
        AppContext {
            environment_service,
            api_service,
            user_info,
        }
    }
}

pub struct ApiResponse<T> {
    pub body: T,
    pub status: u16,
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
