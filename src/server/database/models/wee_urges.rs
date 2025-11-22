use crate::models::{UserId, WeeUrgeId};
use crate::server::database::{connection::DatabaseConnection, schema};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wee_urges)]
pub struct WeeUrge {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub utc_offset: i32,
    pub urgency: i32,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<WeeUrge> for crate::models::WeeUrge {
    fn from(wee_urge: WeeUrge) -> Self {
        let timezone =
            chrono::FixedOffset::east_opt(wee_urge.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = wee_urge.time.with_timezone(&timezone);

        Self {
            id: WeeUrgeId::new(wee_urge.id),
            user_id: UserId::new(wee_urge.user_id),
            time,
            urgency: wee_urge.urgency.try_into().unwrap_or_default(),
            created_at: wee_urge.created_at,
            updated_at: wee_urge.updated_at,
            comments: wee_urge.comments,
        }
    }
}

pub async fn get_wee_urges_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<WeeUrge>, diesel::result::Error> {
    // use crate::server::database::schema::wee_urges::duration as q_duration;
    use crate::server::database::schema::wee_urges::table;
    use crate::server::database::schema::wee_urges::time as q_time;
    use crate::server::database::schema::wee_urges::user_id as q_user_id;

    table
        .select(WeeUrge::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

pub async fn get_wee_urge_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<WeeUrge>, diesel::result::Error> {
    use crate::server::database::schema::wee_urges::id as q_id;
    use crate::server::database::schema::wee_urges::table;
    use crate::server::database::schema::wee_urges::user_id as q_user_id;

    table
        .select(WeeUrge::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wee_urges)]
pub struct NewWeeUrge<'a> {
    user_id: i64,
    time: chrono::DateTime<chrono::Utc>,
    utc_offset: i32,
    urgency: i32,
    comments: Option<&'a str>,
}

impl<'a> NewWeeUrge<'a> {
    pub fn from_front_end(wee_urge: &'a crate::models::NewWeeUrge) -> Self {
        Self {
            user_id: wee_urge.user_id.as_inner(),
            time: wee_urge.time.with_timezone(&Utc),
            utc_offset: wee_urge.time.offset().local_minus_utc(),
            urgency: wee_urge.urgency.into(),
            comments: wee_urge.comments.as_deref(),
        }
    }
}

pub async fn create_wee_urge(
    conn: &mut DatabaseConnection,
    update: &NewWeeUrge<'_>,
) -> Result<WeeUrge, diesel::result::Error> {
    diesel::insert_into(schema::wee_urges::table)
        .values(update)
        .returning(WeeUrge::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::wee_urges)]
pub struct ChangeWeeUrge<'a> {
    time: Option<chrono::DateTime<chrono::Utc>>,
    utc_offset: Option<i32>,
    urgency: Option<i32>,
    comments: Option<Option<&'a str>>,
}

impl<'a> ChangeWeeUrge<'a> {
    pub fn from_front_end(wee: &'a crate::models::ChangeWeeUrge) -> Self {
        Self {
            time: wee.time.map(|time| time.with_timezone(&Utc)).into_option(),
            utc_offset: wee
                .time
                .map(|time| time.offset().local_minus_utc())
                .into_option(),
            urgency: wee.urgency.map_into().into_option(),
            comments: wee.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_wee_urge(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeWeeUrge<'_>,
) -> Result<WeeUrge, diesel::result::Error> {
    diesel::update(schema::wee_urges::table.filter(schema::wee_urges::id.eq(id)))
        .set(update)
        .returning(WeeUrge::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_wee_urge(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::wee_urges::id as q_id;
    use schema::wee_urges::table;
    use schema::wee_urges::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
