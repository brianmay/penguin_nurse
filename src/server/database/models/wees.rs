use crate::models::{UserId, WeeId};
use crate::server::database::{connection::DatabaseConnection, schema};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wees)]
pub struct Wee {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub mls: i32,
    pub colour_hue: f32,
    pub colour_saturation: f32,
    pub colour_value: f32,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub utc_offset: i32,
}

impl From<Wee> for crate::models::Wee {
    fn from(wee: Wee) -> Self {
        let timezone = chrono::FixedOffset::east(wee.utc_offset);
        let time = wee.time.with_timezone(&timezone);

        Self {
            id: WeeId::new(wee.id),
            user_id: UserId::new(wee.user_id),
            time,
            duration: wee.duration,
            urgency: wee.urgency,
            mls: wee.mls,
            colour: palette::Hsv::new(wee.colour_hue, wee.colour_saturation, wee.colour_value),
            created_at: wee.created_at,
            updated_at: wee.updated_at,
            comments: wee.comments.into(),
        }
    }
}

pub async fn get_wees_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<Wee>, diesel::result::Error> {
    // use crate::server::database::schema::wees::duration as q_duration;
    use crate::server::database::schema::wees::table;
    use crate::server::database::schema::wees::time as q_time;
    use crate::server::database::schema::wees::user_id as q_user_id;

    table
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
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
    time: chrono::DateTime<chrono::Utc>,
    utc_offset: i32,
    duration: chrono::Duration,
    urgency: i32,
    mls: i32,
    colour_hue: f32,
    colour_saturation: f32,
    colour_value: f32,
    comments: Option<&'a str>,
}

impl<'a> NewWee<'a> {
    pub fn from_front_end(wee: &'a crate::models::NewWee) -> Self {
        Self {
            user_id: wee.user_id.as_inner(),
            time: wee.time.with_timezone(&Utc),
            utc_offset: wee.time.offset().local_minus_utc(),
            duration: wee.duration,
            urgency: wee.urgency,
            mls: wee.mls,
            colour_hue: wee.colour.hue.into_inner(),
            colour_saturation: wee.colour.saturation,
            colour_value: wee.colour.value,
            comments: wee.comments.as_deref(),
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
pub struct UpdateWee<'a> {
    time: Option<chrono::DateTime<chrono::Utc>>,
    utc_offset: Option<i32>,
    duration: Option<chrono::Duration>,
    urgency: Option<i32>,
    mls: Option<i32>,
    colour_hue: Option<f32>,
    colour_saturation: Option<f32>,
    colour_value: Option<f32>,
    comments: Option<Option<&'a str>>,
}

impl<'a> UpdateWee<'a> {
    pub fn from_front_end(wee: &'a crate::models::UpdateWee) -> Self {
        Self {
            time: wee.time.map(|time| time.with_timezone(&Utc)),
            utc_offset: wee.time.map(|time| time.offset().local_minus_utc()),
            duration: wee.duration,
            urgency: wee.urgency,
            mls: wee.mls,
            colour_hue: wee.colour.map(|x| x.hue.into_inner()),
            colour_saturation: wee.colour.map(|x| x.saturation),
            colour_value: wee.colour.map(|x| x.value),
            comments: wee.comments.as_ref().map(|x| x.as_deref()),
        }
    }
}

pub async fn update_wee(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &UpdateWee<'_>,
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
