use diesel::prelude::*;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::Utc;
use chrono::{DateTime, TimeDelta};

use crate::models;
use crate::server::database::models::consumables::Consumable;
use crate::server::database::models::consumption_consumables::ConsumptionConsumable;
use crate::server::database::{connection::DatabaseConnection, schema};

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone)]
#[db_enum(existing_type_path = "schema::sql_types::ConsumptionType")]
pub enum ConsumptionType {
    Digest,
    InhaleNose,
    InhaleMouth,
    SpitOut,
    Inject,
    ApplySkin,
}

impl From<ConsumptionType> for models::ConsumptionType {
    fn from(unit: ConsumptionType) -> models::ConsumptionType {
        match unit {
            ConsumptionType::Digest => models::ConsumptionType::Digest,
            ConsumptionType::InhaleNose => models::ConsumptionType::InhaleNose,
            ConsumptionType::InhaleMouth => models::ConsumptionType::InhaleMouth,
            ConsumptionType::SpitOut => models::ConsumptionType::SpitOut,
            ConsumptionType::Inject => models::ConsumptionType::Inject,
            ConsumptionType::ApplySkin => models::ConsumptionType::ApplySkin,
        }
    }
}

impl From<models::ConsumptionType> for ConsumptionType {
    fn from(unit: models::ConsumptionType) -> ConsumptionType {
        match unit {
            models::ConsumptionType::Digest => ConsumptionType::Digest,
            models::ConsumptionType::InhaleNose => ConsumptionType::InhaleNose,
            models::ConsumptionType::InhaleMouth => ConsumptionType::InhaleMouth,
            models::ConsumptionType::SpitOut => ConsumptionType::SpitOut,
            models::ConsumptionType::Inject => ConsumptionType::Inject,
            models::ConsumptionType::ApplySkin => ConsumptionType::ApplySkin,
        }
    }
}

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
    pub utc_offset: i32,
    pub consumption_type: ConsumptionType,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<Consumption> for crate::models::Consumption {
    fn from(consumption: Consumption) -> Self {
        let timezone =
            chrono::FixedOffset::east_opt(consumption.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = consumption.time.with_timezone(&timezone);

        Self {
            id: models::ConsumptionId::new(consumption.id),
            user_id: models::UserId::new(consumption.user_id),
            time,
            duration: consumption.duration,
            liquid_mls: consumption.liquid_mls.into(),
            comments: consumption.comments.into(),
            created_at: consumption.created_at,
            updated_at: consumption.updated_at,
            consumption_type: consumption.consumption_type.into(),
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
    pub utc_offset: i32,
    pub duration: TimeDelta,
    pub consumption_type: ConsumptionType,
    pub liquid_mls: Option<f64>,
    pub comments: Option<&'a str>,
}

impl<'a> NewConsumption<'a> {
    pub fn from_front_end(consumption: &'a crate::models::NewConsumption) -> Self {
        Self {
            user_id: consumption.user_id.as_inner(),
            time: consumption.time.with_timezone(&Utc),
            utc_offset: consumption.time.offset().local_minus_utc(),
            duration: consumption.duration,
            consumption_type: consumption.consumption_type.into(),
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
    pub utc_offset: Option<i32>,
    pub duration: Option<TimeDelta>,
    pub consumption_type: Option<ConsumptionType>,
    pub liquid_mls: Option<Option<f64>>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> UpdateConsumption<'a> {
    pub fn from_front_end(consumption: &'a crate::models::UpdateConsumption) -> Self {
        Self {
            time: consumption.time.map(|time| time.with_timezone(&Utc)),
            utc_offset: consumption.time.map(|time| time.offset().local_minus_utc()),
            duration: consumption.duration,
            consumption_type: consumption.consumption_type.map(|x| x.into()),
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
