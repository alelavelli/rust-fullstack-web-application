use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::State,
    routing::{get, post},
};
use tokio::sync::RwLock;

use crate::{
    AppResult, AppState,
    auth::JWTAuthClaim,
    dtos::{admin_request, admin_response},
    facade::admin::AdminFacade,
    service::database::transaction::MongoDBDatabaseTransaction,
    types::AppJson,
};

pub fn add_admin_router(
    base_path: &str,
    base_router: Router<Arc<AppState>>,
) -> Router<Arc<AppState>> {
    let router = Router::new()
        .route("/user", get(get_users))
        .route("/user", post(create_user));
    base_router.nest(base_path, router)
}

async fn get_users(
    State(state): State<Arc<AppState>>,
    jwt_claim: JWTAuthClaim,
) -> AppResult<Vec<admin_response::User>> {
    let facade = AdminFacade::new(jwt_claim, state.database_service.clone()).await?;

    facade.get_users().await.map(|values| {
        AppJson(
            values
                .into_iter()
                .map(|elem| elem.into())
                .collect::<Vec<admin_response::User>>(),
        )
    })
}

async fn create_user(
    State(state): State<Arc<AppState>>,
    Extension(transaction): Extension<Arc<RwLock<MongoDBDatabaseTransaction>>>,
    jwt_claim: JWTAuthClaim,
    Json(payload): Json<admin_request::CreateUser>,
) -> AppResult<String> {
    let facade = AdminFacade::new(jwt_claim, state.database_service.clone()).await?;

    facade
        .create_user(
            transaction,
            payload.first_name,
            payload.last_name,
            payload.username,
            payload.password,
            payload.admin,
        )
        .await
        .map(|value| AppJson(value.to_hex()))
}
