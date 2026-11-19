use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::{MaybeSet, Urgency};

use super::UserId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct WeeUrgeId(i64);

#[allow(dead_code)]
impl WeeUrgeId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for WeeUrgeId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for WeeUrgeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeeUrge {
    pub id: WeeUrgeId,
    pub user_id: UserId,
    pub time: jiff::ZonedDateTime,
    pub urgency: Urgency,
    pub comments: Option<String>,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewWeeUrge {
    pub user_id: UserId,
    pub time: jiff::ZonedDateTime,
    pub urgency: Urgency,
    pub comments: Option<String>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeWeeUrge {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<jiff::ZonedDateTime>,
    pub urgency: MaybeSet<Urgency>,
    pub comments: MaybeSet<Option<String>>,
}
