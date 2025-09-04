use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

use chrono::DateTime;
use chrono::Utc;

use crate::models;
use crate::server::database::{connection::DatabaseConnection, schema};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone, Identifiable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::notes)]
pub struct Note {
    pub id: i64,
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub utc_offset: i32,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<Note> for crate::models::Note {
    fn from(note: Note) -> Self {
        let timezone = chrono::FixedOffset::east_opt(note.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = note.time.with_timezone(&timezone);

        Self {
            id: models::NoteId::new(note.id),
            user_id: models::UserId::new(note.user_id),
            time,
            comments: note.comments,
            created_at: note.created_at,
            updated_at: note.updated_at,
        }
    }
}

pub async fn get_notes_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<Note>, diesel::result::Error> {
    // use crate::server::database::schema::notes::duration as q_duration;
    use crate::server::database::schema::notes::table;
    use crate::server::database::schema::notes::time as q_time;
    use crate::server::database::schema::notes::user_id as q_user_id;

    table
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

pub async fn get_note_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<Note>, diesel::result::Error> {
    use crate::server::database::schema::notes::id as q_id;
    use crate::server::database::schema::notes::table;
    use crate::server::database::schema::notes::user_id as q_user_id;

    table
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::notes)]
pub struct NewNote<'a> {
    pub user_id: i64,
    pub time: DateTime<Utc>,
    pub utc_offset: i32,
    pub comments: Option<&'a str>,
}

impl<'a> NewNote<'a> {
    pub fn from_front_end(note: &'a crate::models::NewNote) -> Self {
        Self {
            user_id: note.user_id.as_inner(),
            time: note.time.with_timezone(&Utc),
            utc_offset: note.time.offset().local_minus_utc(),
            comments: note.comments.as_deref(),
        }
    }
}

pub async fn create_note(
    conn: &mut DatabaseConnection,
    update: &NewNote<'_>,
) -> Result<Note, diesel::result::Error> {
    diesel::insert_into(schema::notes::table)
        .values(update)
        .returning(Note::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::notes)]
pub struct ChangeNote<'a> {
    pub time: Option<DateTime<Utc>>,
    pub utc_offset: Option<i32>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> ChangeNote<'a> {
    pub fn from_front_end(note: &'a crate::models::ChangeNote) -> Self {
        Self {
            time: note.time.map(|time| time.with_timezone(&Utc)).into_option(),
            utc_offset: note
                .time
                .map(|time| time.offset().local_minus_utc())
                .into_option(),
            comments: note.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_note(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeNote<'_>,
) -> Result<Note, diesel::result::Error> {
    diesel::update(schema::notes::table.filter(schema::notes::id.eq(id)))
        .set(update)
        .returning(Note::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_note(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::notes::id as q_id;
    use schema::notes::table;
    use schema::notes::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
