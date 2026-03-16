use chrono::Local;
use derive_enum_all_values::AllValues;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::{UserId, common::MaybeSet};

use super::ConsumptionItem;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, AllValues)]
pub enum ConsumptionType {
    Digest,
    InhaleNose,
    InhaleMouth,
    SpitOut,
    Inject,
    ApplySkin,
}

impl ConsumptionType {
    pub fn as_id(&self) -> &'static str {
        match self {
            Self::Digest => "digest",
            Self::InhaleNose => "inhale_nose",
            Self::InhaleMouth => "inhale_mouth",
            Self::SpitOut => "spit_out",
            Self::Inject => "inject",
            Self::ApplySkin => "apply_skin",
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            Self::Digest => "Digest",
            Self::InhaleNose => "Inhale nose",
            Self::InhaleMouth => "Inhale mouth",
            Self::SpitOut => "Spit out",
            Self::Inject => "Inject",
            Self::ApplySkin => "Apply skin",
        }
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
    pub liquid_mls: Option<bigdecimal::BigDecimal>,
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
    pub liquid_mls: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeConsumption {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: MaybeSet<chrono::TimeDelta>,
    pub consumption_type: MaybeSet<ConsumptionType>,
    pub liquid_mls: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub comments: MaybeSet<Option<String>>,
}

const MIN_CONSUMPTION_DURATION_SECONDS: i64 = 2;

pub fn consumption_errors(
    consumption: &Consumption,
    consumption_consumables: Option<&Vec<ConsumptionItem>>,
) -> Vec<String> {
    let mut errors = Vec::new();

    if consumption.duration.num_seconds() < MIN_CONSUMPTION_DURATION_SECONDS {
        errors.push(format!(
            "Duration {} is suspiciously short",
            consumption.duration
        ));
    }

    if let Some(consumption_consumables) = &consumption_consumables {
        let zero = bigdecimal::BigDecimal::from(0);
        let expected_mls = consumption.liquid_mls.as_ref().unwrap_or(&zero);
        let total_nested_mls: bigdecimal::BigDecimal = consumption_consumables
            .iter()
            .filter_map(|ci| ci.nested.liquid_mls.as_ref())
            .cloned()
            .sum();
        if *expected_mls != total_nested_mls {
            errors.push(format!(
                "Liquid ml total from ingredients {}ml does not match consumption liquid ml {}ml",
                total_nested_mls, expected_mls,
            ));
        }
    }

    // check for any nested consumables with consumption type that doesn't match parent
    if let Some(consumption_consumables) = &consumption_consumables {
        for ci in consumption_consumables.iter() {
            if let Some(consumption_type) = ci.consumable.consumption_type
                && consumption_type != consumption.consumption_type
            {
                errors.push(format!(
                    "Ingredient {} has consumption type {} which does not match parent consumption type {}",
                    ci.consumable.name,
                    consumption_type.as_title(),
                    consumption.consumption_type.as_title(),
                ));
            }
        }
    }

    errors
}
