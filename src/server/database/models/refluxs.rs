use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::Utc;
use chrono::{DateTime, TimeDelta};

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone, Identifiable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::refluxs)]
pub struct Reflux {
    pub id: i64,
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub utc_offset: i32,
    pub duration: TimeDelta,
    pub location: Option<String>,
    pub severity: i32,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<Reflux> for crate::models::Reflux {
    fn from(reflux: Reflux) -> Self {
        let timezone = chrono::FixedOffset::east_opt(reflux.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = reflux.time.with_timezone(&timezone);

        Self {
            id: models::RefluxId::new(reflux.id),
            user_id: models::UserId::new(reflux.user_id),
            time,
            duration: reflux.duration,
            location: reflux.location,
            severity: reflux.severity,
            comments: reflux.comments,
            created_at: reflux.created_at,
            updated_at: reflux.updated_at,
        }
    }
}

pub async fn get_refluxs_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<Reflux>, diesel::result::Error> {
    // use crate::server::database::schema::refluxs::duration as q_duration;
    use crate::server::database::schema::refluxs::table;
    use crate::server::database::schema::refluxs::time as q_time;
    use crate::server::database::schema::refluxs::user_id as q_user_id;

    table
        .select(Reflux::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

pub async fn get_reflux_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Reflux>, diesel::result::Error> {
    use crate::server::database::schema::refluxs::id as q_id;
    use crate::server::database::schema::refluxs::table;
    use crate::server::database::schema::refluxs::user_id as q_user_id;

    table
        .select(Reflux::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::refluxs)]
pub struct NewReflux<'a> {
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub utc_offset: i32,
    pub duration: TimeDelta,
    pub location: Option<&'a str>,
    pub severity: i32,
    pub comments: Option<&'a str>,
}

impl<'a> NewReflux<'a> {
    pub fn from_front_end(reflux: &'a crate::models::NewReflux) -> Self {
        Self {
            user_id: reflux.user_id.as_inner(),
            time: reflux.time.with_timezone(&Utc),
            utc_offset: reflux.time.offset().local_minus_utc(),
            duration: reflux.duration,
            location: reflux.location.as_deref(),
            severity: reflux.severity,
            comments: reflux.comments.as_deref(),
        }
    }
}

pub async fn create_reflux(
    conn: &mut DatabaseConnection,
    update: &NewReflux<'_>,
) -> Result<Reflux, diesel::result::Error> {
    diesel::insert_into(schema::refluxs::table)
        .values(update)
        .returning(Reflux::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::refluxs)]
pub struct ChangeReflux<'a> {
    pub time: Option<DateTime<Utc>>,
    pub utc_offset: Option<i32>,
    pub duration: Option<TimeDelta>,
    pub location: Option<Option<&'a str>>,
    pub severity: Option<i32>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> ChangeReflux<'a> {
    pub fn from_front_end(reflux: &'a crate::models::ChangeReflux) -> Self {
        Self {
            time: reflux
                .time
                .map(|time| time.with_timezone(&Utc))
                .into_option(),
            utc_offset: reflux
                .time
                .map(|time| time.offset().local_minus_utc())
                .into_option(),
            duration: reflux.duration.into_option(),
            location: reflux.location.map_inner_deref().into_option(),
            severity: reflux.severity.into_option(),
            comments: reflux.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_reflux(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeReflux<'_>,
) -> Result<Reflux, diesel::result::Error> {
    diesel::update(schema::refluxs::table.filter(schema::refluxs::id.eq(id)))
        .set(update)
        .returning(Reflux::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_reflux(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::refluxs::id as q_id;
    use schema::refluxs::table;
    use schema::refluxs::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
