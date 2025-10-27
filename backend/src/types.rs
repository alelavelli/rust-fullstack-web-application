use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequest, FromRequestParts},
    http::request::Parts,
    response::{IntoResponse, Response},
};
use tokio::task_local;

use crate::{DatabaseServiceTrait, EnvironmentServiceTrait, error::AppError};

/// JSON extractor wrapping `axum::Json`.
/// This makes it easy to override the rejection and provide our
/// own which formats errors to match our application.
///
/// `axum::Json` responds with plain text if the input is invalid.
#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(AppError))]
pub struct AppJson<T>(pub T);

impl<T> IntoResponse for AppJson<T>
where
    axum::Json<T>: IntoResponse,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

/// Application global variables that defines the common state
/// each request can access without creating new objects
///
/// According to documentation https://github.com/tokio-rs/axum/blob/main/examples/dependency-injection/src/main.rs
/// it is recommended to use dyn trait to leverage on dependency injection
#[derive(Clone)]
pub struct AppState {
    pub environment_service: Arc<dyn EnvironmentServiceTrait>,
    pub database_service: Arc<dyn DatabaseServiceTrait>,
}

impl AppState {
    pub fn new(
        environment_service: Arc<dyn EnvironmentServiceTrait>,
        database_service: Arc<dyn DatabaseServiceTrait>,
    ) -> AppState {
        AppState {
            environment_service,
            database_service,
        }
    }
}

// Implement FromRequestParts trait to use the state in the application routers
impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}

// Declare task local variable so that each request can access the AppState without
// passing to to each method call
task_local! {
    static APP_STATE: AppState;
}
