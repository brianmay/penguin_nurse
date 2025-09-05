use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    forms::{FieldValue, FieldValueError},
    models::MaybeSet,
};

use super::ConsumableItem;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ConsumableUnit {
    Millilitres,
    Grams,
    InternationalUnits,
    Number,
}

impl FieldValue for ConsumableUnit {
    fn as_string(&self) -> String {
        match self {
            Self::Millilitres => "millilitres".to_string(),
            Self::Grams => "grams".to_string(),
            Self::InternationalUnits => "international_units".to_string(),
            Self::Number => "number".to_string(),
        }
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value {
            "millilitres" => Ok(Self::Millilitres),
            "grams" => Ok(Self::Grams),
            "international_units" => Ok(Self::InternationalUnits),
            "number" => Ok(Self::Number),
            _ => Err(FieldValueError::InvalidValue),
        }
    }
}

impl ConsumableUnit {
    pub fn postfix(&self) -> &'static str {
        match self {
            Self::Millilitres => "ml",
            Self::Grams => "g",
            Self::InternationalUnits => "IU",
            Self::Number => "",
        }
    }
}

impl Display for ConsumableUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Millilitres => write!(f, "ml"),
            Self::Grams => write!(f, "g"),
            Self::InternationalUnits => write!(f, "IU"),
            Self::Number => write!(f, "Number"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ConsumableId(i64);

impl ConsumableId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    #[cfg(feature = "server")]
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for ConsumableId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for ConsumableId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Consumable {
    pub id: ConsumableId,
    pub name: String,
    pub brand: Option<String>,
    pub barcode: Option<String>,
    pub is_organic: bool,
    pub unit: ConsumableUnit,
    pub comments: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub destroyed: Option<DateTime<Utc>>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[cfg(feature = "server")]
impl ConsumableWithItems {
    pub fn new(consumable: Consumable, items: Vec<ConsumableItem>) -> Self {
        Self { consumable, items }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConsumableWithItems {
    pub consumable: Consumable,
    pub items: Vec<ConsumableItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewConsumable {
    pub name: String,
    pub brand: Option<String>,
    pub barcode: Option<String>,
    pub is_organic: bool,
    pub unit: ConsumableUnit,
    pub comments: Option<String>,
    pub created: Option<DateTime<Utc>>,
    pub destroyed: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeConsumable {
    pub name: MaybeSet<String>,
    pub brand: MaybeSet<Option<String>>,
    pub barcode: MaybeSet<Option<String>>,
    pub is_organic: MaybeSet<bool>,
    pub unit: MaybeSet<ConsumableUnit>,
    pub comments: MaybeSet<Option<String>>,
    pub created: MaybeSet<Option<DateTime<Utc>>>,
    pub destroyed: MaybeSet<Option<DateTime<Utc>>>,
}
