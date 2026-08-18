use chrono::Local;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::MaybeSet;

use super::UserId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HealthMetricId(i64);

#[allow(dead_code)]
impl HealthMetricId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for HealthMetricId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for HealthMetricId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct HealthMetric {
    pub id: HealthMetricId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub pulse: Option<u16>,
    pub blood_glucose: Option<bigdecimal::BigDecimal>,
    pub systolic_bp: Option<u16>,
    pub diastolic_bp: Option<u16>,
    pub weight: Option<bigdecimal::BigDecimal>,
    pub height: Option<u16>,
    pub waist_circumference: Option<bigdecimal::BigDecimal>,
    pub hip_circumference: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub body_fat_pct: Option<bigdecimal::BigDecimal>,
    pub bia_details: Option<serde_json::Value>,
}

#[allow(dead_code)]
impl HealthMetric {
    pub fn name(&self) -> String {
        self.time.with_timezone(&Local).time().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewHealthMetric {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub pulse: Option<u16>,
    pub blood_glucose: Option<bigdecimal::BigDecimal>,
    pub systolic_bp: Option<u16>,
    pub diastolic_bp: Option<u16>,
    pub weight: Option<bigdecimal::BigDecimal>,
    pub height: Option<u16>,
    pub waist_circumference: Option<bigdecimal::BigDecimal>,
    pub hip_circumference: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub body_fat_pct: Option<bigdecimal::BigDecimal>,
    pub bia_details: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeHealthMetric {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub pulse: MaybeSet<Option<u16>>,
    pub blood_glucose: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub systolic_bp: MaybeSet<Option<u16>>,
    pub diastolic_bp: MaybeSet<Option<u16>>,
    pub weight: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub height: MaybeSet<Option<u16>>,
    pub waist_circumference: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub hip_circumference: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub comments: MaybeSet<Option<String>>,
    pub body_fat_pct: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub bia_details: MaybeSet<Option<serde_json::Value>>,
}
