use crate::{
    model::{JWTAuthClaim, LoggedUserInfo, LoginInfo},
    types::{ApiResponse, ApiResult},
};
use gloo_net::http::Request;
use jsonwebtoken::{EncodingKey, Header, encode};

#[derive(Clone, Debug, PartialEq)]
pub struct ApiService {
    api_url: String,
    token: Option<String>,
    mock: bool,
}

impl ApiService {
    pub fn new(api_url: String, mock: bool, token: Option<String>) -> ApiService {
        Self {
            api_url,
            token,
            mock,
        }
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> ApiResult<Option<LoggedUserInfo>> {
        let (body, status) = if self.mock {
            let now = chrono::offset::Local::now().timestamp() as u32;
            let claims = JWTAuthClaim {
                user_id: "user-id".into(),
                username: "username".into(),
                expiration: now + 10000,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret("secret".as_ref()),
            )
            .expect("failing to mock jwt");
            (Some(LoggedUserInfo { token }), 200)
        } else {
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
            (body, response.status())
        };

        Ok(ApiResponse {
            body,
            status: status.into(),
        })
    }
}
