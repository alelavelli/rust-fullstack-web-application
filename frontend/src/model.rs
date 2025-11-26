use serde::{Deserialize, Serialize};

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
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoggedUserInfo {
    pub token: String,
    pub token_type: String,
}
