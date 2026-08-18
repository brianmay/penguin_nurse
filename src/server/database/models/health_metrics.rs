use crate::models::{HealthMetricId, UserId};
use crate::server::database::models::DatabaseConversionError;
use crate::server::database::{connection::DatabaseConnection, schema};
use chrono::Utc;
use diesel::prelude::*;
use diesel::{ExpressionMethods, QueryDsl, Queryable, Selectable};
use diesel_async::RunQueryDsl;
use tap::Pipe;

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
    pub waist_circumference: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub body_fat_pct: Option<bigdecimal::BigDecimal>,
    pub bia_details: Option<serde_json::Value>,
}

impl TryFrom<HealthMetric> for crate::models::HealthMetric {
    type Error = DatabaseConversionError;

    fn try_from(health_metric: HealthMetric) -> Result<Self, Self::Error> {
        let timezone = chrono::FixedOffset::east_opt(health_metric.utc_offset)
            .ok_or(DatabaseConversionError::InvalidValue)?;
        let time = health_metric.time.with_timezone(&timezone);

        Self {
            id: HealthMetricId::new(health_metric.id),
            user_id: UserId::new(health_metric.user_id),
            time,
            pulse: health_metric.pulse.map(|p| p.try_into()).transpose()?,
            blood_glucose: health_metric.blood_glucose,
            systolic_bp: health_metric
                .systolic_bp
                .map(|p| p.try_into())
                .transpose()?,
            diastolic_bp: health_metric
                .diastolic_bp
                .map(|p| p.try_into())
                .transpose()?,
            weight: health_metric.weight,
            height: health_metric.height.map(|p| p.try_into()).transpose()?,
            waist_circumference: health_metric.waist_circumference,
            created_at: health_metric.created_at,
            updated_at: health_metric.updated_at,
            comments: health_metric.comments,
            body_fat_pct: health_metric.body_fat_pct,
            bia_details: health_metric.bia_details,
        }
        .pipe(Ok)
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
        .select(HealthMetric::as_select())
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
        .select(HealthMetric::as_select())
        .filter(q_id.eq(id))
        .filter(q_user_id.eq(user_id))
        .get_result(conn)
        .await
        .optional()
}

pub async fn get_latest_height(
    conn: &mut DatabaseConnection,
    user_id: i64,
) -> Result<Option<i32>, diesel::result::Error> {
    use crate::server::database::schema::health_metrics::height as q_height;
    use crate::server::database::schema::health_metrics::table;
    use crate::server::database::schema::health_metrics::time as q_time;
    use crate::server::database::schema::health_metrics::user_id as q_user_id;

    table
        .select(HealthMetric::as_select())
        .filter(q_user_id.eq(user_id))
        .filter(q_height.is_not_null())
        .order(q_time.desc())
        .first(conn)
        .await
        .optional()
        .map(|hm| hm.and_then(|hm| hm.height))
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
    waist_circumference: Option<&'a bigdecimal::BigDecimal>,
    comments: Option<&'a str>,
    body_fat_pct: Option<&'a bigdecimal::BigDecimal>,
    bia_details: Option<&'a serde_json::Value>,
}

impl<'a> NewHealthMetric<'a> {
    pub fn from_front_end(health_metric: &'a crate::models::NewHealthMetric) -> Self {
        Self {
            user_id: health_metric.user_id.as_inner(),
            time: health_metric.time.with_timezone(&Utc),
            utc_offset: health_metric.time.offset().local_minus_utc(),
            pulse: health_metric.pulse.map(|p| p.into()),
            blood_glucose: health_metric.blood_glucose.as_ref(),
            systolic_bp: health_metric.systolic_bp.map(|p| p.into()),
            diastolic_bp: health_metric.diastolic_bp.map(|p| p.into()),
            weight: health_metric.weight.as_ref(),
            height: health_metric.height.map(|p| p.into()),
            waist_circumference: health_metric.waist_circumference.as_ref(),
            comments: health_metric.comments.as_deref(),
            body_fat_pct: health_metric.body_fat_pct.as_ref(),
            bia_details: health_metric.bia_details.as_ref(),
        }
    }

    pub fn for_kiosk(
        user_id: i64,
        weight_kg: &'a bigdecimal::BigDecimal,
        body_fat_pct: Option<&'a bigdecimal::BigDecimal>,
        bia_details: Option<&'a serde_json::Value>,
    ) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            time: now,
            utc_offset: 0,
            pulse: None,
            blood_glucose: None,
            systolic_bp: None,
            diastolic_bp: None,
            weight: Some(weight_kg),
            height: None,
            waist_circumference: None,
            comments: None,
            body_fat_pct,
            bia_details,
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
    waist_circumference: Option<Option<&'a bigdecimal::BigDecimal>>,
    comments: Option<Option<&'a str>>,
    body_fat_pct: Option<Option<&'a bigdecimal::BigDecimal>>,
    bia_details: Option<Option<&'a serde_json::Value>>,
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
            pulse: health_metric.pulse.map_inner_into().into_option(),
            blood_glucose: health_metric.blood_glucose.as_inner_ref().into_option(),
            systolic_bp: health_metric.systolic_bp.map_inner_into().into_option(),
            diastolic_bp: health_metric.diastolic_bp.map_inner_into().into_option(),
            weight: health_metric.weight.as_inner_ref().into_option(),
            height: health_metric.height.map_inner_into().into_option(),
            waist_circumference: health_metric
                .waist_circumference
                .as_inner_ref()
                .into_option(),
            comments: health_metric.comments.map_inner_deref().into_option(),
            body_fat_pct: health_metric.body_fat_pct.as_inner_ref().into_option(),
            bia_details: health_metric.bia_details.as_inner_ref().into_option(),
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
