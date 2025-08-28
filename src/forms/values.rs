use chrono::{DateTime, FixedOffset, Local, TimeDelta, Utc};
use palette::RgbHue;
use thiserror::Error;

use crate::models::Maybe;

#[derive(Error, Debug)]
pub enum FieldValueError {
    #[error("Required value")]
    RequiredValue,

    #[error("Invalid value")]
    InvalidValue,
}

pub trait FieldValue: Sized {
    fn as_string(&self) -> String;
    fn from_string(value: &str) -> Result<Self, FieldValueError>;
}

impl FieldValue for String {
    fn as_string(&self) -> String {
        self.clone()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            Err(FieldValueError::RequiredValue)
        } else {
            Ok(value.to_string())
        }
    }
}

impl FieldValue for RgbHue<f32> {
    fn as_string(&self) -> String {
        self.into_inner().to_string()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
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
    fn as_string(&self) -> String {
        self.with_timezone(&Local).to_rfc3339()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
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
    fn as_string(&self) -> String {
        self.to_rfc3339()
    }

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
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
    fn as_string(&self) -> String {
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

    fn from_string(value: &str) -> Result<Self, FieldValueError> {
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
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
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
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
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
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}

impl<T: FieldValue> FieldValue for Maybe<T> {
    fn as_string(&self) -> String {
        match self {
            Maybe::Some(value) => value.as_string(),
            Maybe::None => "".to_string(),
        }
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            Ok(Maybe::None)
        } else {
            Ok(Maybe::Some(T::from_string(value)?))
        }
    }
}

impl FieldValue for i32 {
    fn as_string(&self) -> String {
        self.to_string()
    }
    fn from_string(value: &str) -> Result<Self, FieldValueError> {
        if value.is_empty() {
            return Err(FieldValueError::RequiredValue);
        }
        match value.parse() {
            Ok(value) => Ok(value),
            Err(_) => Err(FieldValueError::InvalidValue),
        }
    }
}
