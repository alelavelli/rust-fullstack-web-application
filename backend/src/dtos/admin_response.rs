use serde::Serialize;

use crate::{model, service::database::document::DatabaseDocumentTrait};

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

impl From<model::User> for User {
    fn from(value: model::User) -> Self {
        Self {
            id: value.get_id().to_hex(),
            first_name: value.first_name().clone(),
            last_name: value.last_name().clone(),
            username: value.username().clone(),
        }
    }
}
