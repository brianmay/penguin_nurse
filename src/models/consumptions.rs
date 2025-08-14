use chrono::Local;
use std::{fmt::Display, str::FromStr};
use tap::Pipe;

use serde::{Deserialize, Serialize};

use crate::forms::{FieldValue, FieldValueError};

use super::{ConsumptionItem, MaybeF64, UserId, common::MaybeString};

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

impl ConsumptionType {
    pub fn options() -> Vec<(&'static str, &'static str)> {
        vec![
            ("digest", "eat/drink"),
            ("inhale_nose", "inhale nose"),
            ("inhale_mouth", "inhale mouth"),
            ("spit_out", "spit out"),
            ("inject", "inject"),
            ("apply_skin", "apply skin"),
        ]
    }
}

impl Display for ConsumptionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Digest => "eat/drink",
            Self::InhaleNose => "inhale nose",
            Self::InhaleMouth => "inhale mouth",
            Self::SpitOut => "spit out",
            Self::Inject => "inject",
            Self::ApplySkin => "apply skin",
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
    pub liquid_mls: MaybeF64,
    pub comments: MaybeString,
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
    pub liquid_mls: MaybeF64,
    pub comments: MaybeString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeConsumption {
    pub user_id: Option<UserId>,
    pub time: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: Option<chrono::TimeDelta>,
    pub consumption_type: Option<ConsumptionType>,
    pub liquid_mls: Option<MaybeF64>,
    pub comments: Option<MaybeString>,
}
