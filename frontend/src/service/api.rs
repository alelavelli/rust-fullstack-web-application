use gloo_net::http::Request;
use log::info;

use crate::{error::ApiResult, model::LoggedUserInfo, service::auth::AuthService};

#[derive(Clone, Debug, PartialEq)]
pub struct ApiService {
    api_url: String,
    token: Option<String>,
}

impl ApiService {
    pub fn new(api_url: String) -> ApiService {
        let token = AuthService::new().get_token().unwrap();
        Self { api_url, token }
    }

    pub async fn login(&self, username: String, password: String) -> ApiResult<LoggedUserInfo> {
        info!(
            "Login with username: {username} and password: {password}",
            username = username,
            password = password
        );

        let mut url = String::from(&self.api_url);
        url.push_str("/guest/login");

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        info!("Response code is {response}", response = &response.status());

        Ok(response.json::<LoggedUserInfo>().await?)
    }
}
