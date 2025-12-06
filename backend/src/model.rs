use crate::database_document;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};

database_document!(
    #[doc = "User document"]
    User,
    "user",
    first_name: String,
    last_name: String,
    username: String,
    password_hash: String,
    admin: bool,
    publisher: bool
);

database_document!(
    #[doc = "Blog post document"]
    BlogPost,
    "blog_post",
    title: String,
    content: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    creation_date: DateTime<Utc>,
    user_id: ObjectId,
    username: String,
);
