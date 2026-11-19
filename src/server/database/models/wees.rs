use crate::models::{UserId, WeeId};
use crate::server::database::{connection::DatabaseConnection, schema};
use crate::server::datetime;
use diesel::pg::data_types::PgInterval;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use jiff_diesel::ToDiesel;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wees)]
pub struct Wee {
    pub id: i64,
    pub user_id: i64,
    pub time: jiff_diesel::Timestamp,
    pub duration: Option<jiff_diesel::Span>,
    pub urgency: i32,
    pub mls: Option<i32>,
    pub colour_hue: Option<f32>,
    pub colour_saturation: Option<f32>,
    pub colour_value: Option<f32>,
    pub comments: Option<String>,
    pub created_at: jiff_diesel::Timestamp,
    pub updated_at: jiff_diesel::Timestamp,
    pub utc_offset: i32,
    pub leakage: Option<i32>,
    pub complete: bool,
}

impl From<Wee> for crate::models::Wee {
    fn from(wee: Wee) -> Self {
        let time = datetime::zoned_from_parts(wee.time.to_jiff(), wee.utc_offset);

        let colour = match (wee.colour_hue, wee.colour_saturation, wee.colour_value) {
            (Some(hue), Some(saturation), Some(value)) => {
                Some(palette::Hsv::new(hue, saturation, value))
            }
            _ => None,
        };

        Self {
            id: WeeId::new(wee.id),
            user_id: UserId::new(wee.user_id),
            time,
            duration: wee
                .duration
                .map(|span| datetime::span_to_signed_duration(span.to_jiff())),
            urgency: wee.urgency.try_into().unwrap_or_default(),
            leakage: wee.leakage,
            mls: wee.mls,
            colour,
            complete: wee.complete,
            created_at: wee.created_at.to_jiff(),
            updated_at: wee.updated_at.to_jiff(),
            comments: wee.comments,
        }
    }
}

pub async fn get_wees_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: jiff::Timestamp,
    end: jiff::Timestamp,
) -> Result<Vec<Wee>, diesel::result::Error> {
    // use crate::server::database::schema::wees::duration as q_duration;
    use crate::server::database::schema::wees::table;
    use crate::server::database::schema::wees::time as q_time;
    use crate::server::database::schema::wees::user_id as q_user_id;

    table
        .select(Wee::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start.to_diesel()))
        .filter(q_time.lt(end.to_diesel()))
        .load(conn)
        .await
}

pub async fn get_wees_incomplete(
    conn: &mut DatabaseConnection,
    user_id: i64,
) -> Result<Vec<Wee>, diesel::result::Error> {
    use crate::server::database::schema::wees::complete as q_complete;
    use crate::server::database::schema::wees::table;
    use crate::server::database::schema::wees::user_id as q_user_id;

    table
        .select(Wee::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_complete.eq(false))
        .load(conn)
        .await
}

pub async fn get_wee_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Wee>, diesel::result::Error> {
    use crate::server::database::schema::wees::id as q_id;
    use crate::server::database::schema::wees::table;
    use crate::server::database::schema::wees::user_id as q_user_id;

    table
        .select(Wee::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wees)]
pub struct NewWee<'a> {
    user_id: i64,
    time: jiff_diesel::Timestamp,
    utc_offset: i32,
    duration: Option<PgInterval>,
    urgency: i32,
    leakage: Option<i32>,
    mls: Option<i32>,
    colour_hue: Option<f32>,
    colour_saturation: Option<f32>,
    colour_value: Option<f32>,
    comments: Option<&'a str>,
    complete: bool,
}

impl<'a> NewWee<'a> {
    pub fn from_front_end(wee: &'a crate::models::NewWee) -> Self {
        Self {
            user_id: wee.user_id.as_inner(),
            time: wee.time.timestamp().to_diesel(),
            utc_offset: wee.time.offset().seconds(),
            duration: wee.duration.map(datetime::signed_duration_to_interval),
            urgency: wee.urgency.into(),
            leakage: wee.leakage,
            mls: wee.mls,
            colour_hue: wee.colour.map(|colour| colour.hue.into_inner()),
            colour_saturation: wee.colour.map(|colour| colour.saturation),
            colour_value: wee.colour.map(|colour| colour.value),
            comments: wee.comments.as_deref(),
            complete: wee.complete,
        }
    }
}

pub async fn create_wee(
    conn: &mut DatabaseConnection,
    update: &NewWee<'_>,
) -> Result<Wee, diesel::result::Error> {
    diesel::insert_into(schema::wees::table)
        .values(update)
        .returning(Wee::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wees)]
pub struct ChangeWee<'a> {
    time: Option<jiff_diesel::Timestamp>,
    utc_offset: Option<i32>,
    duration: Option<Option<PgInterval>>,
    urgency: Option<i32>,
    leakage: Option<Option<i32>>,
    mls: Option<Option<i32>>,
    colour_hue: Option<Option<f32>>,
    colour_saturation: Option<Option<f32>>,
    colour_value: Option<Option<f32>>,
    comments: Option<Option<&'a str>>,
    complete: Option<bool>,
}

impl<'a> ChangeWee<'a> {
    pub fn from_front_end(wee: &'a crate::models::ChangeWee) -> Self {
        Self {
            time: wee
                .time
                .clone()
                .map(|time| time.timestamp().to_diesel())
                .into_option(),
            utc_offset: wee
                .time
                .clone()
                .map(|time| time.offset().seconds())
                .into_option(),
            duration: wee
                .duration
                .map(|duration| duration.map(datetime::signed_duration_to_interval))
                .into_option(),
            urgency: wee.urgency.map_into().into_option(),
            leakage: wee.leakage.into_option(),
            mls: wee.mls.into_option(),
            colour_hue: wee
                .colour
                .map(|x| x.map(|colour| colour.hue.into_inner()))
                .into_option(),
            colour_saturation: wee
                .colour
                .map(|x| x.map(|colour| colour.saturation))
                .into_option(),
            colour_value: wee
                .colour
                .map(|x| x.map(|colour| colour.value))
                .into_option(),
            comments: wee.comments.as_ref().map(|x| x.as_deref()).into_option(),
            complete: wee.complete.into_option(),
        }
    }
}

pub async fn update_wee(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeWee<'_>,
) -> Result<Wee, diesel::result::Error> {
    diesel::update(schema::wees::table.filter(schema::wees::id.eq(id)))
        .set(update)
        .returning(Wee::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_wee(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::wees::id as q_id;
    use schema::wees::table;
    use schema::wees::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
