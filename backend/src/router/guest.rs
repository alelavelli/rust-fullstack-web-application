use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::post};

use crate::{
    AppResult, AppState,
    dtos::{guest_request, guest_response},
    facade::guest::GuestFacade,
};

pub fn add_guest_router(
    base_path: &str,
    base_router: Router<Arc<AppState>>,
) -> Router<Arc<AppState>> {
    let router = Router::new().route("/login", post(login));
    base_router.nest(base_path, router)
}

/// Receives username and password, validates the user and
/// generate JWT for the session
async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<guest_request::JWTAuthPayload>,
) -> AppResult<guest_response::JWTAuthResponse> {
    GuestFacade::new(state)
        .authenticate_user(&payload.username, &payload.password)
        .await
}
