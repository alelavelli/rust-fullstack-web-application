use std::path::Path;

use axum::{
    Router,
    response::Html,
    routing::{get, get_service},
};
use frontend_server::{environment::EnvironmentService, middleware};
use tower_http::services::{ServeDir, ServeFile};
use tracing::info;

#[tokio::main]
async fn main() {
    let environment_service = EnvironmentService::new();

    // initialize tracing logging with level defined by the environment service
    tracing_subscriber::fmt()
        .with_max_level(environment_service.get_logging_level())
        .with_ansi(true)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .expect("Error in binding TcpListener");
    info!("Starting listener on port 3001");
    axum::serve(listener, build_app(&environment_service))
        .await
        .unwrap();
}

fn build_app(environment_service: &EnvironmentService) -> Router {
    let mut app = Router::new()
        .route(
            "/",
            get_service(ServeFile::new(
                Path::new(environment_service.get_files_path()).join("index.html"),
            )),
        )
        .fallback_service(get_service(
            ServeDir::new(environment_service.get_files_path()).not_found_service(ServeFile::new(
                Path::new(environment_service.get_files_path()).join("index.html"),
            )),
        ))
        .route("/api/health", get(health_handler));

    // Add middlewares to our application.
    // Layers are accessed from bottom to up, hence the order is very important
    app = middleware::add_logging_middleware(
        app,
        environment_service.get_logging_include_headers(),
        environment_service.get_logging_level(),
    );
    app = middleware::add_cors_middleware(app);
    app
}

pub async fn health_handler() -> Html<&'static str> {
    Html("Ok!")
}
