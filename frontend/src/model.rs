use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthClaim {
    pub expiration: u32,
    pub user_id: String,
    pub username: String,
}

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub creation_date: String,
    pub creator_id: String,
    pub creator_username: String,
}

#[derive(Clone, PartialEq, Serialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggedUserInfo {
    pub token: String,
    pub user_id: String,
    pub username: String,
    #[serde(default)]
    pub admin: Option<bool>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublishPostRequest {
    pub title: String,
    pub content: String,
}
