use diesel::prelude::*;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use crate::models;
use crate::server::database::models::consumables::Consumable;
use crate::server::database::models::consumption_consumables::ConsumptionConsumable;
use crate::server::database::{connection::DatabaseConnection, schema};
use crate::server::datetime;
use diesel::pg::data_types::PgInterval;
use jiff_diesel::ToDiesel;

#[derive(diesel_derive_enum::DbEnum, Debug, Copy, Clone)]
#[db_enum(existing_type_path = "schema::sql_types::ConsumptionType")]
pub enum ConsumptionType {
    Digest,
    InhaleNose,
    InhaleMouth,
    SpitOut,
    Inject,
    ApplySkin,
    ApplyLips,
    ApplyToes,
    ApplyEars,
    ApplyHair,
    ApplyEyes,
    ApplyFace,
    ApplyHands,
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
            ConsumptionType::ApplyLips => models::ConsumptionType::ApplyLips,
            ConsumptionType::ApplyToes => models::ConsumptionType::ApplyToes,
            ConsumptionType::ApplyEars => models::ConsumptionType::ApplyEars,
            ConsumptionType::ApplyHair => models::ConsumptionType::ApplyHair,
            ConsumptionType::ApplyEyes => models::ConsumptionType::ApplyEyes,
            ConsumptionType::ApplyFace => models::ConsumptionType::ApplyFace,
            ConsumptionType::ApplyHands => models::ConsumptionType::ApplyHands,
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
            models::ConsumptionType::ApplyLips => ConsumptionType::ApplyLips,
            models::ConsumptionType::ApplyToes => ConsumptionType::ApplyToes,
            models::ConsumptionType::ApplyEars => ConsumptionType::ApplyEars,
            models::ConsumptionType::ApplyHair => ConsumptionType::ApplyHair,
            models::ConsumptionType::ApplyEyes => ConsumptionType::ApplyEyes,
            models::ConsumptionType::ApplyFace => ConsumptionType::ApplyFace,
            models::ConsumptionType::ApplyHands => ConsumptionType::ApplyHands,
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
    pub time: jiff_diesel::Timestamp,
    pub duration: Option<jiff_diesel::Span>,
    pub liquid_mls: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub created_at: jiff_diesel::Timestamp,
    pub updated_at: jiff_diesel::Timestamp,
    pub utc_offset: i32,
    pub consumption_type: ConsumptionType,
    pub complete: bool,
}

impl From<Consumption> for crate::models::Consumption {
    fn from(consumption: Consumption) -> Self {
        let time = datetime::zoned_from_parts(consumption.time.to_jiff(), consumption.utc_offset);

        Self {
            id: models::ConsumptionId::new(consumption.id),
            user_id: models::UserId::new(consumption.user_id),
            time,
            duration: consumption
                .duration
                .map(|span| datetime::span_to_signed_duration(span.to_jiff())),
            liquid_mls: consumption.liquid_mls,
            comments: consumption.comments,
            complete: consumption.complete,
            created_at: consumption.created_at.to_jiff(),
            updated_at: consumption.updated_at.to_jiff(),
            consumption_type: consumption.consumption_type.into(),
        }
    }
}

pub async fn get_consumptions_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: jiff::Timestamp,
    end: jiff::Timestamp,
) -> Result<Vec<(Consumption, Vec<(ConsumptionConsumable, Consumable)>)>, diesel::result::Error> {
    let consumptions: Vec<Consumption> = {
        use crate::server::database::schema::consumptions::table;
        use crate::server::database::schema::consumptions::time as q_time;
        use crate::server::database::schema::consumptions::user_id as q_user_id;

        table
            .select(Consumption::as_select())
            .filter(q_user_id.eq(user_id))
            .filter(q_time.ge(start.to_diesel()))
            .filter(q_time.lt(end.to_diesel()))
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

pub async fn get_consumptions_incomplete(
    conn: &mut DatabaseConnection,
    user_id: i64,
) -> Result<Vec<(Consumption, Vec<(ConsumptionConsumable, Consumable)>)>, diesel::result::Error> {
    let consumptions: Vec<Consumption> = {
        use crate::server::database::schema::consumptions::complete as q_complete;
        use crate::server::database::schema::consumptions::table;
        use crate::server::database::schema::consumptions::user_id as q_user_id;

        table
            .select(Consumption::as_select())
            .filter(q_user_id.eq(user_id))
            .filter(q_complete.eq(false))
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
        .select(Consumption::as_select())
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
    pub time: jiff_diesel::Timestamp,
    pub utc_offset: i32,
    pub duration: Option<PgInterval>,
    pub consumption_type: ConsumptionType,
    pub liquid_mls: Option<&'a bigdecimal::BigDecimal>,
    pub comments: Option<&'a str>,
    pub complete: bool,
}

impl<'a> NewConsumption<'a> {
    pub fn from_front_end(consumption: &'a crate::models::NewConsumption) -> Self {
        Self {
            user_id: consumption.user_id.as_inner(),
            time: consumption.time.timestamp().to_diesel(),
            utc_offset: consumption.time.offset().seconds(),
            duration: consumption
                .duration
                .map(datetime::signed_duration_to_interval),
            consumption_type: consumption.consumption_type.into(),
            liquid_mls: consumption.liquid_mls.as_ref(),
            comments: consumption.comments.as_deref(),
            complete: consumption.complete,
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
pub struct ChangeConsumption<'a> {
    pub time: Option<jiff_diesel::Timestamp>,
    pub utc_offset: Option<i32>,
    pub duration: Option<Option<PgInterval>>,
    pub consumption_type: Option<ConsumptionType>,
    pub liquid_mls: Option<Option<&'a bigdecimal::BigDecimal>>,
    pub comments: Option<Option<&'a str>>,
    pub complete: Option<bool>,
}

impl<'a> ChangeConsumption<'a> {
    pub fn from_front_end(consumption: &'a crate::models::ChangeConsumption) -> Self {
        Self {
            time: consumption
                .time
                .clone()
                .map(|time| time.timestamp().to_diesel())
                .into_option(),
            utc_offset: consumption
                .time
                .clone()
                .map(|time| time.offset().seconds())
                .into_option(),
            duration: consumption
                .duration
                .map(|duration| duration.map(datetime::signed_duration_to_interval))
                .into_option(),
            consumption_type: consumption.consumption_type.map_into().into_option(),
            liquid_mls: consumption.liquid_mls.as_inner_ref().into_option(),
            comments: consumption.comments.map_inner_deref().into_option(),
            complete: consumption.complete.into_option(),
        }
    }
}

pub async fn update_consumption(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeConsumption<'_>,
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
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::consumptions::id as q_id;
    use schema::consumptions::table;
    use schema::consumptions::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
