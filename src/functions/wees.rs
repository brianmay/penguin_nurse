use crate::models::{self, UserId, WeeId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::{ServerFnError, server};

#[cfg(feature = "server")]
use crate::models::MaybeSet;

#[cfg(feature = "server")]
use super::common::{AppError, get_database_connection, get_user_id};

#[server]
pub async fn get_wees_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Wee>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::wees::get_wees_for_time_range(
        &mut conn,
        user_id.as_inner(),
        start,
        end,
    )
    .await
    .map(|x| x.into_iter().map(|y| y.into()).collect())
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_wee_by_id(id: WeeId) -> Result<Option<models::Wee>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::wees::get_wee_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_wee(wee: models::NewWee) -> Result<models::Wee, ServerFnError> {
    use crate::server::database::models::wees;

    let logged_in_user_id = get_user_id().await?;

    if wee.user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_wee = wees::NewWee::from_front_end(&wee);

    crate::server::database::models::wees::create_wee(&mut conn, &new_wee)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_wee(id: WeeId, wee: models::ChangeWee) -> Result<models::Wee, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let MaybeSet::Set(req_user_id) = wee.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates = crate::server::database::models::wees::ChangeWee::from_front_end(&wee);

    crate::server::database::models::wees::update_wee(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_wee(id: WeeId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::wees::delete_wee(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}
