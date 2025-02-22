use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{ConsumptionItem, MaybeF64, UserId, common::MaybeString};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsumptionId(i64);

impl ConsumptionId {
    #[cfg(feature = "server")]
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
    pub liquid_mls: MaybeF64,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
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
    pub liquid_mls: MaybeF64,
    pub comments: MaybeString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateConsumption {
    pub user_id: Option<UserId>,
    pub time: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: Option<chrono::TimeDelta>,
    pub liquid_mls: Option<MaybeF64>,
    pub comments: Option<MaybeString>,
}
