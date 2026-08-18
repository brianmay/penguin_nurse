use std::env;

use axum::{
    Extension, Json, Router,
    body::Body,
    extract::{Query, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use chrono::{Datelike, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::server::database::connection::DatabasePool;
use crate::server::database::models::health_metrics as db_health_metrics;
use crate::server::database::models::users::{self as db_users, Sex};

// ---------------------------------------------------------------------------
// API key authentication middleware
// ---------------------------------------------------------------------------

async fn api_key_auth(req: Request<Body>, next: Next) -> Response<Body> {
    let api_key = match env::var("KIOSK_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error": "kiosk API not configured"})),
            )
                .into_response();
        }
    };

    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok());

    let provided_key = auth_header
        .and_then(|h| h.strip_prefix("Bearer "))
        .unwrap_or("");

    if provided_key.len() != api_key.len()
        || !constant_time_eq(provided_key.as_bytes(), api_key.as_bytes())
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"error": "unauthorized"})),
        )
            .into_response();
    }

    next.run(req).await
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        result |= x ^ y;
    }
    result == 0
}

// ---------------------------------------------------------------------------
// JSON error helper
// ---------------------------------------------------------------------------

fn error_response(status: StatusCode, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(serde_json::json!({"error": message})))
}

// ---------------------------------------------------------------------------
// GET /api/users
// ---------------------------------------------------------------------------

#[derive(Serialize)]
struct KioskUser {
    user_id: i64,
    name: String,
    age: u8,
    sex: String,
    height: Option<u16>,
}

#[derive(Serialize)]
struct UsersResponse {
    users: Vec<KioskUser>,
}

async fn get_users(
    State(pool): State<DatabasePool>,
) -> Result<Json<UsersResponse>, (StatusCode, Json<serde_json::Value>)> {
    let mut conn = pool.get().await.map_err(|e| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("database pool error: {e}"),
        )
    })?;

    let db_user_list = db_users::get_users(&mut conn).await.map_err(|e| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("database error: {e}"),
        )
    })?;

    let today = Utc::now().date_naive();
    let mut users = Vec::with_capacity(db_user_list.len());

    for db_user in db_user_list {
        let age = compute_age(db_user.date_of_birth, today);
        let sex = match db_user.sex {
            Some(s) => sex_to_string(s),
            None => continue,
        };
        let height = db_health_metrics::get_latest_height(&mut conn, db_user.id)
            .await
            .map_err(|e| {
                error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &format!("database error: {e}"),
                )
            })?
            .map(|h| h as u16);

        users.push(KioskUser {
            user_id: db_user.id,
            name: db_user.full_name,
            age,
            sex,
            height,
        });
    }

    Ok(Json(UsersResponse { users }))
}

fn compute_age(date_of_birth: Option<chrono::NaiveDate>, today: chrono::NaiveDate) -> u8 {
    match date_of_birth {
        Some(dob) => {
            let years = today.year() - dob.year();
            if today.month() < dob.month()
                || (today.month() == dob.month() && today.day() < dob.day())
            {
                (years - 1) as u8
            } else {
                years as u8
            }
        }
        None => 0,
    }
}

fn sex_to_string(sex: Sex) -> String {
    match sex {
        Sex::Male => "male".to_string(),
        Sex::Female => "female".to_string(),
    }
}

// ---------------------------------------------------------------------------
// POST /api/measurements
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct MeasurementRequest {
    user_id: i64,
    weight_g: u32,
    body_fat_deci_pct: Option<u16>,
    details: Option<serde_json::Value>,
    timezone: String,
}

#[derive(Serialize)]
struct MeasurementResponse {
    status: String,
    id: String,
    timestamp: String,
}

