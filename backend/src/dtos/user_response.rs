use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    pub title: String,
    pub content: String,
    pub creator: String,
    pub creation_date: DateTime<Utc>,
}

impl From<model::BlogPost> for BlogPost {
    fn from(value: model::BlogPost) -> Self {
        Self {
            title: value.title().clone(),
            content: value.content().clone(),
            creator: value.user_id().to_string(),
            creation_date: *value.creation_date(),
        }
    }
}
