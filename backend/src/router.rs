use axum::response::Html;

pub async fn health_handler() -> Html<&'static str> {
    Html("Ok!")
}
