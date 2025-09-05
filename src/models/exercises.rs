use chrono::Local;
use std::{fmt::Display, str::FromStr};
use tap::Pipe;

use serde::{Deserialize, Serialize};

use crate::{
    forms::{FieldValue, FieldValueError},
    models::common::MaybeSet,
};

use super::UserId;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub enum ExerciseType {
    Walking,
    Running,
    Cycling,
    IndoorCycling,
    Jumping,
    Skipping,
    Flying,
    Other,
}

impl FieldValue for ExerciseType {
    fn as_string(&self) -> String {
        match self {
            Self::Walking => "walking".to_string(),
            Self::Running => "running".to_string(),
            Self::Cycling => "cycling".to_string(),
            Self::IndoorCycling => "indoor_cycling".to_string(),
            Self::Jumping => "jumping".to_string(),
            Self::Skipping => "skipping".to_string(),
            Self::Flying => "flying".to_string(),
            Self::Other => "other".to_string(),
        }
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        match value {
            "walking" => Ok(Self::Walking),
            "running" => Ok(Self::Running),
            "cycling" => Ok(Self::Cycling),
            "indoor_cycling" => Ok(Self::IndoorCycling),
            "jumping" => Ok(Self::Jumping),
            "skipping" => Ok(Self::Skipping),
            "flying" => Ok(Self::Flying),
            "other" => Ok(Self::Other),
            _ => Err(FieldValueError::InvalidValue),
        }
    }
}

impl Display for ExerciseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Walking => "walking",
            Self::Running => "running",
            Self::Cycling => "cycling",
            Self::IndoorCycling => "indoor cycling",
            Self::Jumping => "jumping",
            Self::Skipping => "skipping",
            Self::Flying => "flying",
            Self::Other => "other",
        }
        .pipe(|s| f.write_str(s))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ExerciseId(i64);

#[allow(dead_code)]
impl ExerciseId {
    pub fn new(id: i64) -> Self {
        Self(id)
    }
    pub fn as_inner(self) -> i64 {
        self.0
    }
}

impl FromStr for ExerciseId {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl std::fmt::Display for ExerciseId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Exercise {
    pub id: ExerciseId,
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::TimeDelta,
    pub location: Option<String>,
    pub distance: Option<bigdecimal::BigDecimal>,
    pub calories: Option<i32>,
    pub rpe: Option<i32>,
    pub exercise_type: ExerciseType,
    pub comments: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[allow(dead_code)]
impl Exercise {
    pub fn name(&self) -> String {
        self.time.with_timezone(&Local).time().to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewExercise {
    pub user_id: UserId,
    pub time: chrono::DateTime<chrono::FixedOffset>,
    pub duration: chrono::TimeDelta,
    pub location: Option<String>,
    pub distance: Option<bigdecimal::BigDecimal>,
    pub calories: Option<i32>,
    pub rpe: Option<i32>,
    pub exercise_type: ExerciseType,
    pub comments: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChangeExercise {
    pub user_id: MaybeSet<UserId>,
    pub time: MaybeSet<chrono::DateTime<chrono::FixedOffset>>,
    pub duration: MaybeSet<chrono::TimeDelta>,
    pub location: MaybeSet<Option<String>>,
    pub distance: MaybeSet<Option<bigdecimal::BigDecimal>>,
    pub calories: MaybeSet<Option<i32>>,
    pub rpe: MaybeSet<Option<i32>>,
    pub exercise_type: MaybeSet<ExerciseType>,
    pub comments: MaybeSet<Option<String>>,
}
