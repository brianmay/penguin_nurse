use crate::models::{self, HealthMetricId, UserId};
use chrono::{DateTime, Utc};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::common::{get_database_connection, get_user_id};

#[server]
pub async fn get_health_metrics_for_time_range(
    user_id: UserId,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Result<Vec<models::HealthMetric>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    if user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    crate::server::database::models::health_metrics::get_health_metrics_for_time_range(
        &mut conn,
        user_id.as_inner(),
        start,
        end,
    )
    .await
    .map(|x| x.into_iter().map(|y| y.into()).collect())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn get_health_metric_by_id(
    id: HealthMetricId,
) -> Result<Option<models::HealthMetric>, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::health_metrics::get_health_metric_by_id(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map(|x| x.map(|y| y.into()))
    .map_err(ServerFnError::from)
}

#[server]
pub async fn create_health_metric(
    health_metric: models::NewHealthMetric,
) -> Result<models::HealthMetric, ServerFnError> {
    use crate::server::database::models::health_metrics;

    let logged_in_user_id = get_user_id().await?;

    if health_metric.user_id != logged_in_user_id {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let new_health_metric = health_metrics::NewHealthMetric::from_front_end(&health_metric);

    crate::server::database::models::health_metrics::create_health_metric(
        &mut conn,
        &new_health_metric,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn update_health_metric(
    id: HealthMetricId,
    health_metric: models::ChangeHealthMetric,
) -> Result<models::HealthMetric, ServerFnError> {
    let logged_in_user_id = get_user_id().await?;

    if let Some(req_user_id) = health_metric.user_id
        && logged_in_user_id != req_user_id
    {
        return Err(ServerFnError::ServerError(
            "User ID does not match the logged in user".to_string(),
        ));
    }

    let mut conn = get_database_connection().await?;
    let updates =
        crate::server::database::models::health_metrics::ChangeHealthMetric::from_front_end(
            &health_metric,
        );

    crate::server::database::models::health_metrics::update_health_metric(
        &mut conn,
        id.as_inner(),
        &updates,
    )
    .await
    .map(|x| x.into())
    .map_err(ServerFnError::from)
}

#[server]
pub async fn delete_health_metric(id: HealthMetricId) -> Result<(), ServerFnError> {
    let logged_in_user_id = get_user_id().await?;
    let mut conn = get_database_connection().await?;

    crate::server::database::models::health_metrics::delete_health_metric(
        &mut conn,
        id.as_inner(),
        logged_in_user_id.as_inner(),
    )
    .await
    .map_err(ServerFnError::from)
}
