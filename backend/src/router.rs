use axum::response::Html;
mod admin;
mod guest;
mod user;

pub use admin::add_admin_router;
pub use guest::add_guest_router;
pub use user::add_user_router;

pub async fn health_handler() -> Html<&'static str> {
    Html("Ok!")
}
