use log::info;

#[derive(Clone, Debug, PartialEq)]
pub struct ApiService {
    api_url: String,
}

impl ApiService {
    pub fn new(api_url: String) -> ApiService {
        Self { api_url }
    }

    pub async fn login(&self, username: String, password: String) {
        info!(
            "Login with username: {username} and password: {password}",
            username = username,
            password = password
        );
    }
}
