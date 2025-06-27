use dioxus::prelude::*;

#[cfg(feature = "server")]
use server_fn::error::NoCustomError;

use crate::models::{self, UserId};

#[cfg(feature = "server")]
use super::common::{assert_is_admin, get_database_connection};

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
pub async fn get_user_by_id(id: UserId) -> Result<Option<models::User>, ServerFnError> {
    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::users::get_user_by_id(&mut conn, id.as_inner())
        .await
        .map(|x| x.map(|y| y.into()))
        .map_err(ServerFnError::<NoCustomError>::from)
}

#[server]
pub async fn create_user(user: models::NewUser) -> Result<models::User, ServerFnError> {
    use crate::server::database::models::users as server;

    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    let hashed_password = password_auth::generate_hash(&user.password);
    let new_user: server::NewUser = server::NewUser::from_front_end(&user, &hashed_password);

    crate::server::database::models::users::create_user(&mut conn, new_user)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::<NoCustomError>::from)
}

#[server]
pub async fn update_user(
    id: UserId,
    user: models::UpdateUser,
) -> Result<models::User, ServerFnError> {
    use crate::server::database::models::users as server;

    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    let hashed_password = user.password.as_ref().map(password_auth::generate_hash);

    let updates: server::UpdateUser =
        server::UpdateUser::from_front_end(&user, hashed_password.as_deref());

    crate::server::database::models::users::update_user(&mut conn, id.as_inner(), updates)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::<NoCustomError>::from)
}

#[server]
pub async fn delete_user(id: UserId) -> Result<(), ServerFnError> {
    assert_is_admin().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::users::delete_user(&mut conn, id.as_inner())
        .await
        .map_err(ServerFnError::<NoCustomError>::from)
}
