use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use dioxus::prelude::*;
use palette::RgbHue;
use thiserror::Error;

use crate::components::consumables::{ConsumableIcon, ConsumableLabel, ConsumableUnitIcon};
use crate::components::consumptions::ConsumptionTypeIcon;
use crate::components::events::{UrgencyIcon, UrgencyLabel};
use crate::components::exercises::{ExerciseRpeIcon, ExerciseRpeLabel, ExerciseTypeIcon};
use crate::components::poos::PooBristolIcon;
use crate::components::{ElementIcon, StrIcon};
use crate::models::{
    Bristol, Consumable, ConsumableUnit, ConsumptionType, ExerciseRpe, ExerciseType, Urgency,
};

#[derive(Error, Debug)]
pub enum FieldValueError {
    #[error("Required value")]
    RequiredValue,

    #[error("Invalid value")]
    InvalidValue,
}

pub trait FieldValue: Sized {
    type RawValue;
    type DerefValue: ?Sized;
    fn as_raw(&self) -> Self::RawValue;
    fn from_raw(value: &Self::DerefValue) -> Result<Self, FieldValueError>;
}

pub trait FieldLabel {
    fn as_label(&self) -> Element;
}

impl FieldValue for String {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.clone()
    }

    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            Err(FieldValueError::RequiredValue)
        } else {
            Ok(value.to_string())
        }
    }
}

impl FieldValue for RgbHue<f32> {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.into_inner().to_string()
    }

    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) if (0.0..=360.0).contains(&value) => Ok(RgbHue::new(value)),
            _ => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for DateTime<Utc> {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.with_timezone(&Local).to_rfc3339()
    }

    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match DateTime::parse_from_rfc3339(value) {
            Ok(time) => Ok(time.with_timezone(&Utc)),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for DateTime<FixedOffset> {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.to_rfc3339()
    }

    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match DateTime::parse_from_rfc3339(value) {
            Ok(time) => Ok(time),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for TimeDelta {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        let (negative, total_seconds) = {
            let seconds = self.num_seconds();
            if seconds < 0 {
                (true, -seconds)
            } else {
                (false, seconds)
            }
        };
        let sign = if negative { "-" } else { "" };
        let seconds = total_seconds % 60;
        let minutes = (total_seconds / 60) % 60;
        let hours = (total_seconds / 60) / 60;
        format!("{sign}{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
    }

    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        let (negative, value) = if let Some(stripped) = value.strip_prefix('-') {
            (-1, stripped)
        } else {
            (1, value)
        };
        let split = value.split([':', '.']).collect::<Vec<&str>>();

        let (hours, minutes, seconds) = match split[..] {
            [a] => {
                let minutes = a
                    .parse::<i64>()
                    .map_err(|_| FieldValueError::InvalidValue)?;
                (0, minutes, 0)
            }

            [a, b] => {
                let minutes = a
                    .parse::<i64>()
                    .map_err(|_| FieldValueError::InvalidValue)?;

                let seconds = b
                    .parse::<i64>()
                    .map_err(|_| FieldValueError::InvalidValue)?;

                (0, minutes, seconds)
            }

            [a, b, c] => {
                let hours = a
                    .parse::<i64>()
                    .map_err(|_| FieldValueError::InvalidValue)?;

                let minutes = b
                    .parse::<i64>()
                    .map_err(|_| FieldValueError::InvalidValue)?;

                let seconds = c
                    .parse::<i64>()
                    .map_err(|_| FieldValueError::InvalidValue)?;

                (hours, minutes, seconds)
            }

            _ => {
                return Err(FieldValueError::InvalidValue);
            }
        };

        if hours < 0 || minutes < 0 || seconds < 0 {
            return Err(FieldValueError::InvalidValue);
        }
        if hours > 23 || minutes > 59 || seconds > 59 {
            return Err(FieldValueError::InvalidValue);
        }

        Ok(
            (TimeDelta::hours(hours) + TimeDelta::minutes(minutes) + TimeDelta::seconds(seconds))
                * negative,
        )
    }
}

impl FieldValue for f32 {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.to_string()
    }
    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for f64 {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.to_string()
    }
    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldValue for bigdecimal::BigDecimal {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.to_string()
    }
    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl<T: FieldValue<RawValue = String, DerefValue = str>> FieldValue for Option<T> {
    type RawValue = T::RawValue;
    type DerefValue = T::DerefValue;

    fn as_raw(&self) -> Self::RawValue {
        match self {
            Some(value) => value.as_raw(),
            None => "".to_string(),
        }
    }
    fn from_raw(value: &Self::DerefValue) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            Ok(None)
        } else {
            Ok(Some(T::from_raw(value)?))
        }
    }
}

impl FieldValue for i32 {
    type RawValue = String;
    type DerefValue = str;

    fn as_raw(&self) -> String {
        self.to_string()
    }
    fn from_raw(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl FieldLabel for ConsumptionType {
    fn as_label(&self) -> Element {
        let label = self.as_title();
        rsx! {
            StrIcon {
                title: label,
                icon: rsx! {
                    ConsumptionTypeIcon { consumption_type: *self }
                },
            }
        }
    }
}

impl FieldLabel for ConsumableUnit {
    fn as_label(&self) -> Element {
        let label = self.as_title();
        rsx! {
            StrIcon {
                title: label,
                icon: rsx! {
                    ConsumableUnitIcon { consumable_unit: *self }
                },
            }
        }
    }
}

impl FieldLabel for ExerciseType {
    fn as_label(&self) -> Element {
        let label = self.as_title();
        rsx! {
            StrIcon {
                title: label,
                icon: rsx! {
                    ExerciseTypeIcon { exercise_type: *self }
                },
            }
        }
    }
}

impl FieldLabel for ExerciseRpe {
    fn as_label(&self) -> Element {
        rsx! {
            ElementIcon {
                title: rsx! {
                    ExerciseRpeLabel { rpe: *self }
                },
                icon: rsx! {
                    ExerciseRpeIcon { rpe: *self }
                },
            }
        }
    }
}

impl FieldLabel for Urgency {
    fn as_label(&self) -> Element {
        rsx! {
            ElementIcon {
                title: rsx! {
                    UrgencyLabel { urgency: *self }
                },
                icon: rsx! {
                    UrgencyIcon { urgency: *self }
                },
            }
        }
    }
}

impl FieldLabel for Bristol {
    fn as_label(&self) -> Element {
        let label = self.as_title();
        rsx! {
            StrIcon {
                title: label,
                icon: rsx! {
                    PooBristolIcon { bristol: *self }
                },
            }
        }
    }
}

impl FieldLabel for Consumable {
    fn as_label(&self) -> Element {
        rsx! {
            ElementIcon {
                title: rsx! {
                    ConsumableLabel { consumable: self.clone() }
                },
                icon: rsx! {
                    ConsumableIcon {}
                },
            }
        }
    }
}
