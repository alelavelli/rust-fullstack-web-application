use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};

use crate::{
    AppResult, AppState,
    dtos::{guest_request, guest_response},
    facade::guest::GuestFacade,
    service::database::DatabaseServiceTrait,
};

pub fn add_guest_router<T: DatabaseServiceTrait + 'static>(
    base_path: &str,
    base_router: Router<Arc<AppState<T>>>,
) -> Router<Arc<AppState<T>>> {
    let router = Router::new().route("/login", post(login));
    base_router.nest(base_path, router)
}

/// Receives username and password, validates the user and
/// generate JWT for the session
async fn login<T: DatabaseServiceTrait>(
    State(state): State<Arc<AppState<T>>>,
    Json(payload): Json<guest_request::JWTAuthPayload>,
) -> AppResult<guest_response::JWTAuthResponse> {
    GuestFacade::new(state)
        .authenticate_user(&payload.username, &payload.password)
        .await
}