async fn post_measurement(
    State(pool): State<DatabasePool>,
    Extension(injection): Extension<InjectionState>,
    Json(body): Json<MeasurementRequest>,
) -> Result<(StatusCode, Json<MeasurementResponse>), (StatusCode, Json<serde_json::Value>)> {
    if let Some((status, message)) = injection.take().await {
        return Err((
            StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(serde_json::json!({"error": message})),
        ));
    }

    if body.weight_g > 500_000 {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "'weight_g' must be a non-negative integer (grams)",
        ));
    }

    if let Some(bfp) = body.body_fat_deci_pct
        && bfp > 1000
    {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "'body_fat_deci_pct' must be an integer in 0..=1000 (deci-%)",
        ));
    }

    if let Some(ref details) = body.details
        && !details.is_object()
    {
        return Err(error_response(
            StatusCode::BAD_REQUEST,
            "'details' must be a JSON object",
        ));
    }

    let mut conn = pool.get().await.map_err(|e| {
        error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            &format!("database pool error: {e}"),
        )
    })?;

    let user = db_users::get_user_by_id(&mut conn, body.user_id)
        .await
        .map_err(|e| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("database error: {e}"),
            )
        })?;

    if user.is_none() {
        return Err(error_response(
            StatusCode::UNPROCESSABLE_ENTITY,
            &format!("unknown user_id: {}", body.user_id),
        ));
    }

    let weight_kg =
        bigdecimal::BigDecimal::from(body.weight_g) / bigdecimal::BigDecimal::from(1000u32);

    let body_fat_pct = body
        .body_fat_deci_pct
        .map(|v| bigdecimal::BigDecimal::from(v) / bigdecimal::BigDecimal::from(10u32));

    let tz: Tz = body
        .timezone
        .parse()
        .map_err(|e| error_response(StatusCode::BAD_REQUEST, &format!("invalid timezone: {e}")))?;

    let now_utc = Utc::now();
    let now_local = now_utc.with_timezone(&tz);
    let now_fixed = now_local.fixed_offset();
    let bia_details = body.details.unwrap_or(serde_json::json!({}));

    let new_metric = db_health_metrics::NewHealthMetric::for_kiosk(
        body.user_id,
        now_fixed,
        &weight_kg,
        body_fat_pct.as_ref(),
        Some(&bia_details),
    );

    let _record = db_health_metrics::create_health_metric(&mut conn, &new_metric)
        .await
        .map_err(|e| {
            error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &format!("database error: {e}"),
            )
        })?;

    let id = uuid::Uuid::new_v4().to_string();
    let timestamp = now_utc.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    Ok((
        StatusCode::CREATED,
        Json(MeasurementResponse {
            status: "ok".to_string(),
            id,
            timestamp,
        }),
    ))
}

// ---------------------------------------------------------------------------
// Debug endpoints (error injection for kiosk testing)
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct InjectionState {
    inner: std::sync::Arc<RwLock<InjectionInner>>,
}

struct InjectionInner {
    armed: bool,
    status: u16,
    message: String,
    remaining: Option<u32>,
}

impl InjectionState {
    fn new() -> Self {
        Self {
            inner: std::sync::Arc::new(RwLock::new(InjectionInner {
                armed: false,
                status: 400,
                message: "injected test error".to_string(),
                remaining: None,
            })),
        }
    }

    async fn take(&self) -> Option<(u16, String)> {
        let mut inner = self.inner.write().await;
        if !inner.armed {
            return None;
        }
        let status = inner.status;
        let message = inner.message.clone();
        match &mut inner.remaining {
            Some(1) => {
                inner.armed = false;
                inner.remaining = None;
            }
            Some(n) => {
                *n -= 1;
            }
            None => {}
        }
        Some((status, message))
    }

    async fn arm(&self, status: u16, message: String, remaining: Option<u32>) {
        let mut inner = self.inner.write().await;
        inner.armed = true;
        inner.status = status;
        inner.message = message;
        inner.remaining = remaining;
    }

    async fn status(&self) -> serde_json::Value {
        let inner = self.inner.read().await;
        if inner.armed {
            serde_json::json!({
                "armed": true,
                "status": inner.status,
                "message": inner.message,
                "remaining": inner.remaining,
            })
        } else {
            serde_json::json!({"armed": false})
        }
    }

    async fn clear(&self) {
        let mut inner = self.inner.write().await;
        inner.armed = false;
        inner.remaining = None;
    }
}

#[derive(Deserialize)]
struct SimulateErrorParams {
    status: Option<u16>,
    message: Option<String>,
    count: Option<String>,
}

async fn debug_simulate_error(
    Extension(state): Extension<InjectionState>,
    Query(params): Query<SimulateErrorParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let status = params.status.unwrap_or(400);
    let message = params
        .message
        .unwrap_or_else(|| "injected test error".to_string());

    let remaining = match params.count.as_deref() {
        None => None,
        Some("") => None,
        Some(s) => {
            let n: u32 = s.parse().map_err(|_| {
                error_response(
                    StatusCode::BAD_REQUEST,
                    "'count' must be a positive integer",
                )
            })?;
            if n == 0 {
                return Err(error_response(
                    StatusCode::BAD_REQUEST,
                    "'count' must be a positive integer",
                ));
            }
            Some(n)
        }
    };

    state.arm(status, message, remaining).await;
    let status_val = state.status().await;

    Ok(Json(serde_json::json!({"ok": true, "inject": status_val})))
}

async fn debug_status(Extension(state): Extension<InjectionState>) -> Json<serde_json::Value> {
    Json(state.status().await)
}

async fn debug_clear(Extension(state): Extension<InjectionState>) -> Json<serde_json::Value> {
    state.clear().await;
    Json(serde_json::json!({"ok": true}))
}

// ---------------------------------------------------------------------------
// Router
// ---------------------------------------------------------------------------

pub fn router(database: DatabasePool) -> Router {
    let injection_state = InjectionState::new();

    Router::new()
        .route("/api/users", get(get_users))
        .route("/api/measurements", post(post_measurement))
        .route("/api/debug/simulate-error", get(debug_simulate_error))
        .route("/api/debug/status", get(debug_status))
        .route("/api/debug/clear", get(debug_clear))
        .layer(axum::middleware::from_fn(api_key_auth))
        .layer(Extension(injection_state))
        .layer(Extension(database.clone()))
        .with_state(database)
}
