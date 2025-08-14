use crate::models::{self, ConsumableId};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn search_consumables_with_nested(
    query: String,
    include_only_created: bool,
    include_destroyed: bool,
) -> Result<Vec<models::ConsumableWithItems>, ServerFnError> {
    pub fn items_to_front_end(
        items: Vec<(
            crate::server::database::models::nested_consumables::NestedConsumable,
            crate::server::database::models::consumables::Consumable,
        )>,
    ) -> Vec<models::ConsumableItem> {
        items
            .into_iter()
            .map(|(consumption_consumable, consumable)| {
                models::ConsumableItem::new(
                    models::NestedConsumable::from(consumption_consumable),
                    models::Consumable::from(consumable),
                )
            })
            .collect()
    }

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::consumables::search_consumables_with_nested(
        &mut conn,
        &query,
        include_only_created,
        include_destroyed,
    )
    .await
    // .map(|x| x.into_iter().map(|y| y.into()).collect())
    .map(|x| {
        x.into_iter()
            .map(|(consumption, items)| {
                models::ConsumableWithItems::new(consumption.into(), items_to_front_end(items))
            })
            .collect()
    })
    .map_err(ServerFnError::from)
}

#[server]
pub async fn search_consumables(
    query: String,
    include_only_created: bool,
    include_destroyed: bool,
) -> Result<Vec<models::Consumable>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::consumables::search_consumables(
        &mut conn,
        &query,
        include_only_created,
        include_destroyed,
    )
    .await
    .map(|x| x.into_iter().map(|y| y.into()).collect())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_child_consumables(
    parent_id: ConsumableId,
) -> Result<Vec<models::ConsumableItem>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::nested_consumables::get_child_consumables(
        &mut conn,
        parent_id.as_inner(),
    )
    .await
    .map(|x| {
        x.into_iter()
            .map(|(a, b)| models::ConsumableItem::new(a.into(), b.into()))
            .collect()
    })
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_parent_consumables(
    consumable_id: ConsumableId,
) -> Result<Vec<(models::NestedConsumable, models::Consumable)>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    crate::server::database::models::nested_consumables::get_parent_consumables(
        &mut conn,
        consumable_id.as_inner(),
    )
    .await
    .map(|x| x.into_iter().map(|(a, b)| (a.into(), b.into())).collect())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_consumable_by_id(
    id: ConsumableId,
) -> Result<Option<models::Consumable>, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;

    crate::server::database::models::consumables::get_consumable_by_id(&mut conn, id.as_inner())
        .await
        .map(|x| x.map(|y| y.into()))
        .map_err(ServerFnError::from)
}

#[server]
pub async fn create_consumable(
    consumable: models::NewConsumable,
) -> Result<models::Consumable, ServerFnError> {
    use crate::server::database::models::consumables;

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let new_consumable = consumables::NewConsumable::from_front_end(&consumable);

    crate::server::database::models::consumables::create_consumable(&mut conn, &new_consumable)
        .await
        .map(|x| x.into())
        .map_err(ServerFnError::from)
}

#[server]
pub async fn update_consumable(
    id: ConsumableId,
    consumable: models::ChangeConsumable,
) -> Result<models::Consumable, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::consumables::ChangeConsumable::from_front_end(&consumable);

    crate::server::database::models::consumables::update_consumable(
        &mut conn,
        id.as_inner(),
        &updates,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_consumable(id: ConsumableId) -> Result<(), ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::consumables::delete_consumable(&mut conn, id.as_inner())
        .await
        .map_err(ServerFnError::from)
}

#[server]
pub async fn create_nested_consumable(
    consumable: models::NewNestedConsumable,
) -> Result<models::NestedConsumable, ServerFnError> {
    use crate::server::database::models::nested_consumables;

    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let new_nested_consumable =
        nested_consumables::NewNestedConsumable::from_front_end(&consumable);

    crate::server::database::models::nested_consumables::create_nested_consumable(
        &mut conn,
        &new_nested_consumable,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_nested_consumable(id: models::NestedConsumableId) -> Result<(), ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;
    let (parent_id, consumable_id) = id.as_inner();

    crate::server::database::models::nested_consumables::delete_nested_consumable(
        &mut conn,
        parent_id.as_inner(),
        consumable_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}

#[server]
pub async fn update_nested_consumable(
    id: models::NestedConsumableId,
    consumable: models::ChangeNestedConsumable,
) -> Result<models::NestedConsumable, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::nested_consumables::ChangeNestedConsumable::from_front_end(
            &consumable,
        );
    let (parent_id, consumable_id) = id.as_inner();

    crate::server::database::models::nested_consumables::update_nested_consumable(
        &mut conn,
        parent_id.as_inner(),
        consumable_id.as_inner(),
        &updates,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}
