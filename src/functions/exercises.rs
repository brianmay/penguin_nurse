use crate::models::{self, ExerciseId, UserId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use dioxus_fullstack::{ServerFnError, server};

#[cfg(feature = "server")]
use crate::models::MaybeSet;

#[cfg(feature = "server")]
use super::common::{AppError, get_database_connection, get_user_id};

#[server]
pub async fn get_exercises_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Exercise>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::exercises::get_exercises_for_time_range(
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
pub async fn get_exercise_by_id(id: ExerciseId) -> Result<Option<models::Exercise>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::exercises::get_exercise_by_id(
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
pub async fn create_exercise(
    exercise: models::NewExercise,
) -> Result<models::Exercise, ServerFnError> {
    use crate::server::database::models::exercises;

    let logged_in_user_id = get_user_id().await?;

    if exercise.user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_exercise = exercises::NewExercise::from_front_end(&exercise);

    crate::server::database::models::exercises::create_exercise(&mut conn, &new_exercise)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_exercise(
    id: ExerciseId,
    exercise: models::ChangeExercise,
) -> Result<models::Exercise, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let MaybeSet::Set(req_user_id) = exercise.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::exercises::ChangeExercise::from_front_end(&exercise);

    crate::server::database::models::exercises::update_exercise(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_exercise(id: ExerciseId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::exercises::delete_exercise(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}
