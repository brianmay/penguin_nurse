use diesel::prelude::*;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::Utc;
use chrono::{DateTime, TimeDelta};

use crate::models;
use crate::server::database::models::consumables::Consumable;
use crate::server::database::models::consumption_consumables::ConsumptionConsumable;
use crate::server::database::{connection::DatabaseConnection, schema};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone, Identifiable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumptions)]
pub struct Consumption {
    pub id: i64,
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub duration: TimeDelta,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Consumption> for crate::models::Consumption {
    fn from(consumption: Consumption) -> Self {
        Self {
            id: models::ConsumptionId::new(consumption.id),
            user_id: models::UserId::new(consumption.user_id),
            time: consumption.time,
            duration: consumption.duration,
            liquid_mls: consumption.liquid_mls.into(),
            comments: consumption.comments.into(),
            created_at: consumption.created_at,
            updated_at: consumption.updated_at,
        }
    }
}

pub async fn get_consumptions_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<(Consumption, Vec<(ConsumptionConsumable, Consumable)>)>, diesel::result::Error> {
    let consumptions: Vec<Consumption> = {
        use crate::server::database::schema::consumptions::table;
        use crate::server::database::schema::consumptions::time as q_time;
        use crate::server::database::schema::consumptions::user_id as q_user_id;

        table
            .filter(q_user_id.eq(user_id))
            .filter(q_time.ge(start))
            .filter(q_time.lt(end))
            .load(conn)
            .await?
    };

    let nested: Vec<(ConsumptionConsumable, Consumable)> =
        ConsumptionConsumable::belonging_to(&consumptions)
            .inner_join(schema::consumables::table)
            .load(conn)
            .await?;

    let result: Vec<_> = nested
        .grouped_by(&consumptions)
        .into_iter()
        .zip(consumptions)
        .map(|(a, b)| (b, a))
        .collect();

    Ok(result)
}

pub async fn get_consumption_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Consumption>, diesel::result::Error> {
    use crate::server::database::schema::consumptions::id as q_id;
    use crate::server::database::schema::consumptions::table;
    use crate::server::database::schema::consumptions::user_id as q_user_id;

    table
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumptions)]
pub struct NewConsumption<'a> {
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub duration: TimeDelta,
    pub liquid_mls: Option<f64>,
    pub comments: Option<&'a str>,
}

impl<'a> NewConsumption<'a> {
    pub fn from_front_end(consumption: &'a crate::models::NewConsumption) -> Self {
        Self {
            user_id: consumption.user_id.as_inner(),
            time: consumption.time,
            duration: consumption.duration,
            liquid_mls: consumption.liquid_mls.into(),
            comments: consumption.comments.as_deref(),
        }
    }
}

pub async fn create_consumption(
    conn: &mut DatabaseConnection,
    update: &NewConsumption<'_>,
) -> Result<Consumption, diesel::result::Error> {
    diesel::insert_into(schema::consumptions::table)
        .values(update)
        .returning(Consumption::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::consumptions)]
pub struct UpdateConsumption<'a> {
    pub time: Option<DateTime<Utc>>,
    pub duration: Option<TimeDelta>,
    pub liquid_mls: Option<Option<f64>>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> UpdateConsumption<'a> {
    pub fn from_front_end(consumption: &'a crate::models::UpdateConsumption) -> Self {
        Self {
            time: consumption.time,
            duration: consumption.duration,
            liquid_mls: consumption.liquid_mls.map(|x| x.into()),
            comments: consumption.comments.as_ref().map(|x| x.as_deref()),
        }
    }
}

pub async fn update_consumption(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &UpdateConsumption<'_>,
) -> Result<Consumption, diesel::result::Error> {
    diesel::update(schema::consumptions::table.filter(schema::consumptions::id.eq(id)))
        .set(update)
        .returning(Consumption::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_consumption(
    conn: &mut DatabaseConnection,
    id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::consumptions::id as q_id;
    use schema::consumptions::table;

    diesel::delete(table.filter(q_id.eq(id)))
        .execute(conn)
        .await?;
    Ok(())
}
