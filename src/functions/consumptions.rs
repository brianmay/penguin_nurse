use dioxus::prelude::*;

use crate::models::{self, ConsumableId, ConsumptionId, UserId};

#[cfg(feature = "server")]
use crate::models::ConsumptionWithItems;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_consumptions_for_time_range(
    user_id: UserId,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<models::ConsumptionWithItems>, ServerFnError> {
    pub fn items_to_front_end(
        items: Vec<(
            crate::server::database::models::consumption_consumables::ConsumptionConsumable,
            crate::server::database::models::consumables::Consumable,
        )>,
    ) -> Vec<models::ConsumptionItem> {
        items
            .into_iter()
            .map(|(consumption_consumable, consumable)| {
                models::ConsumptionItem::new(
                    models::ConsumptionConsumable::from(consumption_consumable),
                    models::Consumable::from(consumable),
                )
            })
            .collect()
    }

    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::consumptions::get_consumptions_for_time_range(
        &mut conn,
        user_id.as_inner(),
        start,
        end,
    )
    .await
    .map(|x| {
        x.into_iter()
            .map(|(consumption, items)| {
                ConsumptionWithItems::new(consumption.into(), items_to_front_end(items))
            })
            .collect()
    })
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_child_consumables(
    parent_id: ConsumptionId,
) -> Result<Vec<models::ConsumptionItem>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::consumption_consumables::get_child_consumables(
        &mut conn,
        parent_id.as_inner(),
    )
    .await
    .map(|x| {
        x.into_iter()
            .map(|(a, b)| models::ConsumptionItem::new(a.into(), b.into()))
            .collect()
    })
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_parent_consumables(
    consumable_id: ConsumableId,
) -> Result<Vec<(models::ConsumptionConsumable, models::Consumable)>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::consumption_consumables::get_parent_consumables(
        &mut conn,
        consumable_id.as_inner(),
    )
    .await
    .map(|x| x.into_iter().map(|(a, b)| (a.into(), b.into())).collect())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_consumption_by_id(
    id: ConsumptionId,
) -> Result<Option<models::Consumption>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;

    crate::server::database::models::consumptions::get_consumption_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_consumption(
    consumption: models::NewConsumption,
) -> Result<models::Consumption, ServerFnError> {
    use crate::server::database::models::consumptions;

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let new_consumption = consumptions::NewConsumption::from_front_end(&consumption);

    crate::server::database::models::consumptions::create_consumption(&mut conn, &new_consumption)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_consumption(
    id: ConsumptionId,
    consumption: models::ChangeConsumption,
) -> Result<models::Consumption, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let updates = crate::server::database::models::consumptions::ChangeConsumption::from_front_end(
        &consumption,
    );

    crate::server::database::models::consumptions::update_consumption(
        &mut conn,
        id.as_inner(),
        &updates,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_consumption(id: ConsumptionId) -> Result<(), ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::consumptions::delete_consumption(&mut conn, id.as_inner())
        .await
        .map_err(ServerFnError::from)
}

#[server]
pub async fn create_consumption_consumable(
    consumable: models::NewConsumptionConsumable,
) -> Result<models::ConsumptionConsumable, ServerFnError> {
    use crate::server::database::models::consumption_consumables;

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let new_consumptions_consumable =
        consumption_consumables::NewConsumptionConsumable::from_front_end(&consumable);

    crate::server::database::models::consumption_consumables::create_consumption_consumable(
        &mut conn,
        &new_consumptions_consumable,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_consumption_consumable(
    id: models::ConsumptionConsumableId,
) -> Result<(), ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;
    let (parent_id, consumable_id) = id.as_inner();

    crate::server::database::models::consumption_consumables::delete_consumption_consumable(
        &mut conn,
        parent_id.as_inner(),
        consumable_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}

#[server]
pub async fn update_consumption_consumable(
    id: models::ConsumptionConsumableId,
    consumable: models::ChangeConsumptionConsumable,
) -> Result<models::ConsumptionConsumable, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::consumption_consumables::ChangeConsumptionConsumable::from_front_end(
            &consumable,
        );
    let (parent_id, consumable_id) = id.as_inner();

    crate::server::database::models::consumption_consumables::update_consumption_consumable(
        &mut conn,
        parent_id.as_inner(),
        consumable_id.as_inner(),
        &updates,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}
