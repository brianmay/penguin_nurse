use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};
use crate::server::datetime;
use diesel::pg::data_types::PgInterval;
use jiff_diesel::ToDiesel;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone, Identifiable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::refluxs)]
pub struct Reflux {
    pub id: i64,
    pub user_id: i64,
    pub time: jiff_diesel::Timestamp,
    pub utc_offset: i32,
    pub duration: Option<jiff_diesel::Span>,
    pub location: Option<String>,
    pub severity: i32,
    pub comments: Option<String>,
    pub complete: bool,
    pub created_at: jiff_diesel::Timestamp,
    pub updated_at: jiff_diesel::Timestamp,
}

impl From<Reflux> for crate::models::Reflux {
    fn from(reflux: Reflux) -> Self {
        let time = datetime::zoned_from_parts(reflux.time.to_jiff(), reflux.utc_offset);

        Self {
            id: models::RefluxId::new(reflux.id),
            user_id: models::UserId::new(reflux.user_id),
            time,
            duration: reflux
                .duration
                .map(|span| datetime::span_to_signed_duration(span.to_jiff())),
            location: reflux.location,
            severity: reflux.severity,
            comments: reflux.comments,
            complete: reflux.complete,
            created_at: reflux.created_at.to_jiff(),
            updated_at: reflux.updated_at.to_jiff(),
        }
    }
}

pub async fn get_refluxs_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: jiff::Timestamp,
    end: jiff::Timestamp,
) -> Result<Vec<Reflux>, diesel::result::Error> {
    // use crate::server::database::schema::refluxs::duration as q_duration;
    use crate::server::database::schema::refluxs::table;
    use crate::server::database::schema::refluxs::time as q_time;
    use crate::server::database::schema::refluxs::user_id as q_user_id;

    table
        .select(Reflux::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start.to_diesel()))
        .filter(q_time.lt(end.to_diesel()))
        .load(conn)
        .await
}

pub async fn get_refluxs_incomplete(
    conn: &mut DatabaseConnection,
    user_id: i64,
) -> Result<Vec<Reflux>, diesel::result::Error> {
    use crate::server::database::schema::refluxs::complete as q_complete;
    use crate::server::database::schema::refluxs::table;
    use crate::server::database::schema::refluxs::user_id as q_user_id;

    table
        .select(Reflux::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_complete.eq(false))
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
    pub time: jiff_diesel::Timestamp,
    pub utc_offset: i32,
    pub duration: Option<PgInterval>,
    pub location: Option<&'a str>,
    pub severity: i32,
    pub comments: Option<&'a str>,
    pub complete: bool,
}

impl<'a> NewReflux<'a> {
    pub fn from_front_end(reflux: &'a crate::models::NewReflux) -> Self {
        Self {
            user_id: reflux.user_id.as_inner(),
            time: reflux.time.timestamp().to_diesel(),
            utc_offset: reflux.time.offset().seconds(),
            duration: reflux.duration.map(datetime::signed_duration_to_interval),
            location: reflux.location.as_deref(),
            severity: reflux.severity,
            comments: reflux.comments.as_deref(),
            complete: reflux.complete,
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
    pub time: Option<jiff_diesel::Timestamp>,
    pub utc_offset: Option<i32>,
    pub duration: Option<Option<PgInterval>>,
    pub location: Option<Option<&'a str>>,
    pub severity: Option<i32>,
    pub comments: Option<Option<&'a str>>,
    pub complete: Option<bool>,
}

impl<'a> ChangeReflux<'a> {
    pub fn from_front_end(reflux: &'a crate::models::ChangeReflux) -> Self {
        Self {
            time: reflux
                .time
                .clone()
                .map(|time| time.timestamp().to_diesel())
                .into_option(),
            utc_offset: reflux
                .time
                .clone()
                .map(|time| time.offset().seconds())
                .into_option(),
            duration: reflux
                .duration
                .map(|duration| duration.map(datetime::signed_duration_to_interval))
                .into_option(),
            location: reflux.location.map_inner_deref().into_option(),
            severity: reflux.severity.into_option(),
            comments: reflux.comments.map_inner_deref().into_option(),
            complete: reflux.complete.into_option(),
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
