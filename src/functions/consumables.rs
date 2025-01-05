use crate::models::{self, ConsumableId};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn search_consumables(query: String) -> Result<Vec<models::Consumable>, ServerFnError> {
    let mut conn = get_database_connection().await?;
    crate::server::database::models::consumables::search_consumables(&mut conn, &query)
        .await
        .map(|x| x.into_iter().map(|y| y.into()).collect())
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
    consumable: models::UpdateConsumable,
) -> Result<models::Consumable, ServerFnError> {
    let _logged_in_user_id = get_user_id().await?;

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::consumables::UpdateConsumable::from_front_end(&consumable);

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
