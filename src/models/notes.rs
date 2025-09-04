use chrono::Local;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::MaybeSet;

use super::{UserId, common::MaybeString};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct NoteId(i64);

#[allow(dead_code)]
impl NoteId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for NoteId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for NoteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Note {
    pub id: NoteId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
impl Note {
    pub fn name(&self) -> String {
        self.time.with_timezone(&Local).time().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewNote {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub comments: MaybeString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeNote {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub comments: MaybeSet<MaybeString>,
}
