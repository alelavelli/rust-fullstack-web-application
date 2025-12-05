//! This modules defines functions to add middlewares to the
//! application router.
//!
//! Middlewares are added to the application using
//! functions that takes a router, add the middleware
//! as new layer and then returns it

use axum::Router;
use tower_http::{
    LatencyUnit,
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};

/// CORS Layer for the application
///
/// In this simple version, we allow everything,
/// for production code the scope can be narrowed
pub fn add_cors_middleware(router: Router) -> Router {
    router.layer(
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any),
    )
}

/// Add logging layer to record the requests log
///
/// When the argument `include_headers` is `true` then
/// each span is instrumented with headers as parameters.
pub fn add_logging_middleware(
    router: Router,
    include_headers: bool,
    logging_level: tracing::Level,
) -> Router {
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
