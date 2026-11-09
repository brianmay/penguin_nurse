use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

use super::consumables::Consumable;
use super::schedules::Schedule;

#[derive(Queryable, Selectable, Debug, Clone, Identifiable, Associations)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::schedule_consumables)]
#[diesel(belongs_to(Schedule, foreign_key = parent_id))]
#[diesel(primary_key(parent_id, consumable_id))]
pub struct ScheduleConsumable {
    pub parent_id: i64,
    pub consumable_id: i64,
    pub quantity: Option<bigdecimal::BigDecimal>,
    pub liquid_mls: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<ScheduleConsumable> for crate::models::ScheduleConsumable {
    fn from(schedule_consumable: ScheduleConsumable) -> Self {
        Self {
            id: models::ScheduleConsumableId::new(
                models::ScheduleId::new(schedule_consumable.parent_id),
                models::ConsumableId::new(schedule_consumable.consumable_id),
            ),
            quantity: schedule_consumable.quantity,
            liquid_mls: schedule_consumable.liquid_mls,
            comments: schedule_consumable.comments,
            created_at: schedule_consumable.created_at,
            updated_at: schedule_consumable.updated_at,
        }
    }
}

pub async fn get_child_consumables(
    conn: &mut DatabaseConnection,
    parent_id: i64,
) -> Result<Vec<(ScheduleConsumable, Consumable)>, diesel::result::Error> {
    use schema::consumables::dsl as q_consumables;
    use schema::schedule_consumables::dsl as q;
    use schema::schedule_consumables::table;

    let schedule_consumables = table
        .filter(q::parent_id.eq(parent_id))
        .inner_join(schema::consumables::table.on(schema::consumables::id.eq(q::consumable_id)))
        .order(q_consumables::name.asc())
        .select((ScheduleConsumable::as_select(), Consumable::as_select()))
        .get_results::<(ScheduleConsumable, Consumable)>(conn)
        .await?;

    Ok(schedule_consumables)
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::schedule_consumables)]
pub struct NewScheduleConsumable<'a> {
    pub parent_id: i64,
    pub consumable_id: i64,
    pub quantity: Option<&'a bigdecimal::BigDecimal>,
    pub liquid_mls: Option<&'a bigdecimal::BigDecimal>,
    pub comments: Option<&'a str>,
}

impl<'a> NewScheduleConsumable<'a> {
    pub fn from_front_end(schedule_consumable: &'a crate::models::NewScheduleConsumable) -> Self {
        let (parent_id, consumable_id) = schedule_consumable.id.as_inner();

        Self {
            parent_id: parent_id.as_inner(),
            consumable_id: consumable_id.as_inner(),
            quantity: schedule_consumable.quantity.as_ref(),
            liquid_mls: schedule_consumable.liquid_mls.as_ref(),
            comments: schedule_consumable.comments.as_deref(),
        }
    }
}

pub async fn create_schedule_consumable(
    conn: &mut DatabaseConnection,
    update: &NewScheduleConsumable<'_>,
) -> Result<ScheduleConsumable, diesel::result::Error> {
    diesel::insert_into(schema::schedule_consumables::table)
        .values(update)
        .returning(ScheduleConsumable::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::schedule_consumables)]
pub struct ChangeScheduleConsumable<'a> {
    pub quantity: Option<Option<&'a bigdecimal::BigDecimal>>,
    pub liquid_mls: Option<Option<&'a bigdecimal::BigDecimal>>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> ChangeScheduleConsumable<'a> {
    pub fn from_front_end(
        schedule_consumable: &'a crate::models::ChangeScheduleConsumable,
    ) -> Self {
        Self {
            quantity: schedule_consumable.quantity.as_inner_ref().into_option(),
            liquid_mls: schedule_consumable.liquid_mls.as_inner_ref().into_option(),
            comments: schedule_consumable.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_schedule_consumable(
    conn: &mut DatabaseConnection,
    parent_id: i64,
    consumable_id: i64,
    update: &ChangeScheduleConsumable<'_>,
) -> Result<ScheduleConsumable, diesel::result::Error> {
    diesel::update(
        schema::schedule_consumables::table
            .filter(schema::schedule_consumables::parent_id.eq(parent_id))
            .filter(schema::schedule_consumables::consumable_id.eq(consumable_id)),
    )
    .set(update)
    .returning(ScheduleConsumable::as_returning())
    .get_result(conn)
    .await
}

pub async fn delete_schedule_consumable(
    conn: &mut DatabaseConnection,
    parent_id: i64,
    consumable_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::schedule_consumables::consumable_id as q_consumable_id;
    use schema::schedule_consumables::parent_id as q_parent_id;
    use schema::schedule_consumables::table;

    diesel::delete(
        table
            .filter(q_parent_id.eq(parent_id))
            .filter(q_consumable_id.eq(consumable_id)),
    )
    .execute(conn)
    .await?;
    Ok(())
}
