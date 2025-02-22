use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{UserId, common::MaybeString};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WeeId(i64);

impl WeeId {
    #[cfg(feature = "server")]
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for WeeId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for WeeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Wee {
    pub id: WeeId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub mls: i32,
    pub colour: palette::Hsv,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewWee {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::Duration,
    pub urgency: i32,
    pub mls: i32,
    pub colour: palette::Hsv,
    pub comments: MaybeString,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct UpdateWee {
    pub user_id: Option<UserId>,
    pub time: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: Option<chrono::Duration>,
    pub urgency: Option<i32>,
    pub mls: Option<i32>,
    pub colour: Option<palette::Hsv>,
    pub comments: Option<MaybeString>,
}
