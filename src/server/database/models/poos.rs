use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::server::database::{connection::DatabaseConnection, schema};

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::poos)]
pub struct Poo {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub quantity: i32,
    pub bristol: i32,
    pub colour_hue: f32,
    pub colour_saturation: f32,
    pub colour_value: f32,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Poo> for crate::models::Poo {
    fn from(poo: Poo) -> Self {
        Self {
            id: poo.id,
            user_id: poo.user_id,
            time: poo.time,
            duration: poo.duration,
            urgency: poo.urgency,
            quantity: poo.quantity,
            bristol: poo.bristol,
            colour: palette::Hsv::new(poo.colour_hue, poo.colour_saturation, poo.colour_value),
            comments: poo.comments.into(),
            created_at: poo.created_at,
            updated_at: poo.updated_at,
        }
    }
}

pub async fn get_poos_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<Poo>, diesel::result::Error> {
    // use crate::server::database::schema::wees::duration as q_duration;
    use crate::server::database::schema::poos::table;
    use crate::server::database::schema::poos::time as q_time;
    use crate::server::database::schema::poos::user_id as q_user_id;

    table
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::poos)]
pub struct NewPoo<'a> {
    user_id: i64,
    time: chrono::DateTime<chrono::Utc>,
    duration: chrono::Duration,
    urgency: i32,
    quantity: i32,
    bristol: i32,
    colour_hue: f32,
    colour_saturation: f32,
    colour_value: f32,
    comments: Option<&'a str>,
}

impl<'a> NewPoo<'a> {
    pub fn from_front_end(poo: &'a crate::models::NewPoo) -> Self {
        Self {
            user_id: poo.user_id,
            time: poo.time,
            duration: poo.duration,
            urgency: poo.urgency,
            quantity: poo.quantity,
            bristol: poo.bristol,
            colour_hue: poo.colour.hue.into_inner(),
            colour_saturation: poo.colour.saturation,
            colour_value: poo.colour.value,
            comments: poo.comments.as_deref(),
        }
    }
}

pub async fn create_poo<'a>(
    conn: &mut DatabaseConnection,
    new_poo: NewPoo<'a>,
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
pub struct UpdatePoo<'a> {
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<chrono::Duration>,
    pub urgency: Option<i32>,
    pub quantity: Option<i32>,
    pub bristol: Option<i32>,
    pub colour_hue: Option<f32>,
    pub colour_saturation: Option<f32>,
    pub colour_value: Option<f32>,
    pub comments: Option<Option<&'a str>>,
}

impl<'a> UpdatePoo<'a> {
    pub fn from_front_end(poo: &'a crate::models::UpdatePoo) -> Self {
        Self {
            time: poo.time,
            duration: poo.duration,
            urgency: poo.urgency,
            quantity: poo.quantity,
            bristol: poo.bristol,
            colour_hue: poo.colour.map(|x| x.hue.into_inner()),
            colour_saturation: poo.colour.map(|x| x.saturation),
            colour_value: poo.colour.map(|x| x.value),
            comments: poo.comments.as_ref().map(|x| x.as_deref()),
        }
    }
}

pub async fn update_poo<'a>(
    conn: &mut DatabaseConnection,
    id: i64,
    updates: UpdatePoo<'a>,
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
