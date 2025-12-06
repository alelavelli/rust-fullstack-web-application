use std::sync::Arc;

use crate::{
    AppResult, AppState,
    dtos::{guest_request, guest_response},
    facade::guest::GuestFacade,
    service::database::transaction::MongoDBDatabaseTransaction,
};
use axum::{Extension, Json, Router, extract::State, routing::post};
use tokio::sync::RwLock;

pub fn add_guest_router(
    base_path: &str,
    base_router: Router<Arc<AppState>>,
) -> Router<Arc<AppState>> {
    let router = Router::new()
        .route("/login", post(login))
        .route("/register", post(register));
    base_router.nest(base_path, router)
}

/// Receives first_name, last_name, username and password, creates the user and then
/// generate JWT for the session
async fn register(
    State(state): State<Arc<AppState>>,
    Extension(transaction): Extension<Arc<RwLock<MongoDBDatabaseTransaction>>>,
    Json(payload): Json<guest_request::RegisterInfo>,
) -> AppResult<guest_response::LoggedUserInfoResponse> {
    let database_service = state.database_service.clone();
    GuestFacade::new(state)
        .register_user(
            database_service,
            transaction,
            payload.first_name,
            payload.last_name,
            payload.username,
            payload.password,
        )
        .await
}

/// Receives username and password, validates the user and
/// generate JWT for the session
async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<guest_request::JWTAuthPayload>,
) -> AppResult<guest_response::LoggedUserInfoResponse> {
    GuestFacade::new(state)
        .authenticate_user(&payload.username, &payload.password)
        .await
}
