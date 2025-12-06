use serde::Serialize;

use crate::{model, service::database::document::DatabaseDocumentTrait};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub admin: bool,
}

impl From<model::User> for User {
    fn from(value: model::User) -> Self {
        Self {
            user_id: value.get_id().to_hex(),
            first_name: value.first_name().clone(),
            last_name: value.last_name().clone(),
            username: value.username().clone(),
            admin: *value.admin(),
        }
    }
}
