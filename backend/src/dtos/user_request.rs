use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishPost {
    pub title: String,
    pub content: String,
}
