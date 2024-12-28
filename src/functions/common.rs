use axum::Extension;
use dioxus::prelude::*;
use server_fn::error::NoCustomError;

use crate::server::auth::Session;
use crate::server::database::connection::DatabaseConnection;
use crate::server::database::connection::DatabasePool;

pub async fn get_database_connection() -> Result<DatabaseConnection, ServerFnError> {
    let Extension(pool): Extension<DatabasePool> = extract().await?;
    pool.get().await.map_err(ServerFnError::from)
}

pub async fn get_user_id() -> Result<i64, ServerFnError> {
    let session: Session = extract().await?;

    session
        .user
        .as_ref()
        .map(|x| x.id)
        .ok_or(ServerFnError::ServerError::<NoCustomError>(
            "Not Logged In".to_string(),
        ))
}

pub async fn assert_is_admin() -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let user = session
        .user
        .as_ref()
        .ok_or(ServerFnError::ServerError::<NoCustomError>(
            "Not Logged In".to_string(),
        ))?;
    user.is_admin
        .then_some(())
        .ok_or(ServerFnError::ServerError::<NoCustomError>(
            "Not Admin".to_string(),
        ))?;
    Ok(())
}
