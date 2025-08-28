use crate::models::{self, UserId, WeeUrgeId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_wee_urges_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::WeeUrge>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::wee_urges::get_wee_urges_for_time_range(
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
pub async fn get_wee_urge_by_id(id: WeeUrgeId) -> Result<Option<models::WeeUrge>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::wee_urges::get_wee_urge_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_wee_urge(
    wee_urge: models::NewWeeUrge,
) -> Result<models::WeeUrge, ServerFnError> {
    use crate::server::database::models::wee_urges;

    let logged_in_user_id = get_user_id().await?;

    if wee_urge.user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_wee_urge = wee_urges::NewWeeUrge::from_front_end(&wee_urge);

    crate::server::database::models::wee_urges::create_wee_urge(&mut conn, &new_wee_urge)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_wee_urge(
    id: WeeUrgeId,
    wee_urge: models::ChangeWeeUrge,
) -> Result<models::WeeUrge, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let Some(req_user_id) = wee_urge.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::wee_urges::ChangeWeeUrge::from_front_end(&wee_urge);

    crate::server::database::models::wee_urges::update_wee_urge(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_wee_urge(id: WeeUrgeId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::wee_urges::delete_wee_urge(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}
