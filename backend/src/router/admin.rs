use std::sync::Arc;

use axum::Router;

use crate::{AppState, service::database::DatabaseServiceTrait};

pub fn add_admin_router<T: DatabaseServiceTrait + 'static>(
    base_path: &str,
    base_router: Router<Arc<AppState<T>>>,
) -> Router<Arc<AppState<T>>> {
    let router = Router::new();
    base_router.nest(base_path, router)
}
