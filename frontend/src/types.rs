use crate::{enums::HttpStatus, error::ApiError, model::LoggedUserInfo};

/// Struct containing general context for the application
///
/// it is shared among all the components and contains the
/// logged user informations
#[derive(Clone, Debug, PartialEq)]
pub struct AppContext {
    pub user_info: Option<LoggedUserInfo>,
}

impl AppContext {
    pub fn new(user_info: Option<LoggedUserInfo>) -> AppContext {
        AppContext { user_info }
    }
}

pub struct ApiResponse<T> {
    pub body: T,
    pub status: HttpStatus,
}

pub type ApiResult<T> = Result<ApiResponse<T>, ApiError>;
