use gloo_net::http::Request;

use crate::{
    model::{LoggedUserInfo, LoginInfo},
    service::auth::AuthService,
    types::{ApiResponse, ApiResult},
};

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

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> ApiResult<Option<LoggedUserInfo>> {
        let login_info = LoginInfo { username, password };

        let mut url = String::from(&self.api_url);
        url.push_str("/guest/login");

        let response = Request::post(&url)
            .header("Content-Type", "application/json")
            .json(&login_info)?
            .send()
            .await?;

        let body = if response.status() == 200 {
            Some(response.json::<LoggedUserInfo>().await?)
        } else {
            None
        };

        Ok(ApiResponse {
            body,
            status: response.status(),
        })
    }
}
