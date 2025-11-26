use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::JsValue;

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
    pub token_type: String,
}
