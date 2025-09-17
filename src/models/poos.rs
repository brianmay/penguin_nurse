use std::str::FromStr;

use derive_enum_all_values::AllValues;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::{MaybeSet, common::Urgency};

use super::UserId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PooId(i64);

impl PooId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for PooId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for PooId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Default, AllValues)]
#[serde(tag = "type")]
pub enum Bristol {
    #[default]
    B0,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
}

#[derive(Error, Debug)]
#[error("Failed to parse Bristol")]
pub struct BristolParseError;

impl TryFrom<i32> for Bristol {
    type Error = BristolParseError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bristol::B0),
            1 => Ok(Bristol::B1),
            2 => Ok(Bristol::B2),
            3 => Ok(Bristol::B3),
            4 => Ok(Bristol::B4),
            5 => Ok(Bristol::B5),
            6 => Ok(Bristol::B6),
            7 => Ok(Bristol::B7),
            _ => Err(BristolParseError),
        }
    }
}

impl From<Bristol> for i32 {
    fn from(value: Bristol) -> Self {
        match value {
            Bristol::B0 => 0,
            Bristol::B1 => 1,
            Bristol::B2 => 2,
            Bristol::B3 => 3,
            Bristol::B4 => 4,
            Bristol::B5 => 5,
            Bristol::B6 => 6,
            Bristol::B7 => 7,
        }
    }
}

impl Bristol {
    pub fn as_id(&self) -> &'static str {
        match self {
            Bristol::B0 => "0",
            Bristol::B1 => "1",
            Bristol::B2 => "2",
            Bristol::B3 => "3",
            Bristol::B4 => "4",
            Bristol::B5 => "5",
            Bristol::B6 => "6",
            Bristol::B7 => "7",
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            Bristol::B0 => "No poo. Looks like a ghost.",
            Bristol::B1 => "Separate hard lumps. Looks like rabbit droppings.",
            Bristol::B2 => "Lumpy and sausage-like. Looks like a bunch of grapes.",
            Bristol::B3 => "Sausage shape with cracks. Looks like corn on the cob.",
            Bristol::B4 => "Smooth and soft. Looks like a well-cooked sausage.",
            Bristol::B5 => "Soft blobs with clear-cut edges. Looks like a chicken nuggets.",
            Bristol::B6 => "Mushy. Looks like a porridge.",
            Bristol::B7 => "Watery. Looks like a gravy.",
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Poo {
    pub id: PooId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::Duration,
    pub urgency: Urgency,
    pub quantity: i32,
    pub bristol: Bristol,
    pub colour: palette::Hsv,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewPoo {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::Duration,
    pub urgency: Urgency,
    pub quantity: i32,
    pub bristol: Bristol,
    pub colour: palette::Hsv,
    pub comments: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangePoo {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: MaybeSet<chrono::Duration>,
    pub urgency: MaybeSet<Urgency>,
    pub quantity: MaybeSet<i32>,
    pub bristol: MaybeSet<Bristol>,
    pub colour: MaybeSet<palette::Hsv>,
    pub comments: MaybeSet<Option<String>>,
}
