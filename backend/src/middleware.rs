//! Middlewares are added to the application using
//! functions that takes a router, add the middleware
//! as new layer and then returns it

use axum::Router;
use tower_http::{
    LatencyUnit,
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

use crate::{DatabaseServiceTrait, types::AppState};

/// CORS Layer for the application
///
/// In this simple version, we allow everything,
/// for production code the scope can be narrowed
pub fn add_cors_middleware<T: DatabaseServiceTrait + Clone + 'static>(
    router: Router<AppState<T>>,
) -> Router<AppState<T>> {
    router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    )
}

/// Add logging layer to record the requests log
pub fn add_logging_middleware<T: DatabaseServiceTrait + Clone + 'static>(
    router: Router<AppState<T>>,
    include_headers: bool,
    logging_level: tracing::Level,
) -> Router<AppState<T>> {
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(include_headers))
            .on_request(DefaultOnRequest::new().level(logging_level))
            .on_response(
                DefaultOnResponse::new()
                    .level(logging_level)
                    .latency_unit(LatencyUnit::Micros),
            ),
    )
}
