use crate::models::{self, MaybeSet, NoteId, UserId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_notes_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Note>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::notes::get_notes_for_time_range(
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
pub async fn get_note_by_id(id: NoteId) -> Result<Option<models::Note>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::notes::get_note_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_note(note: models::NewNote) -> Result<models::Note, ServerFnError> {
    use crate::server::database::models::notes;

    let logged_in_user_id = get_user_id().await?;

    if note.user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_note = notes::NewNote::from_front_end(&note);

    crate::server::database::models::notes::create_note(&mut conn, &new_note)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_note(
    id: NoteId,
    note: models::ChangeNote,
) -> Result<models::Note, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let MaybeSet::Set(req_user_id) = note.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates = crate::server::database::models::notes::ChangeNote::from_front_end(&note);

    crate::server::database::models::notes::update_note(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_note(id: NoteId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::notes::delete_note(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}
