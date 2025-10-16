use crate::models::{self, SymptomId, UserId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::{ServerFnError, server};

#[cfg(feature = "server")]
use crate::models::MaybeSet;

#[cfg(feature = "server")]
use super::common::{AppError, get_database_connection, get_user_id};

#[server]
pub async fn get_symptoms_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Symptom>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::symptoms::get_symptoms_for_time_range(
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
pub async fn get_symptom_by_id(id: SymptomId) -> Result<Option<models::Symptom>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::symptoms::get_symptom_by_id(
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
pub async fn create_symptom(symptom: models::NewSymptom) -> Result<models::Symptom, ServerFnError> {
    use crate::server::database::models::symptoms;

    let logged_in_user_id = get_user_id().await?;

    if symptom.user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_symptom = symptoms::NewSymptom::from_front_end(&symptom);

    crate::server::database::models::symptoms::create_symptom(&mut conn, &new_symptom)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_symptom(
    id: SymptomId,
    symptom: models::ChangeSymptom,
) -> Result<models::Symptom, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let MaybeSet::Set(req_user_id) = symptom.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::symptoms::ChangeSymptom::from_front_end(&symptom);

    crate::server::database::models::symptoms::update_symptom(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_symptom(id: SymptomId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::symptoms::delete_symptom(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}
