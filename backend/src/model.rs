use crate::database_document;

database_document!(
    #[doc = "User document"]
    User,
    "user",
    first_name: String,
    last_name: String,
    username: String,
    password_hash: String,
    admin: bool,
);
