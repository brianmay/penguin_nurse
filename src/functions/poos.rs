use crate::models::{self, PooId, UserId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_poos_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Poo>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::poos::get_poos_for_time_range(
        &mut conn,
        logged_in_user_id.as_inner(),
        start,
        end,
    )
    .await
    .map(|x| x.into_iter().map(|y| y.into()).collect())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_poo_by_id(id: PooId) -> Result<Option<models::Poo>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::poos::get_poo_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_poo(poo: models::NewPoo) -> Result<models::Poo, ServerFnError> {
    use crate::server::database::models::poos;

    let logged_in_user_id = get_user_id().await?;

    if poo.user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_poo = poos::NewPoo::from_front_end(&poo);

    crate::server::database::models::poos::create_poo(&mut conn, new_poo)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_poo(id: PooId, poo: models::UpdatePoo) -> Result<models::Poo, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let Some(req_user_id) = poo.user_id {
        if logged_in_user_id != req_user_id {
            return Err(ServerFnError::ServerError(
                "User ID does not match the logged in user".to_string(),
            ));
        }
    }

    let mut conn = get_database_connection().await?;
    let updates = crate::server::database::models::poos::UpdatePoo::from_front_end(&poo);

    crate::server::database::models::poos::update_poo(&mut conn, id.as_inner(), updates)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_poo(id: PooId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::poos::delete_poo(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}
