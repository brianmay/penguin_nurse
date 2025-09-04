use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

use super::consumables::Consumable;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::nested_consumables)]
pub struct NestedConsumable {
    pub parent_id: i64,
    pub consumable_id: i64,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<NestedConsumable> for crate::models::NestedConsumable {
    fn from(nested_consumable: NestedConsumable) -> Self {
        Self {
            id: models::NestedConsumableId::new(
                models::ConsumableId::new(nested_consumable.parent_id),
                models::ConsumableId::new(nested_consumable.consumable_id),
            ),
            quantity: nested_consumable.quantity,
            liquid_mls: nested_consumable.liquid_mls,
            comments: nested_consumable.comments,
            created_at: nested_consumable.created_at,
            updated_at: nested_consumable.updated_at,
        }
    }
}

pub async fn get_child_consumables(
    conn: &mut DatabaseConnection,
    parent_id: i64,
) -> Result<Vec<(NestedConsumable, Consumable)>, diesel::result::Error> {
    use schema::nested_consumables::dsl as q;
    use schema::nested_consumables::table;

    let nested_consumables = table
        .filter(q::parent_id.eq(parent_id))
        .inner_join(schema::consumables::table.on(schema::consumables::id.eq(q::consumable_id)))
        .get_results::<(NestedConsumable, Consumable)>(conn)
        .await?;

    Ok(nested_consumables)
}

pub async fn get_parent_consumables(
    conn: &mut DatabaseConnection,
    consumable_id: i64,
) -> Result<Vec<(NestedConsumable, Consumable)>, diesel::result::Error> {
    use schema::nested_consumables::dsl as q;
    use schema::nested_consumables::table;

    let nested_consumables = table
        .filter(q::consumable_id.eq(consumable_id))
        .inner_join(schema::consumables::table.on(schema::consumables::id.eq(q::parent_id)))
        .get_results::<(NestedConsumable, Consumable)>(conn)
        .await?;

    Ok(nested_consumables)
}

// pub async fn get_nested_consumable_by_id(
//     conn: &mut DatabaseConnection,
//     parent_id: i64,
//     consumable_id: i64,
// ) -> Result<Option<NestedConsumable>, diesel::result::Error> {
//     use schema::nested_consumables::consumable_id as q_consumable_id;
//     use schema::nested_consumables::parent_id as q_parent_id;
//     use schema::nested_consumables::table;

//     table
//         .filter(q_parent_id.eq(parent_id))
//         .filter(q_consumable_id.eq(consumable_id))
//         .get_result(conn)
//         .await
//         .optional()
// }

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::nested_consumables)]
pub struct NewNestedConsumable<'a> {
    pub parent_id: i64,
    pub consumable_id: i64,
    pub quantity: Option<f64>,
    pub liquid_mls: Option<f64>,
    pub comments: Option<&'a str>,
}

impl<'a> NewNestedConsumable<'a> {
    pub fn from_front_end(nested_consumable: &'a crate::models::NewNestedConsumable) -> Self {
        let (parent_id, consumable_id) = nested_consumable.id.as_inner();

        Self {
            parent_id: parent_id.as_inner(),
            consumable_id: consumable_id.as_inner(),
            quantity: nested_consumable.quantity,
            liquid_mls: nested_consumable.liquid_mls,
            comments: nested_consumable.comments.as_deref(),
        }
    }
}

pub async fn create_nested_consumable(
    conn: &mut DatabaseConnection,
    update: &NewNestedConsumable<'_>,
) -> Result<NestedConsumable, diesel::result::Error> {
    diesel::insert_into(schema::nested_consumables::table)
        .values(update)
        .returning(NestedConsumable::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::nested_consumables)]
pub struct ChangeNestedConsumable<'a> {
    pub quantity: Option<Option<f64>>,
    pub liquid_mls: Option<Option<f64>>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> ChangeNestedConsumable<'a> {
    pub fn from_front_end(nested_consumable: &'a crate::models::ChangeNestedConsumable) -> Self {
        Self {
            quantity: nested_consumable.quantity.into_option(),
            liquid_mls: nested_consumable.liquid_mls.into_option(),
            comments: nested_consumable.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_nested_consumable(
    conn: &mut DatabaseConnection,
    parent_id: i64,
    consumable_id: i64,
    update: &ChangeNestedConsumable<'_>,
) -> Result<NestedConsumable, diesel::result::Error> {
    diesel::update(
        schema::nested_consumables::table
            .filter(schema::nested_consumables::parent_id.eq(parent_id))
            .filter(schema::nested_consumables::consumable_id.eq(consumable_id)),
    )
    .set(update)
    .returning(NestedConsumable::as_returning())
    .get_result(conn)
    .await
}

pub async fn delete_nested_consumable(
    conn: &mut DatabaseConnection,
    parent_id: i64,
    consumable_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::nested_consumables::consumable_id as q_consumable_id;
    use schema::nested_consumables::parent_id as q_parent_id;
    use schema::nested_consumables::table;

    diesel::delete(
        table
            .filter(q_parent_id.eq(parent_id))
            .filter(q_consumable_id.eq(consumable_id)),
    )
    .execute(conn)
    .await?;
    Ok(())
}
