use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub creation_date: String,
    pub creator_id: String,
    pub creator_username: String,
}
