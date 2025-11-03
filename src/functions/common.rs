use axum::Extension;
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

pub async fn get_database_connection() -> Result<DatabaseConnection, ServerFnError> {
    let Extension(pool): Extension<DatabasePool> = FullstackContext::extract().await?;
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
