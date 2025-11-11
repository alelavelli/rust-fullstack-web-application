use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    routing::{get, post},
};

use bson::oid::ObjectId;
use tokio::sync::RwLock;

use crate::{
    AppResult, AppState,
    auth::JWTAuthClaim,
    dtos::{user_request, user_response::BlogPost},
    facade::user::UserFacade,
    service::database::transaction::MongoDBDatabaseTransaction,
    types::AppJson,
};

pub fn add_user_router(
    base_path: &str,
    base_router: Router<Arc<AppState>>,
) -> Router<Arc<AppState>> {
    let router = Router::new()
        .route("/blog/post", post(publish_post))
        .route("/blog/post", get(get_posts))
        .route("/blog/post/user/{id}", get(get_user_posts));
    base_router.nest(base_path, router)
}

async fn publish_post(
    State(state): State<Arc<AppState>>,
    Extension(transaction): Extension<Arc<RwLock<MongoDBDatabaseTransaction>>>,
    jwt_claim: JWTAuthClaim,
    Json(payload): Json<user_request::PublishPost>,
) -> AppResult<String> {
    UserFacade::new(jwt_claim, state.database_service.clone())
        .await?
        .publish_post(transaction, payload.title, payload.content)
        .await
        .map(AppJson)
}

async fn get_posts(
    State(state): State<Arc<AppState>>,
    jwt_claim: JWTAuthClaim,
) -> AppResult<Vec<BlogPost>> {
    UserFacade::new(jwt_claim, state.database_service.clone())
        .await?
        .get_posts(None)
        .await
        .map(|values| {
            AppJson(
                values
                    .into_iter()
                    .map(|elem| elem.into())
                    .collect::<Vec<BlogPost>>(),
            )
        })
}

async fn get_user_posts(
    State(state): State<Arc<AppState>>,
    Path(id): Path<ObjectId>,
    jwt_claim: JWTAuthClaim,
) -> AppResult<Vec<BlogPost>> {
    UserFacade::new(jwt_claim, state.database_service.clone())
        .await?
        .get_posts(Some(id))
        .await
        .map(|values| {
            AppJson(
                values
                    .into_iter()
                    .map(|elem| elem.into())
                    .collect::<Vec<BlogPost>>(),
            )
        })
}
