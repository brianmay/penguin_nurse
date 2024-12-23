use axum::extract::ws;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use axum::{extract::WebSocketUpgrade, response::Response};
use tracing::{debug, error};

use crate::server::database::DatabasePool;

#[axum::debug_handler]
pub async fn dioxus_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|mut socket| async move { while let Some(Ok(_msg)) = socket.recv().await {} })
}

// health check
#[axum::debug_handler]
pub async fn health_check(Extension(pool): Extension<DatabasePool>) -> Response {
    let mut conn = pool.get().await.unwrap();
    // match crate::server::database::list_penguin_encounters(&mut conn).await {
    //     Ok(_) => (StatusCode::OK, "OK").into_response(),
    //     Err(e) => {
    //         error!("Error: {:?}", e);
    //         (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
    //     }
    // }
    (StatusCode::OK, "OK").into_response()
}
