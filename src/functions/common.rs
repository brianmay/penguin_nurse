use axum::http;
use axum::response::IntoResponse;
use axum::response::Response;
use diesel_async::pooled_connection::PoolError;
use dioxus::prelude::*;
use dioxus_fullstack::FullstackContext;
use dioxus_fullstack::ServerFnError;
use tap::Pipe;
use thiserror::Error;

use crate::models::UserId;
use crate::server::auth::Session;
use crate::server::database::connection::DatabaseConnection;
use crate::server::database::connection::DatabasePool;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database pool error: {0}")]
    DbPoolError(#[from] mobc::Error<PoolError>),

    #[error("Database error: {0}")]
    DbError(#[from] diesel::result::Error),
}

impl From<AppError> for ServerFnError {
    fn from(err: AppError) -> Self {
        ServerFnError::new(err.to_string())
    }
}

#[derive(Debug, Error)]
#[error("Database connection not found")]
pub struct DatabaseConnectionNotFound;

impl IntoResponse for DatabaseConnectionNotFound {
    fn into_response(self) -> Response {
        (
            http::status::StatusCode::INTERNAL_SERVER_ERROR,
            "DatabaseConnection was not found",
        )
            .into_response()
    }
}

impl<S: std::marker::Sync + std::marker::Send> axum::extract::FromRequestParts<S> for DatabasePool {
    type Rejection = DatabaseConnectionNotFound;

    async fn from_request_parts(
        parts: &mut http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<DatabasePool>()
            .cloned()
            .ok_or(DatabaseConnectionNotFound)
    }
}

pub async fn get_database_connection() -> Result<DatabaseConnection, ServerFnError> {
    let pool: DatabasePool = FullstackContext::extract().await?;
    pool.get().await.map_err(AppError::from)?.pipe(Ok)
}

pub async fn get_user_id() -> Result<UserId, ServerFnError> {
    let session: Session = FullstackContext::extract().await?;
    session
        .user
        .as_ref()
        .map(|x| UserId::new(x.id))
        .ok_or(ServerFnError::new("Not Logged In".to_string()))
}

pub async fn assert_is_admin() -> Result<(), ServerFnError> {
    let session: Session = FullstackContext::extract().await?;
    let user = session
        .user
        .as_ref()
        .ok_or(ServerFnError::new("Not Logged In".to_string()))?;
    user.is_admin
        .then_some(())
        .ok_or(ServerFnError::new("Not Admin".to_string()))?;
    Ok(())
}
