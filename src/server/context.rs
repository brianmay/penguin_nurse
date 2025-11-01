use axum::body::Body;
use axum::{
    http::{Request, Response},
    middleware::Next,
};
use dioxus_fullstack::ServerFnError;
use std::sync::Arc;
use thiserror::Error;
use tokio::task_local;

task_local! {
    static REQ_EXTENSIONS: Arc<axum::http::Extensions>;
}

/// Middleware version of `with_request_context`
pub async fn with_request_context(req: Request<Body>, next: Next) -> Response<Body> {
    // Extract parts (headers, extensions, etc.)
    let parts_arc = Arc::new(req.extensions().clone()); // or store extensions if needed
    // let (parts, body) = req.into_parts(); // extract full Parts
    // let parts_arc = Arc::new(parts);

    // Store in task-local, run next without changing the request
    REQ_EXTENSIONS
        .scope(parts_arc.clone(), async move { next.run(req).await })
        .await
}

#[derive(Debug, Error)]
#[error("Requested extension not found")]
pub struct ExtractError;

impl From<ExtractError> for ServerFnError {
    fn from(err: ExtractError) -> Self {
        ServerFnError::new(err.to_string())
    }
}

/// Helper similar to old `extract::<T>()`
pub fn extract<T>() -> Result<T, ExtractError>
where
    T: Clone + Send + Sync + 'static,
{
    REQ_EXTENSIONS
        .try_with(|parts| {
            // Try to extract Extension<T> from request extensions
            parts.get::<T>().cloned().ok_or(ExtractError)
        })
        .map_err(|_| ExtractError)?
}
