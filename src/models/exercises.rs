use chrono::Local;
use derive_enum_all_values::AllValues;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::models::common::MaybeSet;

use super::UserId;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, AllValues)]
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

impl ExerciseType {
    pub fn as_id(&self) -> &'static str {
        match self {
            Self::Walking => "walking",
            Self::Running => "running",
            Self::Cycling => "cycling",
            Self::IndoorCycling => "indoor_cycling",
            Self::Jumping => "jumping",
            Self::Skipping => "skipping",
            Self::Flying => "flying",
            Self::Other => "other",
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            Self::Walking => "Walking",
            Self::Running => "Running",
            Self::Cycling => "Cycling",
            Self::IndoorCycling => "Indoor Cycling",
            Self::Jumping => "Jumping",
            Self::Skipping => "Skipping",
            Self::Flying => "Flying",
            Self::Other => "Other",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, AllValues)]
pub enum ExerciseRpe {
    Rpe1,
    Rpe2,
    Rpe3,
    Rpe4,
    Rpe5,
    Rpe6,
    Rpe7,
    Rpe8,
    Rpe9,
    Rpe10,
}

impl ExerciseRpe {
    pub fn as_id(&self) -> &'static str {
        match self {
            Self::Rpe1 => "1",
            Self::Rpe2 => "2",
            Self::Rpe3 => "3",
            Self::Rpe4 => "4",
            Self::Rpe5 => "5",
            Self::Rpe6 => "6",
            Self::Rpe7 => "7",
            Self::Rpe8 => "8",
            Self::Rpe9 => "9",
            Self::Rpe10 => "10",
        }
    }

    pub fn as_title(&self) -> &'static str {
        match self {
            Self::Rpe1 => "Very light",
            Self::Rpe2 => "Light",
            Self::Rpe3 => "Moderate",
            Self::Rpe4 => "Somewhat hard",
            Self::Rpe5 => "Hard",
            Self::Rpe6 => "Harder",
            Self::Rpe7 => "Very hard",
            Self::Rpe8 => "Very, very hard",
            Self::Rpe9 => "Extremely hard",
            Self::Rpe10 => "Maximal effort",
        }
    }
}

impl TryFrom<i32> for ExerciseRpe {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ExerciseRpe::Rpe1),
            2 => Ok(ExerciseRpe::Rpe2),
            3 => Ok(ExerciseRpe::Rpe3),
            4 => Ok(ExerciseRpe::Rpe4),
            5 => Ok(ExerciseRpe::Rpe5),
            6 => Ok(ExerciseRpe::Rpe6),
            7 => Ok(ExerciseRpe::Rpe7),
            8 => Ok(ExerciseRpe::Rpe8),
            9 => Ok(ExerciseRpe::Rpe9),
            10 => Ok(ExerciseRpe::Rpe10),
            _ => Err(()),
        }
    }
}

impl From<ExerciseRpe> for i32 {
    fn from(value: ExerciseRpe) -> Self {
        match value {
            ExerciseRpe::Rpe1 => 1,
            ExerciseRpe::Rpe2 => 2,
            ExerciseRpe::Rpe3 => 3,
            ExerciseRpe::Rpe4 => 4,
            ExerciseRpe::Rpe5 => 5,
            ExerciseRpe::Rpe6 => 6,
            ExerciseRpe::Rpe7 => 7,
            ExerciseRpe::Rpe8 => 8,
            ExerciseRpe::Rpe9 => 9,
            ExerciseRpe::Rpe10 => 10,
        }
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
    pub rpe: Option<ExerciseRpe>,
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
    pub rpe: Option<ExerciseRpe>,
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
    pub rpe: MaybeSet<Option<ExerciseRpe>>,
    pub exercise_type: MaybeSet<ExerciseType>,
    pub comments: MaybeSet<Option<String>>,
}
