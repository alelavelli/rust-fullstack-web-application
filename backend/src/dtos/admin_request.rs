use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
    pub admin: bool,
}
