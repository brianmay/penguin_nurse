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
    pub quantity: i32,
    pub bristol: i32,
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Poo> for crate::models::Poo {
    fn from(poos: Poo) -> Self {
        Self {
            id: poos.id,
            user_id: poos.user_id,
            time: poos.time,
            duration: poos.duration,
            quantity: poos.quantity,
            bristol: poos.bristol,
            colour: palette::Hsv::new(poos.hue, poos.saturation, poos.value),
            created_at: poos.created_at,
            updated_at: poos.updated_at,
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
