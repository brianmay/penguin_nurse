use axum::Extension;
use dioxus::prelude::*;

use crate::models::UserId;
use crate::server::auth::Session;
use crate::server::database::connection::DatabaseConnection;
use crate::server::database::connection::DatabasePool;

pub async fn get_database_connection() -> Result<DatabaseConnection, ServerFnError> {
    let Extension(pool): Extension<DatabasePool> = extract().await?;
    pool.get().await.map_err(ServerFnError::from)
}

pub async fn get_user_id() -> Result<UserId, ServerFnError> {
    let session: Session = extract().await?;

    session
        .user
        .as_ref()
        .map(|x| UserId::new(x.id))
        .ok_or(ServerFnError::ServerError::<String>(
            "Not Logged In".to_string(),
        ))
}

pub async fn assert_is_admin() -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let user = session
        .user
        .as_ref()
        .ok_or(ServerFnError::ServerError::<String>(
            "Not Logged In".to_string(),
        ))?;
    user.is_admin
        .then_some(())
        .ok_or(ServerFnError::ServerError::<String>(
            "Not Admin".to_string(),
        ))?;
    Ok(())
}
