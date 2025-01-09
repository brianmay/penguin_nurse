use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

use super::consumables::Consumable;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumption_consumables)]
pub struct ConsumptionConsumable {
    pub parent_id: i64,
    pub consumable_id: i64,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ConsumptionConsumable> for crate::models::ConsumptionConsumable {
    fn from(consumption_consumable: ConsumptionConsumable) -> Self {
        Self {
            id: models::ConsumptionConsumableId::new(
                models::ConsumptionId::new(consumption_consumable.parent_id),
                models::ConsumableId::new(consumption_consumable.consumable_id),
            ),
            quantity: consumption_consumable.quantity.into(),
            liquid_mls: consumption_consumable.liquid_mls.into(),
            comments: consumption_consumable.comments.into(),
            created_at: consumption_consumable.created_at,
            updated_at: consumption_consumable.updated_at,
        }
    }
}

pub async fn get_child_consumables(
    conn: &mut DatabaseConnection,
    parent_id: i64,
) -> Result<Vec<(ConsumptionConsumable, Consumable)>, diesel::result::Error> {
    use schema::consumption_consumables::dsl as q;
    use schema::consumption_consumables::table;

    let consumption_consumables = table
        .filter(q::parent_id.eq(parent_id))
        .inner_join(schema::consumables::table.on(schema::consumables::id.eq(q::consumable_id)))
        .get_results::<(ConsumptionConsumable, Consumable)>(conn)
        .await?;

    Ok(consumption_consumables)
}

pub async fn get_parent_consumables(
    conn: &mut DatabaseConnection,
    consumable_id: i64,
) -> Result<Vec<(ConsumptionConsumable, Consumable)>, diesel::result::Error> {
    use schema::consumption_consumables::dsl as q;
    use schema::consumption_consumables::table;

    let consumption_consumables = table
        .filter(q::consumable_id.eq(consumable_id))
        .inner_join(schema::consumables::table.on(schema::consumables::id.eq(q::parent_id)))
        .get_results::<(ConsumptionConsumable, Consumable)>(conn)
        .await?;

    Ok(consumption_consumables)
}

// pub async fn get_consumption_consumable_by_id(
//     conn: &mut DatabaseConnection,
//     parent_id: i64,
//     consumable_id: i64,
// ) -> Result<Option<ConsumptionConsumable>, diesel::result::Error> {
//     use schema::consumption_consumables::consumable_id as q_consumable_id;
//     use schema::consumption_consumables::parent_id as q_parent_id;
//     use schema::consumption_consumables::table;

//     table
//         .filter(q_parent_id.eq(parent_id))
//         .filter(q_consumable_id.eq(consumable_id))
//         .get_result(conn)
//         .await
//         .optional()
// }

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumption_consumables)]
pub struct NewConsumptionConsumable<'a> {
    pub parent_id: i64,
    pub consumable_id: i64,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<&'a str>,
}

impl<'a> NewConsumptionConsumable<'a> {
    pub fn from_front_end(
        consumption_consumable: &'a crate::models::NewConsumptionConsumable,
    ) -> Self {
        let (parent_id, consumable_id) = consumption_consumable.id.as_inner();

        Self {
            parent_id: parent_id.as_inner(),
            consumable_id: consumable_id.as_inner(),
            quantity: consumption_consumable.quantity.into(),
            liquid_mls: consumption_consumable.liquid_mls.into(),
            comments: consumption_consumable.comments.as_deref(),
        }
    }
}

pub async fn create_consumption_consumable(
    conn: &mut DatabaseConnection,
    update: &NewConsumptionConsumable<'_>,
) -> Result<ConsumptionConsumable, diesel::result::Error> {
    diesel::insert_into(schema::consumption_consumables::table)
        .values(update)
        .returning(ConsumptionConsumable::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumption_consumables)]
pub struct UpdateConsumptionConsumable<'a> {
    pub quantity: Option<Option<f64>>,
    pub liquid_mls: Option<Option<f64>>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> UpdateConsumptionConsumable<'a> {
    pub fn from_front_end(
        consumption_consumable: &'a crate::models::UpdateConsumptionConsumable,
    ) -> Self {
        Self {
            quantity: consumption_consumable.quantity.map(|x| x.into()),
            liquid_mls: consumption_consumable.liquid_mls.map(|x| x.into()),
            comments: consumption_consumable
                .comments
                .as_ref()
                .map(|x| x.as_deref()),
        }
    }
}

pub async fn update_consumption_consumable(
    conn: &mut DatabaseConnection,
    parent_id: i64,
    consumable_id: i64,
    update: &UpdateConsumptionConsumable<'_>,
) -> Result<ConsumptionConsumable, diesel::result::Error> {
    diesel::update(
        schema::consumption_consumables::table
            .filter(schema::consumption_consumables::parent_id.eq(parent_id))
            .filter(schema::consumption_consumables::consumable_id.eq(consumable_id)),
    )
    .set(update)
    .returning(ConsumptionConsumable::as_returning())
    .get_result(conn)
    .await
}

pub async fn delete_consumption_consumable(
    conn: &mut DatabaseConnection,
    parent_id: i64,
    consumable_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::consumption_consumables::consumable_id as q_consumable_id;
    use schema::consumption_consumables::parent_id as q_parent_id;
    use schema::consumption_consumables::table;

    diesel::delete(
        table
            .filter(q_parent_id.eq(parent_id))
            .filter(q_consumable_id.eq(consumable_id)),
    )
    .execute(conn)
    .await?;
    Ok(())
}
