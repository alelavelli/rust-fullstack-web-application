use std::sync::Arc;

use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{EncodingKey, Header, Validation, decode, encode};

use mongodb::bson::doc;
use tracing::error;

use crate::{
    AppState,
    error::{AppError, AuthError},
};

/// Trait for auth info objects that need to return specific information
pub trait AuthInfo: Clone {
    fn user_id(&self) -> &ObjectId;
}

/// Struct containing information that will be encoded inside the jwt token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JWTAuthClaim {
    pub expiration: u32,
    pub user_id: ObjectId,
    pub username: String,
}

impl JWTAuthClaim {
    pub fn new(expiration: u32, user_id: ObjectId, username: String) -> Self {
        Self {
            expiration,
            user_id,
            username,
        }
    }
    pub fn build_token(
        &self,
        header: &Header,
        encoding_key: &EncodingKey,
    ) -> Result<String, AuthError> {
        let token = encode(header, &self, encoding_key).map_err(|e| {
            error!("Got error {e}", e = e.to_string());
            AuthError::TokenCreation
        })?;
        Ok(token)
    }
}

impl<S> FromRequestParts<S> for JWTAuthClaim
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let state = Arc::from_ref(state);

        // Decode the user data
        let token_data = decode::<JWTAuthClaim>(
            bearer.token(),
            state.environment_service.get_authentication_jwt_decoding(),
            &Validation::default(),
        )
        .map_err(|e| {
            tracing::error!("Got error {}", e);
            AuthError::InvalidToken
        })?;

        Ok(token_data.claims)
    }
}

impl AuthInfo for JWTAuthClaim {
    fn user_id(&self) -> &ObjectId {
        &self.user_id
    }
}
