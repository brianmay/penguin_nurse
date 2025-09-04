use chrono::Local;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::{UserId, common::MaybeString};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SymptomId(i64);

#[allow(dead_code)]
impl SymptomId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for SymptomId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for SymptomId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Symptom {
    pub id: SymptomId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub appetite_loss: i32,
    pub fever: i32,
    pub cough: i32,
    pub sore_throat: i32,
    pub runny_nose: i32,
    pub sneezing: i32,
    pub heart_burn: i32,
    pub abdominal_pain: i32,
    pub abdominal_pain_location: MaybeString,
    pub diarrhea: i32,
    pub constipation: i32,
    pub lower_back_pain: i32,
    pub upper_back_pain: i32,
    pub neck_pain: i32,
    pub joint_pain: i32,
    pub headache: i32,
    pub nausea: i32,
    pub dizziness: i32,
    pub stomach_ache: i32,
    pub chest_pain: i32,
    pub shortness_of_breath: i32,
    pub fatigue: i32,
    pub anxiety: i32,
    pub depression: i32,
    pub insomnia: i32,
    pub comments: MaybeString,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
impl Symptom {
    pub fn name(&self) -> String {
        self.time.with_timezone(&Local).time().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewSymptom {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub appetite_loss: i32,
    pub fever: i32,
    pub cough: i32,
    pub sore_throat: i32,
    pub runny_nose: i32,
    pub sneezing: i32,
    pub heart_burn: i32,
    pub abdominal_pain: i32,
    pub abdominal_pain_location: MaybeString,
    pub diarrhea: i32,
    pub constipation: i32,
    pub lower_back_pain: i32,
    pub upper_back_pain: i32,
    pub neck_pain: i32,
    pub joint_pain: i32,
    pub headache: i32,
    pub nausea: i32,
    pub dizziness: i32,
    pub stomach_ache: i32,
    pub chest_pain: i32,
    pub shortness_of_breath: i32,
    pub fatigue: i32,
    pub anxiety: i32,
    pub depression: i32,
    pub insomnia: i32,
    pub comments: MaybeString,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeSymptom {
    pub user_id: Option<UserId>,
    pub time: Option<chrono::DateTime<chrono::FixedOffset>>,
    pub appetite_loss: Option<i32>,
    pub fever: Option<i32>,
    pub cough: Option<i32>,
    pub sore_throat: Option<i32>,
    pub runny_nose: Option<i32>,
    pub sneezing: Option<i32>,
    pub heart_burn: Option<i32>,
    pub abdominal_pain: Option<i32>,
    pub abdominal_pain_location: Option<MaybeString>,
    pub diarrhea: Option<i32>,
    pub constipation: Option<i32>,
    pub lower_back_pain: Option<i32>,
    pub upper_back_pain: Option<i32>,
    pub neck_pain: Option<i32>,
    pub joint_pain: Option<i32>,
    pub headache: Option<i32>,
    pub nausea: Option<i32>,
    pub dizziness: Option<i32>,
    pub stomach_ache: Option<i32>,
    pub chest_pain: Option<i32>,
    pub shortness_of_breath: Option<i32>,
    pub fatigue: Option<i32>,
    pub anxiety: Option<i32>,
    pub depression: Option<i32>,
    pub insomnia: Option<i32>,
    pub comments: Option<MaybeString>,
}
