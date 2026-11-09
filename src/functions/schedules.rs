use dioxus::prelude::*;
use dioxus_fullstack::{ServerFnError, server};

use crate::models::{self, ScheduleConsumableId, ScheduleId, UserId};

#[cfg(feature = "server")]
use crate::models::MaybeSet;

#[cfg(feature = "server")]
use super::common::{AppError, get_database_connection, get_user_id};

#[cfg(feature = "server")]
fn items_to_front_end(
    items: Vec<(
        crate::server::database::models::schedule_consumables::ScheduleConsumable,
        crate::server::database::models::consumables::Consumable,
    )>,
) -> Vec<models::ScheduleItem> {
    items
        .into_iter()
        .map(|(schedule_consumable, consumable)| {
            models::ScheduleItem::new(
                models::ScheduleConsumable::from(schedule_consumable),
                models::Consumable::from(consumable),
            )
        })
        .collect()
}

#[server]
pub async fn get_today_schedule_status(
    user_id: UserId,
    now: chrono::DateTime<chrono::FixedOffset>,
) -> Result<Vec<models::ScheduleStatus>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::schedules::get_today_schedule_status(
        &mut conn,
        user_id.as_inner(),
        now,
    )
    .await
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_schedules(
    user_id: UserId,
) -> Result<Vec<models::ScheduleWithItems>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::schedules::get_schedules(&mut conn, user_id.as_inner())
        .await
        .map(|x| {
            x.into_iter()
                .map(|(schedule, items)| {
                    models::ScheduleWithItems::new(schedule.into(), items_to_front_end(items))
                })
                .collect()
        })
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn get_schedule_by_id(id: ScheduleId) -> Result<Option<models::Schedule>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;

    crate::server::database::models::schedules::get_schedule_by_id(
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
pub async fn create_schedule(
    schedule: models::NewSchedule,
) -> Result<models::Schedule, ServerFnError> {
    use crate::server::database::models::schedules;

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let new_schedule = schedules::NewSchedule::from_front_end(&schedule);

    schedules::create_schedule(&mut conn, &new_schedule)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_schedule(
    id: ScheduleId,
    schedule: models::ChangeSchedule,
) -> Result<models::Schedule, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let MaybeSet::Set(req_user_id) = schedule.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::new(
            "User ID does not match the logged in user",
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::schedules::ChangeSchedule::from_front_end(&schedule);

    crate::server::database::models::schedules::update_schedule(&mut conn, id.as_inner(), &updates)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_schedule(id: ScheduleId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::schedules::delete_schedule(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_child_schedule_consumables(
    parent_id: ScheduleId,
) -> Result<Vec<models::ScheduleItem>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::schedule_consumables::get_child_consumables(
        &mut conn,
        parent_id.as_inner(),
    )
    .await
    .map(items_to_front_end)
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_schedule_consumable(
    consumable: models::NewScheduleConsumable,
) -> Result<models::ScheduleConsumable, ServerFnError> {
    use crate::server::database::models::schedule_consumables;

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let new_schedule_consumable =
        schedule_consumables::NewScheduleConsumable::from_front_end(&consumable);

    schedule_consumables::create_schedule_consumable(&mut conn, &new_schedule_consumable)
        .await
        .map(|x| x.into())
        .map_err(AppError::from)
        .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_schedule_consumable(id: ScheduleConsumableId) -> Result<(), ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;
    let (parent_id, consumable_id) = id.as_inner();

    crate::server::database::models::schedule_consumables::delete_schedule_consumable(
        &mut conn,
        parent_id.as_inner(),
        consumable_id.as_inner(),
    )
    .await
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}

#[server]
pub async fn update_schedule_consumable(
    id: ScheduleConsumableId,
    consumable: models::ChangeScheduleConsumable,
) -> Result<models::ScheduleConsumable, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::schedule_consumables::ChangeScheduleConsumable::from_front_end(
            &consumable,
        );
    let (parent_id, consumable_id) = id.as_inner();

    crate::server::database::models::schedule_consumables::update_schedule_consumable(
        &mut conn,
        parent_id.as_inner(),
        consumable_id.as_inner(),
        &updates,
    )
    .await
    .map(|x| x.into())
    .map_err(AppError::from)
    .map_err(ServerFnError::from)
}
