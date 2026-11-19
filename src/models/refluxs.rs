use jiff::tz::TimeZone;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::MaybeSet;

use super::UserId;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RefluxId(i64);

#[allow(dead_code)]
impl RefluxId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for RefluxId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for RefluxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Reflux {
    pub id: RefluxId,
    pub user_id: UserId,
    pub time: jiff::ZonedDateTime,
    pub duration: Option<jiff::SignedDuration>,
    pub location: Option<String>,
    pub severity: i32,
    pub comments: Option<String>,
    pub complete: bool,
    pub created_at: jiff::Timestamp,
    pub updated_at: jiff::Timestamp,
}

#[allow(dead_code)]
impl Reflux {
    pub fn name(&self) -> String {
        self.time
            .with_time_zone(TimeZone::system())
            .time()
            .to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewReflux {
    pub user_id: UserId,
    pub time: jiff::ZonedDateTime,
    pub duration: Option<jiff::SignedDuration>,
    pub location: Option<String>,
    pub severity: i32,
    pub comments: Option<String>,
    pub complete: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeReflux {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<jiff::ZonedDateTime>,
    pub duration: MaybeSet<Option<jiff::SignedDuration>>,
    pub location: MaybeSet<Option<String>>,
    pub severity: MaybeSet<i32>,
    pub comments: MaybeSet<Option<String>>,
    pub complete: MaybeSet<bool>,
}
