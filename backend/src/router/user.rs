use std::sync::Arc;

use axum::Router;

use crate::AppState;

pub fn add_user_router(
    base_path: &str,
    base_router: Router<Arc<AppState>>,
) -> Router<Arc<AppState>> {
    let router = Router::new();
    base_router.nest(base_path, router)
}
