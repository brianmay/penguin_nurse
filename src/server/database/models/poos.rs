use diesel::pg::data_types::PgInterval;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use jiff_diesel::ToDiesel;

use crate::{
    models::{PooId, UserId},
    server::database::{connection::DatabaseConnection, schema},
    server::datetime,
};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::poos)]
pub struct Poo {
    pub id: i64,
    pub user_id: i64,
    pub time: jiff_diesel::Timestamp,
    pub duration: Option<jiff_diesel::Span>,
    pub urgency: i32,
    pub quantity: Option<i32>,
    pub bristol: Option<i32>,
    pub colour_hue: Option<f32>,
    pub colour_saturation: Option<f32>,
    pub colour_value: Option<f32>,
    pub comments: Option<String>,
    pub created_at: jiff_diesel::Timestamp,
    pub updated_at: jiff_diesel::Timestamp,
    pub utc_offset: i32,
    pub complete: bool,
}

impl From<Poo> for crate::models::Poo {
    fn from(poo: Poo) -> Self {
        let time = datetime::zoned_from_parts(poo.time.to_jiff(), poo.utc_offset);

        let colour = match (poo.colour_hue, poo.colour_saturation, poo.colour_value) {
            (Some(hue), Some(saturation), Some(value)) => {
                Some(palette::Hsv::new(hue, saturation, value))
            }
            _ => None,
        };

        let bristol = poo.bristol.and_then(|b| b.try_into().ok());

        Self {
            id: PooId::new(poo.id),
            user_id: UserId::new(poo.user_id),
            time,
            duration: poo
                .duration
                .map(|span| datetime::span_to_signed_duration(span.to_jiff())),
            urgency: poo.urgency.try_into().unwrap_or_default(),
            quantity: poo.quantity,
            bristol,
            colour,
            comments: poo.comments,
            complete: poo.complete,
            created_at: poo.created_at.to_jiff(),
            updated_at: poo.updated_at.to_jiff(),
        }
    }
}

pub async fn get_poos_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: jiff::Timestamp,
    end: jiff::Timestamp,
) -> Result<Vec<Poo>, diesel::result::Error> {
    // use crate::server::database::schema::wees::duration as q_duration;
    use crate::server::database::schema::poos::table;
    use crate::server::database::schema::poos::time as q_time;
    use crate::server::database::schema::poos::user_id as q_user_id;

    table
        .select(Poo::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start.to_diesel()))
        .filter(q_time.lt(end.to_diesel()))
        .load(conn)
        .await
}

pub async fn get_poos_incomplete(
    conn: &mut DatabaseConnection,
    user_id: i64,
) -> Result<Vec<Poo>, diesel::result::Error> {
    use crate::server::database::schema::poos::complete as q_complete;
    use crate::server::database::schema::poos::table;
    use crate::server::database::schema::poos::user_id as q_user_id;

    table
        .select(Poo::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_complete.eq(false))
        .load(conn)
        .await
}

pub async fn get_poo_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Poo>, diesel::result::Error> {
    use crate::server::database::schema::poos::id as q_id;
    use crate::server::database::schema::poos::table;
    use crate::server::database::schema::poos::user_id as q_user_id;

    table
        .select(Poo::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::poos)]
pub struct NewPoo<'a> {
    user_id: i64,
    time: jiff_diesel::Timestamp,
    utc_offset: i32,
    duration: Option<PgInterval>,
    urgency: i32,
    quantity: Option<i32>,
    bristol: Option<i32>,
    colour_hue: Option<f32>,
    colour_saturation: Option<f32>,
    colour_value: Option<f32>,
    comments: Option<&'a str>,
    complete: bool,
}

impl<'a> NewPoo<'a> {
    pub fn from_front_end(poo: &'a crate::models::NewPoo) -> Self {
        Self {
            user_id: poo.user_id.as_inner(),
            time: poo.time.timestamp().to_diesel(),
            utc_offset: poo.time.offset().seconds(),
            duration: poo.duration.map(datetime::signed_duration_to_interval),
            urgency: poo.urgency.into(),
            quantity: poo.quantity,
            bristol: poo.bristol.map(|b| b.into()),
            colour_hue: poo.colour.map(|colour| colour.hue.into_inner()),
            colour_saturation: poo.colour.map(|colour| colour.saturation),
            colour_value: poo.colour.map(|colour| colour.value),
            comments: poo.comments.as_deref(),
            complete: poo.complete,
        }
    }
}

pub async fn create_poo(
    conn: &mut DatabaseConnection,
    new_poo: NewPoo<'_>,
) -> Result<Poo, diesel::result::Error> {
    use crate::server::database::schema::poos::table;

    diesel::insert_into(table)
        .values(new_poo)
        .returning(Poo::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::poos)]
pub struct ChangePoo<'a> {
    pub time: Option<jiff_diesel::Timestamp>,
    pub utc_offset: Option<i32>,
    pub duration: Option<Option<PgInterval>>,
    pub urgency: Option<i32>,
    pub quantity: Option<Option<i32>>,
    pub bristol: Option<Option<i32>>,
    pub colour_hue: Option<Option<f32>>,
    pub colour_saturation: Option<Option<f32>>,
    pub colour_value: Option<Option<f32>>,
    pub comments: Option<Option<&'a str>>,
    pub complete: Option<bool>,
}

impl<'a> ChangePoo<'a> {
    pub fn from_front_end(poo: &'a crate::models::ChangePoo) -> Self {
        Self {
            time: poo
                .time
                .clone()
                .map(|time| time.timestamp().to_diesel())
                .into_option(),
            utc_offset: poo
                .time
                .clone()
                .map(|time| time.offset().seconds())
                .into_option(),
            duration: poo
                .duration
                .map(|duration| duration.map(datetime::signed_duration_to_interval))
                .into_option(),
            urgency: poo.urgency.map_into().into_option(),
            quantity: poo.quantity.into_option(),
            bristol: poo.bristol.map(|x| x.map(|b| b.into())).into_option(),
            colour_hue: poo
                .colour
                .map(|x| x.map(|colour| colour.hue.into_inner()))
                .into_option(),
            colour_saturation: poo
                .colour
                .map(|x| x.map(|colour| colour.saturation))
                .into_option(),
            colour_value: poo
                .colour
                .map(|x| x.map(|colour| colour.value))
                .into_option(),
            comments: poo.comments.map_inner_deref().into_option(),
            complete: poo.complete.into_option(),
        }
    }
}

pub async fn update_poo(
    conn: &mut DatabaseConnection,
    id: i64,
    updates: ChangePoo<'_>,
) -> Result<Poo, diesel::result::Error> {
    use crate::server::database::schema::poos::id as q_id;
    use crate::server::database::schema::poos::table;

    diesel::update(table.filter(q_id.eq(id)))
        .set(updates)
        .returning(Poo::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_poo(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use crate::server::database::schema::poos::id as q_id;
    use crate::server::database::schema::poos::table;
    use crate::server::database::schema::poos::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
