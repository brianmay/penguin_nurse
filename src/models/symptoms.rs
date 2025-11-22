use chrono::Local;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::MaybeSet;

use super::UserId;

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
    pub nasal_symptom: i32,
    pub nasal_symptom_description: Option<String>,
    pub sneezing: i32,
    pub heart_burn: i32,
    pub abdominal_pain: i32,
    pub abdominal_pain_location: Option<String>,
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
    pub shoulder_pain: i32,
    pub hand_pain: i32,
    pub foot_pain: i32,
    pub wrist_pain: i32,
    pub dental_pain: i32,
    pub dental_pain_description: Option<String>,
    pub eye_pain: i32,
    pub ear_pain: i32,
    pub feeling_hot: i32,
    pub feeling_cold: i32,
    pub feeling_thirsty: i32,
    pub comments: Option<String>,
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
    pub nasal_symptom: i32,
    pub nasal_symptom_description: Option<String>,
    pub sneezing: i32,
    pub heart_burn: i32,
    pub abdominal_pain: i32,
    pub abdominal_pain_location: Option<String>,
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
    pub shoulder_pain: i32,
    pub hand_pain: i32,
    pub foot_pain: i32,
    pub wrist_pain: i32,
    pub dental_pain: i32,
    pub dental_pain_description: Option<String>,
    pub eye_pain: i32,
    pub ear_pain: i32,
    pub feeling_hot: i32,
    pub feeling_cold: i32,
    pub feeling_thirsty: i32,
    pub comments: Option<String>,
}

impl NewSymptom {
    pub fn default(user_id: UserId, time: chrono::DateTime<chrono::FixedOffset>) -> Self {
        Self {
            user_id,
            time,
            appetite_loss: 0,
            fever: 0,
            cough: 0,
            sore_throat: 0,
            nasal_symptom: 0,
            nasal_symptom_description: None,
            sneezing: 0,
            heart_burn: 0,
            abdominal_pain: 0,
            abdominal_pain_location: None,
            diarrhea: 0,
            constipation: 0,
            lower_back_pain: 0,
            upper_back_pain: 0,
            neck_pain: 0,
            joint_pain: 0,
            headache: 0,
            nausea: 0,
            dizziness: 0,
            stomach_ache: 0,
            chest_pain: 0,
            shortness_of_breath: 0,
            fatigue: 0,
            anxiety: 0,
            depression: 0,
            insomnia: 0,
            shoulder_pain: 0,
            hand_pain: 0,
            foot_pain: 0,
            wrist_pain: 0,
            dental_pain: 0,
            dental_pain_description: None,
            eye_pain: 0,
            ear_pain: 0,
            feeling_hot: 0,
            feeling_cold: 0,
            feeling_thirsty: 0,
            comments: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct ChangeSymptom {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub appetite_loss: MaybeSet<i32>,
    pub fever: MaybeSet<i32>,
    pub cough: MaybeSet<i32>,
    pub sore_throat: MaybeSet<i32>,
    pub nasal_symptom: MaybeSet<i32>,
    pub nasal_symptom_description: MaybeSet<Option<String>>,
    pub sneezing: MaybeSet<i32>,
    pub heart_burn: MaybeSet<i32>,
    pub abdominal_pain: MaybeSet<i32>,
    pub abdominal_pain_location: MaybeSet<Option<String>>,
    pub diarrhea: MaybeSet<i32>,
    pub constipation: MaybeSet<i32>,
    pub lower_back_pain: MaybeSet<i32>,
    pub upper_back_pain: MaybeSet<i32>,
    pub neck_pain: MaybeSet<i32>,
    pub joint_pain: MaybeSet<i32>,
    pub headache: MaybeSet<i32>,
    pub nausea: MaybeSet<i32>,
    pub dizziness: MaybeSet<i32>,
    pub stomach_ache: MaybeSet<i32>,
    pub chest_pain: MaybeSet<i32>,
    pub shortness_of_breath: MaybeSet<i32>,
    pub fatigue: MaybeSet<i32>,
    pub anxiety: MaybeSet<i32>,
    pub depression: MaybeSet<i32>,
    pub insomnia: MaybeSet<i32>,
    pub shoulder_pain: MaybeSet<i32>,
    pub hand_pain: MaybeSet<i32>,
    pub foot_pain: MaybeSet<i32>,
    pub wrist_pain: MaybeSet<i32>,
    pub dental_pain: MaybeSet<i32>,
    pub dental_pain_description: MaybeSet<Option<String>>,
    pub eye_pain: MaybeSet<i32>,
    pub ear_pain: MaybeSet<i32>,
    pub feeling_hot: MaybeSet<i32>,
    pub feeling_cold: MaybeSet<i32>,
    pub feeling_thirsty: MaybeSet<i32>,
    pub comments: MaybeSet<Option<String>>,
}
