use crate::{
    error::ApiError,
    model::{BlogPost, JWTAuthClaim, LoggedUserInfo, LoginInfo, PublishPostRequest, UserInfo},
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
            (
                Some(LoggedUserInfo {
                    token,
                    user_id: "user-id".into(),
                    username: "username".into(),
                    admin: Some(true),
                }),
                200,
            )
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

    pub async fn get_user_info(&self) -> ApiResult<Option<LoggedUserInfo>> {
        if let Some(token) = &self.token {
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
                (
                    Some(LoggedUserInfo {
                        token,
                        user_id: "user-id".into(),
                        username: "username".into(),
                        admin: Some(true),
                    }),
                    200,
                )
            } else {
                let mut url = String::from(&self.api_url);
                url.push_str("/user/info");

                let response = Request::get(&url)
                    .header("Content=Type", "application/json")
                    .header("Authorization", &format!("Bearer {token}"))
                    .send()
                    .await?;

                let body = if response.status() == 200 {
                    response.json::<Option<LoggedUserInfo>>().await?
                } else {
                    None
                };
                (body, response.status())
            };

            Ok(ApiResponse {
                body,
                status: status.into(),
            })
        } else {
            Err(ApiError::AuthorizationError(
                "Missing authorization token".to_string(),
            ))
        }
    }

    pub async fn get_posts(&self) -> ApiResult<Vec<BlogPost>> {
        if let Some(token) = &self.token {
            let (body, status) = if self.mock {
                (vec![
                BlogPost {
                    id: "1".into(),
                    title: "First blog".into(),
                    content: "this is the content of the blog. I think that I could write something but for now I can simply write a long text just to see how it will be displayed on the screen. Who know how it will be printed on the screen.".into(),
                    creation_date: "2025/11/14".into(),
                    creator_id: "creator-id".into(),
                    creator_username: "alex_sinks".into()
                },
                BlogPost {
                    id: "2".into(),
                    title: "Second blog".into(),
                    content: "this is the content of the blog. I think that I could write something but for now I can simply write a long text just to see how it will be displayed on the screen. Who know how it will be printed on the screen.".into(),
                    creation_date: "2025/11/14".into(),
                    creator_id: "creator-id".into(),
                    creator_username: "alex_sinks".into()
                },
                BlogPost {
                    id: "3".into(),
                    title: "Third blog".into(),
                    content: "this is the content of the blog. I think that I could write something but for now I can simply write a long text just to see how it will be displayed on the screen. Who know how it will be printed on the screen.".into(),
                    creation_date: "2025/11/14".into(),
                    creator_id: "creator-id".into(),
                    creator_username: "alex_sinks".into()
                }
            ], 200)
            } else {
                let mut url = String::from(&self.api_url);
                url.push_str("/user/blog/post");

                let response = Request::get(&url)
                    .header("Content=Type", "application/json")
                    .header("Authorization", &format!("Bearer {token}"))
                    .send()
                    .await?;

                let body = if response.status() == 200 {
                    response.json::<Vec<BlogPost>>().await?
                } else {
                    Vec::new()
                };
                (body, response.status())
            };

            Ok(ApiResponse {
                body,
                status: status.into(),
            })
        } else {
            Err(ApiError::AuthorizationError(
                "Missing authorization token".to_string(),
            ))
        }
    }

    pub async fn publish_post(&self, title: String, content: String) -> ApiResult<()> {
        if let Some(token) = &self.token {
            let (body, status) = if self.mock {
                ((), 200)
            } else {
                let mut url = String::from(&self.api_url);
                url.push_str("/user/blog/post");

                let request_payload = PublishPostRequest { title, content };

                let response = Request::post(&url)
                    .header("Content=Type", "application/json")
                    .header("Authorization", &format!("Bearer {token}"))
                    .json(&request_payload)?
                    .send()
                    .await?;

                ((), response.status())
            };

            Ok(ApiResponse {
                body,
                status: status.into(),
            })
        } else {
            Err(ApiError::AuthorizationError(
                "Missing authorization token".to_string(),
            ))
        }
    }

    pub async fn get_admin_users_list(&self) -> ApiResult<Vec<UserInfo>> {
        if let Some(token) = &self.token {
            let (body, status) = if self.mock {
                (
                    vec![
                        UserInfo {
                            user_id: "user-0".into(),
                            username: "username-0".into(),
                            admin: false,
                        },
                        UserInfo {
                            user_id: "user-1".into(),
                            username: "username-1".into(),
                            admin: true,
                        },
                        UserInfo {
                            user_id: "user-2".into(),
                            username: "username-2".into(),
                            admin: false,
                        },
                    ],
                    200,
                )
            } else {
                let mut url = String::from(&self.api_url);
                url.push_str("/admin/user");

                let response = Request::get(&url)
                    .header("Content=Type", "application/json")
                    .header("Authorization", &format!("Bearer {token}"))
                    .send()
                    .await?;

                let body = if response.status() == 200 {
                    response.json::<Vec<UserInfo>>().await?
                } else {
                    Vec::new()
                };
                (body, response.status())
            };

            Ok(ApiResponse {
                body,
                status: status.into(),
            })
        } else {
            Err(ApiError::AuthorizationError(
                "Missing authorization token".to_string(),
            ))
        }
    }
}
