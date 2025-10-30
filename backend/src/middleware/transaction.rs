use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{Method, Request},
    middleware::Next,
    response::Response,
};
use tokio::sync::RwLock;
use tracing::debug;

use crate::{AppState, DatabaseServiceTrait};

/// Creates a mongodb transaction if the request is not a GET
/// and put it in the request extensions.
///
/// If the request is success then the transaction is committed
/// otherwise it is aborted
pub async fn mongodb_transaction_middleware<T: DatabaseServiceTrait + Clone + 'static>(
    State(app_state): State<AppState<T>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, axum::http::StatusCode>
where
{
    let method = request.method().clone();
    if matches!(
        method,
        Method::POST | Method::PATCH | Method::DELETE | Method::PUT
    ) {
        let db_service = app_state.database_service;
        let transaction = Arc::new(RwLock::new(
            db_service
                .new_transaction()
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?,
        ));

        request.extensions_mut().insert(Arc::clone(&transaction));

        let response = next.run(request).await;
        let mut guard = transaction.write().await;

        if response.status().is_success() {
            debug!(
                "Response status {status}, committing transaction",
                status = response.status()
            );
            let _ = guard
                .commit_transaction()
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        } else {
            debug!(
                "Response status {status}, aborting transaction",
                status = response.status()
            );

            let _ = guard
                .abort_transaction()
                .await
                .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
        }
        Ok(response)
    } else {
        let response = next.run(request).await;
        debug!("{}", format!("Got response {:?}", response));
        Ok(response)
    }
}
