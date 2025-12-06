use serde::Serialize;

/// Authorization response for jwt token
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggedUserInfoResponse {
    // it is Some from the login and register requests but empty for the get_user_info
    pub token: Option<String>,
    pub user_id: String,
    pub username: String,
    pub admin: bool,
}
