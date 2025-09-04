use crate::models::{HealthMetricId, UserId};
use crate::server::database::{connection::DatabaseConnection, schema};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;

#[allow(dead_code)]
#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::health_metrics)]
pub struct HealthMetric {
    pub id: i64,
    pub user_id: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub utc_offset: i32,
    pub pulse: Option<i32>,
    pub blood_glucose: Option<bigdecimal::BigDecimal>,
    pub systolic_bp: Option<i32>,
    pub diastolic_bp: Option<i32>,
    pub weight: Option<bigdecimal::BigDecimal>,
    pub height: Option<i32>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

const DEFAULT_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(0).unwrap();

impl From<HealthMetric> for crate::models::HealthMetric {
    fn from(health_metric: HealthMetric) -> Self {
        let timezone =
            chrono::FixedOffset::east_opt(health_metric.utc_offset).unwrap_or(DEFAULT_TIMEZONE);
        let time = health_metric.time.with_timezone(&timezone);

        Self {
            id: HealthMetricId::new(health_metric.id),
            user_id: UserId::new(health_metric.user_id),
            time,
            pulse: health_metric.pulse,
            blood_glucose: health_metric.blood_glucose,
            systolic_bp: health_metric.systolic_bp,
            diastolic_bp: health_metric.diastolic_bp,
            weight: health_metric.weight,
            height: health_metric.height,
            created_at: health_metric.created_at,
            updated_at: health_metric.updated_at,
            comments: health_metric.comments,
        }
    }
}

pub async fn get_health_metrics_for_time_range(
    conn: &mut DatabaseConnection,
    user_id: i64,
    start: chrono::DateTime<chrono::Utc>,
    end: chrono::DateTime<chrono::Utc>,
) -> Result<Vec<HealthMetric>, diesel::result::Error> {
    // use crate::server::database::schema::health_metrics::duration as q_duration;
    use crate::server::database::schema::health_metrics::table;
    use crate::server::database::schema::health_metrics::time as q_time;
    use crate::server::database::schema::health_metrics::user_id as q_user_id;

    table
        .filter(q_user_id.eq(user_id))
        .filter(q_time.ge(start))
        .filter(q_time.lt(end))
        .load(conn)
        .await
}

pub async fn get_health_metric_by_id(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<Option<HealthMetric>, diesel::result::Error> {
    use crate::server::database::schema::health_metrics::id as q_id;
    use crate::server::database::schema::health_metrics::table;
    use crate::server::database::schema::health_metrics::user_id as q_user_id;

    table
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

#[derive(Insertable, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::health_metrics)]
pub struct NewHealthMetric<'a> {
    user_id: i64,
    time: chrono::DateTime<chrono::Utc>,
    utc_offset: i32,
    pulse: Option<i32>,
    blood_glucose: Option<&'a bigdecimal::BigDecimal>,
    systolic_bp: Option<i32>,
    diastolic_bp: Option<i32>,
    weight: Option<&'a bigdecimal::BigDecimal>,
    height: Option<i32>,
    comments: Option<&'a str>,
}

impl<'a> NewHealthMetric<'a> {
    pub fn from_front_end(health_metric: &'a crate::models::NewHealthMetric) -> Self {
        Self {
            user_id: health_metric.user_id.as_inner(),
            time: health_metric.time.with_timezone(&Utc),
            utc_offset: health_metric.time.offset().local_minus_utc(),
            pulse: health_metric.pulse,
            blood_glucose: health_metric.blood_glucose.as_ref(),
            systolic_bp: health_metric.systolic_bp,
            diastolic_bp: health_metric.diastolic_bp,
            weight: health_metric.weight.as_ref(),
            height: health_metric.height,
            comments: health_metric.comments.as_deref(),
        }
    }
}

pub async fn create_health_metric(
    conn: &mut DatabaseConnection,
    update: &NewHealthMetric<'_>,
) -> Result<HealthMetric, diesel::result::Error> {
    diesel::insert_into(schema::health_metrics::table)
        .values(update)
        .returning(HealthMetric::as_returning())
        .get_result(conn)
        .await
}

#[derive(AsChangeset, Debug, Clone)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = schema::health_metrics)]
pub struct ChangeHealthMetric<'a> {
    time: Option<chrono::DateTime<chrono::Utc>>,
    utc_offset: Option<i32>,
    pulse: Option<Option<i32>>,
    blood_glucose: Option<Option<&'a bigdecimal::BigDecimal>>,
    systolic_bp: Option<Option<i32>>,
    diastolic_bp: Option<Option<i32>>,
    weight: Option<Option<&'a bigdecimal::BigDecimal>>,
    height: Option<Option<i32>>,
    comments: Option<Option<&'a str>>,
}

impl<'a> ChangeHealthMetric<'a> {
    pub fn from_front_end(health_metric: &'a crate::models::ChangeHealthMetric) -> Self {
        Self {
            time: health_metric
                .time
                .map(|time| time.with_timezone(&Utc))
                .into_option(),
            utc_offset: health_metric
                .time
                .map(|time| time.offset().local_minus_utc())
                .into_option(),
            pulse: health_metric.pulse.into_option(),
            blood_glucose: health_metric.blood_glucose.as_inner_ref().into_option(),
            systolic_bp: health_metric.systolic_bp.into_option(),
            diastolic_bp: health_metric.diastolic_bp.into_option(),
            weight: health_metric.weight.as_inner_ref().into_option(),
            height: health_metric.height.into_option(),
            comments: health_metric.comments.map_inner_deref().into_option(),
        }
    }
}

pub async fn update_health_metric(
    conn: &mut DatabaseConnection,
    id: i64,
    update: &ChangeHealthMetric<'_>,
) -> Result<HealthMetric, diesel::result::Error> {
    diesel::update(schema::health_metrics::table.filter(schema::health_metrics::id.eq(id)))
        .set(update)
        .returning(HealthMetric::as_returning())
        .get_result(conn)
        .await
}

pub async fn delete_health_metric(
    conn: &mut DatabaseConnection,
    id: i64,
    user_id: i64,
) -> Result<(), diesel::result::Error> {
    use schema::health_metrics::id as q_id;
    use schema::health_metrics::table;
    use schema::health_metrics::user_id as q_user_id;

    diesel::delete(table.filter(q_id.eq(id)).filter(q_user_id.eq(user_id)))
        .execute(conn)
        .await?;
    Ok(())
}
