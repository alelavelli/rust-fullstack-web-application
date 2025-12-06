use serde::Serialize;

/// Authorization response for jwt token
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTAuthResponse {
    pub token: String,
    pub user_id: String,
    pub username: String,
    pub admin: Option<bool>,
}
