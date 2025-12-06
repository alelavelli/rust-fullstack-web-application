use std::{path::Path, sync::Arc};

use axum::{
    Router,
    routing::{get, get_service},
};
use backend::{
    AppState, EnvironmentService, EnvironmentServiceTrait, FrontendMode, middleware, router,
    service::database::{DatabaseServiceTrait, MongoDBDatabaseService},
};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{error, info};

/// Start the application server listening for requests
///
/// The initialization steps are:
///
/// - build environment service
/// - build app state
/// - setup logging
/// - build app
/// - serve app
#[tokio::main]
async fn main() {
    let environment_service = EnvironmentService::default();
    let mut database_service = MongoDBDatabaseService::new(
        environment_service.get_database_db_name().into(),
        environment_service.get_database_connection_string().into(),
    );
    let connection_result = database_service.connect().await;
    if let Err(connection_error) = connection_result {
        error!(
            "Error in connecting to database: {err}",
            err = connection_error
        );
        return;
    }

    let app_state = Arc::new(AppState::new(
        Box::new(environment_service),
        Arc::new(database_service),
    ));

    // initialize tracing logging with level defined by the environment service
    tracing_subscriber::fmt()
        .with_max_level(app_state.environment_service.get_logging_level())
        .with_ansi(true)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Error in binding TcpListener");
    info!("Starting listener on port 3000");
    axum::serve(listener, build_app(app_state)).await.unwrap();
}

/// Build our application routes. According to frontend mode we change the root behavior.
/// When frontend mode is integrated, the root returns index.html and the other static content
/// via fallback service.
///
/// When frontend mode is external then the root returns standard 200 OK
fn build_app(state: Arc<AppState>) -> Router {
    let mut app =
        if let FrontendMode::Integrated(path) = state.environment_service.get_frontend_mode() {
            tracing::info!("working with frontend mode `integrated` with path {path}");
            Router::new()
                .route(
                    "/",
                    get_service(ServeFile::new(Path::new(path).join("index.html"))),
                )
                .fallback_service(get_service(
                    ServeDir::new(path)
                        .not_found_service(ServeFile::new(Path::new(path).join("index.html"))),
                ))
                .route("/api/health", get(router::health_handler))
        } else {
            Router::new()
                // `GET /` goes to `root`
                .route("/", get(router::health_handler))
        };

    app = router::add_guest_router("/api/guest", app);
    app = router::add_admin_router("/api/admin", app);
    app = router::add_user_router("/api/user", app);

    // Add middlewares to our application.
    // Layers are accessed from bottom to up, hence the order is very important
    app = middleware::add_mongodb_transaction_middleware(Arc::clone(&state), app);
    app = middleware::add_logging_middleware(
        app,
        state.environment_service.get_logging_include_headers(),
        state.environment_service.get_logging_level(),
    );
    app = middleware::add_cors_middleware(app);

    // Attach the state of the application that will be used by its services
    app.with_state(state)
}
