use crate::server::database::{connection::DatabaseConnection, schema};
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
    pub mls: i32,
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<Wee> for crate::models::Wee {
    fn from(wees: Wee) -> Self {
        Self {
            id: wees.id,
            user_id: wees.user_id,
            time: wees.time,
            duration: wees.duration,
            mls: wees.mls,
            colour: palette::Hsv::new(wees.hue, wees.saturation, wees.value),
            created_at: wees.created_at,
            updated_at: wees.updated_at,
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
