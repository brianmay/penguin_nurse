use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::forms::{FieldValue, FieldValueError};

use super::common::{MaybeDateTime, MaybeString};

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
    pub fn options() -> Vec<(&'static str, &'static str)> {
        vec![
            ("millilitres", "Mls"),
            ("grams", "G"),
            ("international_units", "IU"),
            ("number", "Number"),
        ]
    }
}

impl Display for ConsumableUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Millilitres => write!(f, "Mls"),
            Self::Grams => write!(f, "G"),
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

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Consumable {
    pub id: ConsumableId,
    pub name: String,
    pub brand: MaybeString,
    pub barcode: MaybeString,
    pub is_organic: bool,
    pub unit: ConsumableUnit,
    pub comments: MaybeString,
    pub created: MaybeDateTime,
    pub destroyed: MaybeDateTime,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewConsumable {
    pub name: String,
    pub brand: MaybeString,
    pub barcode: MaybeString,
    pub is_organic: bool,
    pub unit: ConsumableUnit,
    pub comments: MaybeString,
    pub created: MaybeDateTime,
    pub destroyed: MaybeDateTime,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateConsumable {
    pub name: Option<String>,
    pub brand: Option<MaybeString>,
    pub barcode: Option<MaybeString>,
    pub is_organic: Option<bool>,
    pub unit: Option<ConsumableUnit>,
    pub comments: Option<MaybeString>,
    pub created: Option<MaybeDateTime>,
    pub destroyed: Option<MaybeDateTime>,
}
