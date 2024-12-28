use crate::models;
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_poos_for_time_range(
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::Poo>, ServerFnError> {
    let user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::poos::get_poos_for_time_range(&mut conn, user_id, start, end)
        .await
        .map(|x| x.into_iter().map(|y| y.into()).collect())
        .map_err(ServerFnError::from)
}
