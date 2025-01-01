use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{common::MaybeString, UserId};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Default)]
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

impl FromStr for Bristol {
    type Err = BristolParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Bristol::B0),
            "1" => Ok(Bristol::B1),
            "2" => Ok(Bristol::B2),
            "3" => Ok(Bristol::B3),
            "4" => Ok(Bristol::B4),
            "5" => Ok(Bristol::B5),
            "6" => Ok(Bristol::B6),
            "7" => Ok(Bristol::B7),
            _ => Err(BristolParseError),
        }
    }
}

impl Bristol {
    pub fn as_value(&self) -> i32 {
        (*self).into()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Bristol::B0 => "No poo",
            Bristol::B1 => "Separate hard lumps",
            Bristol::B2 => "Lumpy and sausage-like",
            Bristol::B3 => "Sausage shape with cracks",
            Bristol::B4 => "Smooth and soft",
            Bristol::B5 => "Soft blobs with clear-cut edges",
            Bristol::B6 => "Mushy",
            Bristol::B7 => "Watery",
        }
    }

    pub fn options() -> Vec<(&'static str, &'static str)> {
        vec![
            ("0", "0. No poo"),
            ("1", "1. Separate hard lumps"),
            ("2", "2. Lumpy and sausage-like"),
            ("3", "3. Sausage shape with cracks"),
            ("4", "4. Smooth and soft"),
            ("5", "5. Soft blobs with clear-cut edges"),
            ("6", "6. Mushy"),
            ("7", "7. Watery"),
        ]
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Poo {
    pub id: PooId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub quantity: i32,
    pub bristol: Bristol,
    pub colour: palette::Hsv,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewPoo {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::Utc>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub quantity: i32,
    pub bristol: Bristol,
    pub colour: palette::Hsv,
    pub comments: MaybeString,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdatePoo {
    pub user_id: Option<UserId>,
    pub time: Option<chrono::DateTime<chrono::Utc>>,
    pub duration: Option<chrono::Duration>,
    pub urgency: Option<i32>,
    pub quantity: Option<i32>,
    pub bristol: Option<Bristol>,
    pub colour: Option<palette::Hsv>,
    pub comments: Option<MaybeString>,
}
