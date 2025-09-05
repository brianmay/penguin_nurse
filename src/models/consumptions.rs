use chrono::Local;
use std::{fmt::Display, str::FromStr};
use tap::Pipe;

use serde::{Deserialize, Serialize};

use crate::{
    forms::{FieldValue, FieldValueError},
    models::{UserId, common::MaybeSet},
};

use super::ConsumptionItem;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConsumptionType {
    Digest,
    InhaleNose,
    InhaleMouth,
    SpitOut,
    Inject,
    ApplySkin,
}

impl FieldValue for ConsumptionType {
    fn as_string(&self) -> String {
        match self {
            Self::Digest => "digest".to_string(),
            Self::InhaleNose => "inhale_nose".to_string(),
            Self::InhaleMouth => "inhale_mouth".to_string(),
            Self::SpitOut => "spit_out".to_string(),
            Self::Inject => "inject".to_string(),
            Self::ApplySkin => "apply_skin".to_string(),
        }
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value {
            "digest" => Ok(Self::Digest),
            "inhale_nose" => Ok(Self::InhaleNose),
            "inhale_mouth" => Ok(Self::InhaleMouth),
            "spit_out" => Ok(Self::SpitOut),
            "inject" => Ok(Self::Inject),
            "apply_skin" => Ok(Self::ApplySkin),
            _ => Err(FieldValueError::InvalidValue),
        }
    }
}

impl Display for ConsumptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digest => "Digest",
            Self::InhaleNose => "Inhale nose",
            Self::InhaleMouth => "Inhale mouth",
            Self::SpitOut => "Spit out",
            Self::Inject => "Inject",
            Self::ApplySkin => "Apply skin",
        }
        .pipe(|s| f.write_str(s))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionId(i64);

impl ConsumptionId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for ConsumptionId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for ConsumptionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Consumption {
    pub id: ConsumptionId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::TimeDelta,
    pub consumption_type: ConsumptionType,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Consumption {
    pub fn name(&self) -> String {
        self.time.with_timezone(&Local).time().to_string()
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConsumptionWithItems {
    pub consumption: Consumption,
    pub items: Vec<ConsumptionItem>,
}

#[cfg(feature = "server")]
impl ConsumptionWithItems {
    pub fn new(consumption: Consumption, items: Vec<ConsumptionItem>) -> Self {
        Self { consumption, items }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewConsumption {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::TimeDelta,
    pub consumption_type: ConsumptionType,
    pub liquid_mls: Option<f64>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeConsumption {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: MaybeSet<chrono::TimeDelta>,
    pub consumption_type: MaybeSet<ConsumptionType>,
    pub liquid_mls: MaybeSet<Option<f64>>,
    pub comments: MaybeSet<Option<String>>,
}
