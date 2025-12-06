use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterInfo {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
}

/// Authorization payload for jwt token
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JWTAuthPayload {
    pub username: String,
    pub password: String,
}
