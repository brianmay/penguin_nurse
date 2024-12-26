use dioxus::prelude::*;
use server_fn::error::NoCustomError;

use crate::models;

#[cfg(feature = "server")]
use crate::server::database::connection::DatabaseConnection;

#[cfg(feature = "server")]
pub async fn get_database_connection() -> Result<DatabaseConnection, ServerFnError> {
    use crate::server::database::connection::DatabasePool;
    use axum::Extension;

    let Extension(pool): Extension<DatabasePool> = extract().await?;
    pool.get().await.map_err(ServerFnError::from)
}

#[cfg(feature = "server")]
pub async fn assert_is_admin() -> Result<(), ServerFnError> {
    use crate::server::auth::Session;

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

#[server]
pub async fn get_users() -> Result<Vec<models::User>, ServerFnError> {
    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::users::get_users(&mut conn)
        .await
        .map(|x| x.into_iter().map(|y| y.into()).collect())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn get_user(id: i64) -> Result<Option<models::User>, ServerFnError> {
    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::users::get_user_by_id(&mut conn, id)
        .await
        .map(|x| x.map(|y| y.into()))
        .map_err(ServerFnError::<NoCustomError>::from)
}

#[server]
pub async fn create_user(user: models::NewUser) -> Result<models::User, ServerFnError> {
    use crate::server::database::models::users as server;

    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    let new_user: server::NewUser = server::NewUser::from_front_end(&user);

    crate::server::database::models::users::create_user(&mut conn, new_user)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::<NoCustomError>::from)
}

#[server]
pub async fn update_user(id: i64, user: models::UpdateUser) -> Result<models::User, ServerFnError> {
    use crate::server::database::models::users as server;

    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    let updates: server::UpdateUser = server::UpdateUser::from_front_end(&user);

    crate::server::database::models::users::update_user(&mut conn, id, updates)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::<NoCustomError>::from)
}

#[server]
pub async fn delete_user(id: i64) -> Result<(), ServerFnError> {
    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::users::delete_user(&mut conn, id)
        .await
        .map_err(ServerFnError::<NoCustomError>::from)
}
