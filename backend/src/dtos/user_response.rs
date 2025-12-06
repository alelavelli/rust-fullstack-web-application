use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{model, service::database::document::DatabaseDocumentTrait};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub creator_id: String,
    pub creator_username: String,
    pub creation_date: DateTime<Utc>,
}

impl From<model::BlogPost> for BlogPost {
    fn from(value: model::BlogPost) -> Self {
        Self {
            id: value.get_id().to_string(),
            title: value.title().clone(),
            content: value.content().clone(),
            creator_id: value.user_id().to_string(),
            creator_username: value.username().to_string(),
            creation_date: *value.creation_date(),
        }
    }
}
