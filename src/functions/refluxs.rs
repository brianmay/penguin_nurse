use crate::models::{self, RefluxId, UserId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::models::MaybeSet;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_refluxs_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Reflux>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::refluxs::get_refluxs_for_time_range(
        &mut conn,
        user_id.as_inner(),
        start,
        end,
    )
    .await
    .map(|x| x.into_iter().map(|y| y.into()).collect())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_reflux_by_id(id: RefluxId) -> Result<Option<models::Reflux>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::refluxs::get_reflux_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_reflux(reflux: models::NewReflux) -> Result<models::Reflux, ServerFnError> {
    use crate::server::database::models::refluxs;

    let logged_in_user_id = get_user_id().await?;

    if reflux.user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_reflux = refluxs::NewReflux::from_front_end(&reflux);

    crate::server::database::models::refluxs::create_reflux(&mut conn, &new_reflux)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_reflux(
    id: RefluxId,
    reflux: models::ChangeReflux,
) -> Result<models::Reflux, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let MaybeSet::Set(req_user_id) = reflux.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates = crate::server::database::models::refluxs::ChangeReflux::from_front_end(&reflux);

    crate::server::database::models::refluxs::update_reflux(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_reflux(id: RefluxId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::refluxs::delete_reflux(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}
