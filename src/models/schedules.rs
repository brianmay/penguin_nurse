use std::str::FromStr;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};

use crate::models::{ConsumptionType, UserId, common::MaybeSet};

use super::ScheduleItem;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScheduleId(i64);

impl ScheduleId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for ScheduleId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for ScheduleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Schedule {
    pub id: ScheduleId,
    pub user_id: UserId,
    pub time_of_day: NaiveTime,
    pub consumption_type: ConsumptionType,
    pub comments: Option<String>,
    pub active: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Schedule {
    pub fn name(&self) -> String {
        self.time_of_day.format("%H:%M").to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduleWithItems {
    pub schedule: Schedule,
    pub items: Vec<ScheduleItem>,
}

#[cfg(feature = "server")]
impl ScheduleWithItems {
    pub fn new(schedule: Schedule, items: Vec<ScheduleItem>) -> Self {
        Self { schedule, items }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewSchedule {
    pub user_id: UserId,
    pub time_of_day: NaiveTime,
    pub consumption_type: ConsumptionType,
    pub comments: Option<String>,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeSchedule {
    pub user_id: MaybeSet<UserId>,
    pub time_of_day: MaybeSet<NaiveTime>,
    pub consumption_type: MaybeSet<ConsumptionType>,
    pub comments: MaybeSet<Option<String>>,
    pub active: MaybeSet<bool>,
}

/// Whether the scheduled time-of-day has arrived for the current day.
#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ScheduleDue {
    /// The scheduled time-of-day has passed.
    Due,

    /// The scheduled time-of-day is later today.
    Upcoming,
}

/// A scheduled ingredient, with whether it has already been recorded in a
/// consumption today.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduleItemStatus {
    pub item: ScheduleItem,
    pub recorded: bool,
}

/// A schedule, its occurrence status for the current day, and the recorded
/// status of each of its items.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ScheduleStatus {
    pub schedule: Schedule,
    pub due: ScheduleDue,
    pub items: Vec<ScheduleItemStatus>,
}
