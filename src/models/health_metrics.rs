use chrono::Local;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::common::MaybeI32;

use super::{
    UserId,
    common::{MaybeDecimal, MaybeString},
};

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
    pub pulse: MaybeI32,
    pub blood_glucose: MaybeDecimal,
    pub systolic_bp: MaybeI32,
    pub diastolic_bp: MaybeI32,
    pub weight: MaybeDecimal,
    pub height: MaybeI32,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    pub pulse: MaybeI32,
    pub blood_glucose: MaybeDecimal,
    pub systolic_bp: MaybeI32,
    pub diastolic_bp: MaybeI32,
    pub weight: MaybeDecimal,
    pub height: MaybeI32,
    pub comments: MaybeString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeHealthMetric {
    pub user_id: Option<UserId>,
    pub time: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub pulse: Option<MaybeI32>,
    pub blood_glucose: Option<MaybeDecimal>,
    pub systolic_bp: Option<MaybeI32>,
    pub diastolic_bp: Option<MaybeI32>,
    pub weight: Option<MaybeDecimal>,
    pub height: Option<MaybeI32>,
    pub comments: Option<MaybeString>,
}
